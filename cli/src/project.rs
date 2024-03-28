use core::fmt;
use std::path::{Path, PathBuf};

use log::warn;
use serde::{Deserialize, Serialize};
use serde_json::json;
use typst_ts_compiler::service::Compiler;

use crate::{
    error::prelude::*,
    meta::{BookMeta, BookMetaContent, BookMetaElem, BuildMeta},
    render::{DataDict, HtmlRenderer, TypstRenderer},
    utils::{copy_dir_embedded, create_dirs, release_packages, write_file},
    CompileArgs,
};
use include_dir::include_dir;

/// Typst content kind embedded in metadata nodes
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "func")]
enum JsonContent {
    #[serde(rename = "sequence")]
    Sequence { children: Vec<JsonContent> },
    #[serde(rename = "space")]
    Space {},
    #[serde(rename = "text")]
    Text { text: String },
}

impl fmt::Display for JsonContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sequence { children } => {
                for ch in children {
                    ch.fmt(f)?
                }
            }
            Self::Space {} => f.write_str(" ")?,
            Self::Text { text } => f.write_str(text)?,
        }

        Ok(())
    }
}

pub struct Project {
    pub tr: TypstRenderer,
    pub hr: HtmlRenderer,
    pub book_meta: Option<BookMeta>,
    pub build_meta: Option<BuildMeta>,

    pub dest_dir: PathBuf,
    pub path_to_root: String,
}

impl Project {
    pub fn new(mut args: CompileArgs) -> ZResult<Self> {
        let mut final_dest_dir = args.dest_dir.clone();
        let path_to_root = args.path_to_root.clone();

        if !path_to_root.starts_with('/') {
            args.path_to_root = "/".to_owned() + &args.path_to_root;
        }

        if !path_to_root.ends_with('/') {
            args.path_to_root.push('/');
        }

        if args.workspace.is_empty() {
            args.workspace = args.dir.clone();
        }

        let tr = TypstRenderer::new(args);
        let hr = HtmlRenderer::new();

        let mut proj = Self {
            dest_dir: tr.dest_dir.clone(),
            tr,
            hr,
            book_meta: None,
            build_meta: None,
            path_to_root,
        };

        release_packages(
            proj.tr.compiler.world_mut(),
            include_dir!("$CARGO_MANIFEST_DIR/../contrib/typst/book"),
        );

        release_packages(
            proj.tr.compiler.world_mut(),
            include_dir!("$CARGO_MANIFEST_DIR/../contrib/typst/variables"),
        );

        proj.compile_meta()?;

        if final_dest_dir.is_empty() {
            if let Some(dest_dir) = proj.build_meta.as_ref().map(|b| b.dest_dir.clone()) {
                final_dest_dir = dest_dir;
            }
        }

        if final_dest_dir.is_empty() {
            final_dest_dir = "dist".to_owned();
        }

        proj.tr.fix_dest_dir(Path::new(&final_dest_dir));
        proj.dest_dir = proj.tr.dest_dir.clone();

        Ok(proj)
    }

