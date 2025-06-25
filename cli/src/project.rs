use core::fmt;
use std::{
    net::SocketAddr,
    path::{Path, PathBuf},
    sync::Mutex,
};

use include_dir::include_dir;
use log::warn;
use reflexo_typst::{
    static_html,
    vfs::{notify::NotifyMessage, FsProvider},
    watch_deps, CompilerExt, TypstDocument, WorldDeps,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::mpsc;

use crate::{
    error::prelude::*,
    meta::{BookMeta, BookMetaContent, BookMetaElem, BuildMeta, HtmlMeta},
    render::{DataDict, HtmlRenderContext, HtmlRenderer, SearchCtx, SearchRenderer, TypstRenderer},
    theme::Theme,
    tui, tui_error, tui_hint, tui_info,
    utils::{create_dirs, make_absolute, release_packages, write_file, UnwrapOrExit},
    CompileArgs, MetaSource, RenderMode,
};

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
    pub theme: Theme,

    pub render_mode: RenderMode,
    pub tr: TypstRenderer,
    pub hr: HtmlRenderer,

    pub book_meta: Option<BookMeta>,
    pub build_meta: Option<BuildMeta>,
    pub html_meta: Option<HtmlMeta>,
    pub chapters: Vec<DataDict>,

    pub dest_dir: PathBuf,
    pub args: CompileArgs,
    pub path_to_root: String,
    pub meta_source: MetaSource,
}

impl Project {
    pub fn new(mut args: CompileArgs) -> Result<Self> {
        let path_to_root = args.path_to_root.clone();

        if !path_to_root.starts_with('/') {
            args.path_to_root = "/".to_owned() + &args.path_to_root;
        }

        if !path_to_root.ends_with('/') {
            args.path_to_root.push('/');
        }

        let meta_source = args.meta_source.clone();
        let render_mode = args.mode.clone();

        make_absolute(Path::new(&args.dir))
            .to_str()
            .unwrap()
            .clone_into(&mut args.dir);

        let dir = Path::new(&args.dir);
        if dir.is_file() {
            if meta_source == MetaSource::Strict {
                return Err(error_once!("project dir is a file", dir: dir.display()));
            }
            let w = dir.parent().unwrap().to_str().unwrap().to_owned();
            args.dir = w;
        }

        if args.workspace.is_empty() {
            args.workspace.clone_from(&args.dir);
        }

        let theme = match &args.theme {
            Some(theme) => Theme::new(Path::new(theme))?,
            None => Theme::default(),
        };

        let raw_args = args.clone();
        let tr = TypstRenderer::new(args);
        let hr = HtmlRenderer::new(&theme);

        let mut proj = Self {
            dest_dir: tr.ctx.dest_dir.clone(),
            args: raw_args,

            theme,
            tr,
            hr,
            render_mode,

            book_meta: None,
            build_meta: None,
            html_meta: None,
            chapters: vec![],
            path_to_root,
            meta_source,
        };

        release_packages(
            &mut proj.tr.universe_mut().snapshot(),
            include_dir!("$CARGO_MANIFEST_DIR/../packages/shiroa"),
        );

        proj.build_meta()?;
        Ok(proj)
    }

    fn build_meta(&mut self) -> Result<()> {
        let args = &self.args;
        let meta_source = args.meta_source.clone();
        let mut final_dest_dir = args.dest_dir.clone();

        let dir = Path::new(&args.dir);
        let mut entry_file = None;
        if dir.is_file() {
            if meta_source == MetaSource::Strict {
                return Err(error_once!("project dir is a file", dir: dir.display()));
            }
            entry_file = Some(dir.to_owned());
        }

        if matches!(self.meta_source, MetaSource::Strict) {
            assert!(entry_file.is_none());
            self.compile_meta()?;
        }

        if final_dest_dir.is_empty() {
            if let Some(dest_dir) = self.build_meta.as_ref().map(|b| b.dest_dir.clone()) {
                final_dest_dir = dest_dir;
            }
        }

        if final_dest_dir.is_empty() {
            "dist".clone_into(&mut final_dest_dir);
        }

        self.tr.ctx.fix_dest_dir(Path::new(&final_dest_dir));
        self.dest_dir.clone_from(&self.tr.ctx.dest_dir);

        if matches!(self.meta_source, MetaSource::Outline) {
            assert!(entry_file.is_some());
            self.infer_meta_by_outline(entry_file.unwrap())?;
        }

        Ok(())
    }

