use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::json;
use typst_ts_compiler::service::{Compiler, DiagObserver};

use crate::{
    error::prelude::*,
    meta::{BookMeta, BookMetaContent, BookMetaElem, BuildMeta},
    render::{DataDict, HtmlRenderer, TypstRenderer},
    utils::release_packages,
    CompileArgs,
};
use include_dir::include_dir;

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

        self.tr.setup_entry(Path::new("book.typ"));
        let doc = self
            .tr
            .compiler
            .with_compile_diag::<false, _>(|c| c.pure_compile())
            .ok_or_else(|| error_once!("compile_meta"))?;

        {
            let res = self
                .tr
                .compiler
                .with_compile_diag::<false, _>(|c| {
                    c.query("<typst-book-book-meta>".to_string(), &doc)
                })
                .ok_or_else(|| error_once!("retrieve book meta from book.toml"))?;
            let res =
                serde_json::to_value(&res).map_err(map_string_err("convert_to<BookMeeta>"))?;
            let res: Json<BookMeta> =
                serde_json::from_value(res).map_err(map_string_err("convert_to<BookMeeta>"))?;

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
                .with_compile_diag::<false, _>(|c| {
                    c.query("<typst-book-build-meta>".to_string(), &doc)
                })
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

    pub fn iter_chapters(&self) -> Vec<DataDict> {
        let mut chapters = vec![];

        fn dfs_elem(elem: &BookMetaElem, chapters: &mut Vec<DataDict>) {
            // Create the data to inject in the template
            let mut chapter = DataDict::default();

            match elem {
                BookMetaElem::Part { title, .. } => {
                    let BookMetaContent::PlainText { content: title } = title;

                    chapter.insert("part".to_owned(), json!(title));
                }
                BookMetaElem::Chapter {
                    title,
                    section,
                    link,
                    sub: subs,
                } => {
                    let BookMetaContent::PlainText { content: title } = title;

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
                    dfs_elem(child, chapters);
                }
            }
        }

        for item in self.book_meta.as_ref().unwrap().summary.iter() {
            dfs_elem(item, &mut chapters);
        }

        chapters
    }

    pub fn compile_chapter(&mut self, _ch: DataDict, path: &str) -> ZResult<String> {
        let renderer_module = format!("{}renderer/typst_ts_renderer_bg.wasm", self.path_to_root);
        let rel_data_path = std::path::Path::new(&self.path_to_root)
            .join(path)
            .with_extension("")
            .to_str()
            .ok_or_else(|| error_once!("path_to_root is not a valid utf-8 string"))?
            // windows
            .replace('\\', "/");

        self.tr.setup_entry(Path::new(path));

        self.tr
            .compiler
            .with_compile_diag::<true, _>(|c| c.compile())
            .ok_or_else(|| error_once!("compile_chapter"))?;

        let dynamic_load_trampoline = self
            .hr
            .handlebars
            .render(
                "typst_load_trampoline",
                &json!({
                    "renderer_module" : renderer_module,
                    "rel_data_path": rel_data_path,
                }),
            )
            .map_err(map_string_err(
                "render typst_load_trampoline for compile_chapter",
            ))?;

        Ok(dynamic_load_trampoline.to_owned())
    }

    pub fn render_chapter(&mut self, chapter_data: DataDict, path: &str) -> ZResult<String> {
        let data = serde_json::to_value(self.book_meta.clone())
            .map_err(map_string_err("render_chapter,convert_to<BookMeeta>"))?;
        let mut data: DataDict = serde_json::from_value(data)
            .map_err(map_string_err("render_chapter,convert_to<BookMeeta>"))?;

        // inject chapters
        data.insert("chapters".to_owned(), json!(self.iter_chapters()));

        // inject content
        data.insert(
            "content".to_owned(),
            serde_json::Value::String(self.compile_chapter(chapter_data, path)?),
        );

        // inject path_to_root
        data.insert("path_to_root".to_owned(), json!(self.path_to_root));

        Ok(self.hr.render_index(data, path))
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