    pub fn compile_meta(&mut self) -> ZResult<()> {
        #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
        struct QueryItem<T> {
            pub value: T,
        }

        type Json<T> = Vec<QueryItem<T>>;

        let doc = self.tr.compile_book(Path::new("book.typ"))?;

        #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
        pub enum InternalPackageMeta {
            /// The version of the package used by users
            #[serde(rename = "package")]
            Package { version: String },
        }

        {
            let res = self
                .tr
                .compiler
                .query("<typst-book-internal-package-meta>".to_string(), &doc);
            let res = self
                .tr
                .report(res)
                .ok_or_else(|| error_once!("retrieve book meta from book.toml"))?;
            let res = serde_json::to_value(&res)
                .map_err(map_string_err("convert_to<InternalPackageMeta>"))?;
            let res: Json<InternalPackageMeta> = serde_json::from_value(res)
                .map_err(map_string_err("convert_to<InternalPackageMeta>"))?;

            if res.len() > 1 {
                return Err(error_once!("multiple internal-package meta in book.toml"));
            }

            let package_meta = res
                .first()
                .ok_or_else(|| error_once!("no internal-package meta in book.typ (are you using old book package?, please import @preview/book:0.2.4; or do you forget the show rule `#show: book`?)"))?;

            let InternalPackageMeta::Package { version } = &package_meta.value;
            if version != "0.2.4" {
                return Err(error_once!(
                    "outdated book package, please import @preview/book:0.2.4", importing_version: version,
                ));
            }
        }

        {
            let res = self
                .tr
                .compiler
                .query("<typst-book-book-meta>".to_string(), &doc);
            let res = self
                .tr
                .report(res)
                .ok_or_else(|| error_once!("retrieve book meta from book.toml"))?;
            let res = serde_json::to_value(&res).map_err(map_string_err("convert_to<BookMeta>"))?;
            let res: Json<BookMeta> =
                serde_json::from_value(res).map_err(map_string_err("convert_to<BookMeta>"))?;

            if res.len() > 1 {
                return Err(error_once!("multiple book meta in book.toml"));
            }

            let book_meta = res
                .first()
                .ok_or_else(|| error_once!("no book meta in book.toml"))?;

            let book_meta = book_meta.value.clone();
            self.book_meta = Some(book_meta);
        }

        {
            let res = self
                .tr
                .compiler
                .query("<typst-book-build-meta>".to_string(), &doc);
            let res = self
                .tr
                .report(res)
                .ok_or_else(|| error_once!("retrieve build meta from book.toml"))?;
            let res =
                serde_json::to_value(&res).map_err(map_string_err("convert_to<BuildMeta>"))?;
            let res: Json<BuildMeta> =
                serde_json::from_value(res).map_err(map_string_err("convert_to<BuildMeta>"))?;

            if res.len() > 1 {
                return Err(error_once!("multiple build meta in book.toml"));
            }

            if let Some(res) = res.first() {
                let build_meta = res.value.clone();
                self.build_meta = Some(build_meta);
            }
        }

        Ok(())
    }

    pub fn build(&mut self) -> ZResult<()> {
        let mut write_index = false;

        create_dirs(&self.dest_dir)?;
        copy_dir_embedded(
            include_dir!("$CARGO_MANIFEST_DIR/../themes/mdbook/css"),
            self.dest_dir.join("css"),
        )?;
        copy_dir_embedded(
            include_dir!("$CARGO_MANIFEST_DIR/../themes/mdbook/FontAwesome/css"),
            self.dest_dir.join("FontAwesome/css"),
        )?;
        copy_dir_embedded(
            include_dir!("$CARGO_MANIFEST_DIR/../themes/mdbook/FontAwesome/fonts"),
            self.dest_dir.join("FontAwesome/fonts"),
        )?;

        // todo use themes in filesystem
        // copy_dir_all("themes/mdbook/css", self.dest_dir.join("css")).unwrap();
        // copy_dir_all(
        //     "themes/mdbook/fontAwesome",
        //     self.dest_dir.join("fontAwesome"),
        // )
        // .unwrap();

        // copy files
        create_dirs(self.dest_dir.join("renderer"))?;
        write_file(
            self.dest_dir.join("renderer/typst_ts_renderer_bg.wasm"),
            include_bytes!("../../assets/artifacts/typst_ts_renderer_bg.wasm"),
        )?;
        write_file(
            self.dest_dir.join("svg_utils.js"),
            include_bytes!("../../assets/artifacts/svg_utils.cjs"),
        )?;
        write_file(
            self.dest_dir.join("typst-book.js"),
            include_bytes!("../../assets/artifacts/book.mjs"),
        )?;
        write_file(
            self.dest_dir.join("index.js"),
            include_bytes!("../../themes/mdbook/index.js"),
        )?;

        for ch in self.iter_chapters() {
            if let Some(path) = ch.get("path") {
                let raw_path: String = serde_json::from_value(path.clone())
                    .map_err(error_once_map_string!("retrieve path in book.toml", value: path))?;
                let path = &self.dest_dir.join(&raw_path);
                let path = Path::new(&path);

                let content = self.render_chapter(ch, &raw_path)?;

                create_dirs(path.parent().unwrap())?;
                write_file(path.with_extension("html"), &content)?;
                if !write_index {
                    write_file(&self.dest_dir.join("index.html"), content)?;
                    write_index = true;
                }

                // cleanup cache
                comemo::evict(5);
            }
        }

        Ok(())
    }

    pub fn evaluate_content(&self, title: &BookMetaContent) -> String {
        match title {
            BookMetaContent::PlainText { content } => content.clone(),
            BookMetaContent::Raw { content } => {
                if let Ok(c) = serde_json::from_value::<JsonContent>(content.clone()) {
                    return format!("{}", c);
                }

                warn!("unevaluated {content:#?}");
                "unevaluated title".to_owned()
            }
        }
    }