    fn compile_meta(&mut self) -> Result<()> {
        #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
        struct QueryItem<T> {
            pub value: T,
        }

        type Json<T> = Vec<QueryItem<T>>;

        let (task, doc) = self.tr.compile_book(Path::new("book.typ"))?;

        #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
        pub enum InternalPackageMeta {
            /// The version of the package used by users
            #[serde(rename = "package")]
            Package { version: String },
        }

        let g = &task.graph;
        {
            let res = g.query("<shiroa-internal-package-meta>".to_string(), &doc);
            let res = task
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
                .ok_or_else(|| error_once!("no internal-package meta in book.typ (are you using old book package?, please import @preview/shiroa:0.2.3; or do you forget the show rule `#show: book`?)"))?;

            let InternalPackageMeta::Package { version } = &package_meta.value;
            if version != "0.2.3" {
                return Err(error_once!(
                    "outdated book package, please import @preview/shiroa:0.2.3", importing_version: version,
                ));
            }
        }

        {
            let res = g.query("<shiroa-book-meta>".to_string(), &doc);
            let res = task
                .report(res)
                .ok_or_else(|| error_once!("retrieve book meta from book.typ"))?;
            let res = serde_json::to_value(&res).map_err(map_string_err("convert_to<BookMeta>"))?;
            let res: Json<BookMeta> =
                serde_json::from_value(res).map_err(map_string_err("convert_to<BookMeta>"))?;

            if res.len() > 1 {
                return Err(error_once!("multiple book meta in book.typ"));
            }

            let book_meta = res
                .first()
                .ok_or_else(|| error_once!("no book meta in book.typ"))?;

            let book_meta = book_meta.value.clone();
            self.book_meta = Some(book_meta);
        }

        {
            let res = g.query("<shiroa-build-meta>".to_string(), &doc);
            let res = task
                .report(res)
                .ok_or_else(|| error_once!("retrieve build meta from book.typ"))?;
            let res =
                serde_json::to_value(&res).map_err(map_string_err("convert_to<BuildMeta>"))?;
            let res: Json<BuildMeta> =
                serde_json::from_value(res).map_err(map_string_err("convert_to<BuildMeta>"))?;

            if res.len() > 1 {
                return Err(error_once!("multiple build meta in book.typ"));
            }

            if let Some(res) = res.first() {
                let build_meta = res.value.clone();
                self.build_meta = Some(build_meta);
            }
        }

        {
            let res = g.query("<shiroa-html-meta>".to_string(), &doc);
            let res = task
                .report(res)
                .ok_or_else(|| error_once!("retrieve html meta from book.typ"))?;
            let res = serde_json::to_value(&res).map_err(map_string_err("convert_to<HtmlMeta>"))?;
            let res: Json<HtmlMeta> =
                serde_json::from_value(res).map_err(map_string_err("convert_to<HtmlMeta>"))?;

            if res.len() > 1 {
                return Err(error_once!("multiple build meta in book.typ"));
            }

            self.html_meta = res.first().map(|item| item.value.clone());
        }

        self.tr.ctx = task.ctx;
        Ok(())
    }

    fn infer_meta_by_outline(&mut self, entry: PathBuf) -> Result<()> {
        // println!("entry = {:?}, root = {:?}", entry, self.tr.root_dir);
        let entry = entry.strip_prefix(&self.tr.ctx.root_dir).unwrap_or_exit();
        let (task, doc) = self.tr.compile_book(entry)?;

        // let outline = crate::outline::outline(&doc);
        // println!("outline: {:#?}", outline);

        let chapters = self.tr.compile_pages_by_outline(entry)?;
        self.chapters = self.generate_chapters(&chapters);

        let info = &doc.info();
        let title = info.title.as_ref().map(|t| t.as_str());
        let authors = info.author.iter().map(|a| a.as_str().to_owned()).collect();

        self.book_meta = Some(BookMeta {
            title: title.unwrap_or("Typst Document").to_owned(),
            authors,
            language: "en".to_owned(),
            summary: chapters,
            ..Default::default()
        });

        self.tr.ctx = task.ctx;
        Ok(())
    }

    pub async fn watch(&mut self, addr: Option<SocketAddr>) {
        let _ = self.build();
        let (dep_tx, dep_rx) = mpsc::unbounded_channel();
        let (fs_tx, mut fs_rx) = mpsc::unbounded_channel();
        tokio::spawn(watch_deps(dep_rx, move |event| {
            fs_tx.send(event).unwrap();
        }));
        loop {
            // Notify the new file dependencies.
            let mut deps = vec![];
            let snap = self.tr.snapshot();
            let mut world = snap.world.clone();
            world.iter_dependencies(&mut |dep| {
                if let Ok(x) = world.file_path(dep).and_then(|e| e.to_err()) {
                    deps.push(x.into())
                }
            });
            tui_info!("Watching {} files for changes...", deps.len());
            if let Some(addr) = &addr {
                tui_hint!("Server started at http://{addr}");
            }

            let _ = dep_tx.send(NotifyMessage::SyncDependency(Box::new(deps)));

            if self.need_compile() {
                comemo::evict(10);
                world.evict_source_cache(30);
                world.evict_vfs(60);
            }

            let Some(event) = fs_rx.recv().await else {
                break;
            };

            let _ = tui::clear();

            // todo: reset_snapshot looks not good
            self.tr.reset_snapshot();
            self.tr.universe_mut().increment_revision(|verse| {
                verse.vfs().notify_fs_event(event);
            });
            let _ = self.build_meta();
            let _ = self.compile_once(SearchRenderer::default());
        }
    }

    pub fn build(&mut self) -> Result<()> {
        let sr = SearchRenderer::default();
        self.extract_assets(&sr)?;
        self.compile_once(sr)?;

        Ok(())
    }

    fn extract_assets(&mut self, sr: &SearchRenderer) -> Result<()> {
        // Always update the theme if it is static
        // Or copy on first build
        let themes = self.dest_dir.join("theme");
        if self.theme.is_static() || !themes.exists() {
            log::info!("copying theme assets to {themes:?}");
            self.theme.copy_assets(&themes)?;
        }

        // copy internal files
        create_dirs(self.dest_dir.join("internal"))?;
        write_file(
            self.dest_dir.join("internal/typst_ts_renderer_bg.wasm"),
            include_bytes!("../../assets/artifacts/typst_ts_renderer_bg.wasm"),
        )?;
        write_file(
            self.dest_dir.join("internal/svg_utils.js"),
            include_bytes!("../../assets/artifacts/svg_utils.cjs"),
        )?;
        write_file(
            self.dest_dir.join("internal/shiroa.js"),
            include_bytes!("../../assets/artifacts/book.mjs"),
        )?;

        if sr.config.copy_js {
            write_file(
                self.dest_dir.join("internal/searcher.js"),
                include_bytes!("../../assets/artifacts/searcher.js"),
            )?;
            write_file(
                self.dest_dir.join("internal/mark.min.js"),
                include_bytes!("../../assets/artifacts/mark.min.js"),
            )?;
            write_file(
                self.dest_dir.join("internal/elasticlunr.min.js"),
                include_bytes!("../../assets/artifacts/elasticlunr.min.js"),
            )?;
        }

        Ok(())
    }