    fn iter_chapters_dfs(&self, elem: &BookMetaElem, chapters: &mut Vec<DataDict>) {
        // Create the data to inject in the template
        let mut chapter = DataDict::default();

        match elem {
            BookMetaElem::Part { title, .. } => {
                let title = self.evaluate_content(title);

                chapter.insert("part".to_owned(), json!(title));
            }
            BookMetaElem::Chapter {
                title,
                section,
                link,
                sub: subs,
            } => {
                let title = self.evaluate_content(title);

                if let Some(ref section) = section {
                    chapter.insert("section".to_owned(), json!(section.to_owned() + "."));
                }

                chapter.insert(
                    "has_sub_items".to_owned(),
                    json!((!subs.is_empty()).to_string()),
                );

                chapter.insert("name".to_owned(), json!(title));
                if let Some(ref path) = link {
                    chapter.insert("path".to_owned(), json!(path));
                }
            }
            BookMetaElem::Separator {} => {
                chapter.insert("spacer".to_owned(), json!("_spacer_"));
            }
        }

        chapters.push(chapter);

        if let BookMetaElem::Chapter { sub: subs, .. } = elem {
            for child in subs.iter() {
                self.iter_chapters_dfs(child, chapters);
            }
        }
    }

    pub fn iter_chapters(&self) -> Vec<DataDict> {
        let mut chapters = vec![];

        for item in self.book_meta.as_ref().unwrap().summary.iter() {
            self.iter_chapters_dfs(item, &mut chapters);
        }

        chapters
    }

    pub fn compile_chapter(&mut self, _ch: DataDict, path: &str) -> ZResult<String> {
        let rel_data_path = std::path::Path::new(&self.path_to_root)
            .join(path)
            .with_extension("")
            .to_str()
            .ok_or_else(|| error_once!("path_to_root is not a valid utf-8 string"))?
            // windows
            .replace('\\', "/");

        self.tr.compile_page(Path::new(path))?;

        let dynamic_load_trampoline = self
            .hr
            .handlebars
            .render(
                "typst_load_trampoline",
                &json!({
                    "rel_data_path": rel_data_path,
                }),
            )
            .map_err(map_string_err(
                "render typst_load_trampoline for compile_chapter",
            ))?;

        Ok(dynamic_load_trampoline.to_owned())
    }

    pub fn render_chapter(&mut self, chapter_data: DataDict, path: &str) -> ZResult<String> {
        let instant = std::time::Instant::now();
        log::info!("rendering chapter {}", path);
        // println!("RC = {:?}", rc);
        let data = serde_json::to_value(self.book_meta.clone())
            .map_err(map_string_err("render_chapter,convert_to<BookMeta>"))?;
        let mut data: DataDict = serde_json::from_value(data)
            .map_err(map_string_err("render_chapter,convert_to<BookMeta>"))?;

        // inject chapters
        data.insert("chapters".to_owned(), json!(self.iter_chapters()));

        let renderer_module = format!("{}renderer/typst_ts_renderer_bg.wasm", self.path_to_root);
        data.insert("renderer_module".to_owned(), json!(renderer_module));

        // inject content
        data.insert(
            "content".to_owned(),
            serde_json::Value::String(self.compile_chapter(chapter_data, path)?),
        );

        // inject path_to_root
        data.insert("path_to_root".to_owned(), json!(self.path_to_root));

        let index_html = self.hr.render_index(data, path);
        log::info!("rendering chapter {} in {:?}", path, instant.elapsed());
        Ok(index_html)
    }

    // pub fn auto_order_section(&mut self) {
    //     fn dfs_elem(elem: &mut BookMetaElem, order: &mut Vec<u64>) {
    //         match elem {
    //             BookMetaElem::Chapter {
    //                 section, sub: subs, ..
    //             } => {
    //                 if section.is_none() {
    //                     *order.last_mut().unwrap() += 1;
    //                     *section = Some(format!("{}", order.last().unwrap()));
    //                 }
    //                 for sub in subs.iter_mut() {
    //                     order.push(0);
    //                     dfs_elem(sub, order);
    //                     order.pop();
    //                 }
    //             }
    //             BookMetaElem::Part { .. } | BookMetaElem::Separator {} => {}
    //         }
    //     }

    //     let mut order = vec![0];
    //     for elem in self.book_meta.content.iter_mut() {
    //         dfs_elem(elem, &mut order);
    //     }
    // }
}