    fn compile_once(&mut self, mut sr: SearchRenderer) -> Result<()> {
        self.prepare_chapters();

        let serach_ctx = SearchCtx {
            config: &sr.config,
            items: Mutex::new(vec![]),
        };

        self.hr.render_chapters(
            HtmlRenderContext {
                book_meta: self.book_meta.as_ref().unwrap_or(&Default::default()),
                html_meta: self.html_meta.as_ref().unwrap_or(&Default::default()),
                search: &serach_ctx,
                dest_dir: &self.dest_dir,
                path_to_root: &self.path_to_root,
                chapters: &self.chapters,
            },
            self.chapters.clone(), // todo: only render changed
            |path| self.compile_chapter(path),
        )?;

        sr.build(&serach_ctx.items.into_inner().unwrap())?;

        if sr.config.copy_js {
            sr.render_search_index(&self.dest_dir)?;
        }

        Ok(())
    }

    fn prepare_chapters(&mut self) {
        match self.meta_source {
            MetaSource::Strict => {
                self.chapters = self.generate_chapters(&self.book_meta.as_ref().unwrap().summary)
            }
            MetaSource::Outline => {}
        }
    }

    fn generate_chapters(&self, meta: &[BookMetaElem]) -> Vec<DataDict> {
        let mut chapters = vec![];

        for item in meta.iter() {
            self.iter_chapters_dfs(item, &mut chapters);
        }

        chapters
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

    fn evaluate_content(&self, title: &BookMetaContent) -> String {
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

    fn compile_chapter(&self, path: &str) -> Result<ChapterArtifact> {
        tui_info!(h "Compiling", "{path}");
        let instant = std::time::Instant::now();
        let res = self.compile_chapter_(path);
        let elapsed = instant.elapsed();
        if let Err(e) = &res {
            tui_error!("{path}: compile error: {e}");
        } else {
            tui_info!(h "Finished", "{path} in {elapsed:.3?}");
        }

        res
    }

    fn compile_chapter_(&self, path: &str) -> Result<ChapterArtifact> {
        let file_name = Path::new(&self.path_to_root).join(path).with_extension("");

        // todo: description for single document
        let doc = if self.need_compile() {
            let doc = self.tr.compile_page(Path::new(path))?;
            Some(doc)
        } else {
            None
        };

        let auto_description = || {
            let full_digest = doc.as_ref().map(TypstRenderer::generate_desc).transpose()?;
            let full_digest = full_digest.unwrap_or_default();
            Result::Ok(match full_digest.char_indices().nth(512) {
                Some((idx, _)) => full_digest[..idx].to_owned(),
                None => full_digest,
            })
        };

        let rel_data_path = file_name
            .to_str()
            .ok_or_else(|| error_once!("path_to_root is not a valid utf-8 string"))?
            // windows
            .replace('\\', "/");

        let (description, content) = match self.render_mode.clone() {
            RenderMode::StaticHtml => {
                let doc = doc
                    .as_ref()
                    .expect("doc is not compiled in StaticHtml mode");
                let html_doc = match doc {
                    TypstDocument::Html(doc) => doc,
                    _ => bail!("doc is not Html"),
                };

                let content = self
                    .hr
                    .handlebars
                    .render(
                        "typst_load_html_trampoline",
                        &json!({
                            "rel_data_path": rel_data_path,
                        }),
                    )
                    .map_err(map_string_err(
                        "render typst_load_html_trampoline for compile_chapter",
                    ))?;

                let res = self
                    .tr
                    .report(static_html(html_doc))
                    .expect("failed to render static html");

                let description: Option<Result<String>> = res.description().map(From::from).map(Ok);
                (
                    description.unwrap_or_else(auto_description)?,
                    format!(
                        r#"{content}<div class="typst-preload-content" style="display: none">{}</div>"#,
                        res.body
                    ),
                )
            }
            RenderMode::DynPaged | RenderMode::StaticHtmlDynPaged => {
                let content = self
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

                (auto_description()?, content)
            }
        };

        Ok(ChapterArtifact {
            content,
            description,
        })
    }

    fn need_compile(&self) -> bool {
        matches!(self.meta_source, MetaSource::Strict)
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

pub struct ChapterArtifact {
    pub description: String,
    pub content: String,
}
