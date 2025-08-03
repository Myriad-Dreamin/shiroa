use core::fmt;
use std::{
    collections::BTreeMap,
    net::SocketAddr,
    path::{Path, PathBuf},
    sync::Mutex,
};

use include_dir::include_dir;
use log::warn;
use reflexo_typst::{
    path::unix_slash,
    static_html,
    vfs::{notify::NotifyMessage, FilesystemEvent, FsProvider},
    watch_deps, CompilerExt, ImmutStr, TypstDocument, TypstSystemWorld, WorldDeps,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::{broadcast, mpsc};
use typst::foundations::Content;

use crate::{
    error::prelude::*,
    meta::{BookMeta, BookMetaContent, BookMetaElem, BuildMeta},
    render::{DataDict, HtmlRenderContext, SearchCtx, SearchRenderer, TypstRenderer},
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
    pub render_mode: RenderMode,
    pub tr: TypstRenderer,

    pub book_meta: BookMeta,
    pub build_meta: Option<BuildMeta>,
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

        let raw_args = args.clone();
        let tr = TypstRenderer::new(args);

        let mut proj = Self {
            dest_dir: tr.ctx.dest_dir.clone(),
            args: raw_args,

            tr,
            render_mode,

            book_meta: Default::default(),
            build_meta: None,
            chapters: vec![],
            path_to_root,
            meta_source,
        };

        release_packages(
            &mut proj.tr.universe_mut().snapshot(),
            include_dir!("$CARGO_MANIFEST_DIR/../packages/shiroa"),
        );
        release_packages(
            &mut proj.tr.universe_mut().snapshot(),
            include_dir!("$CARGO_MANIFEST_DIR/../themes/starlight"),
        );
        release_packages(
            &mut proj.tr.universe_mut().snapshot(),
            include_dir!("$CARGO_MANIFEST_DIR/../themes/mdbook"),
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
            if let Some(build_meta) = self.build_meta.as_ref() {
                final_dest_dir = build_meta.dest_dir.clone();
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
        let (task, doc) = self.tr.compile_book(Path::new("book.typ"))?;

        let g = &task.graph;
        let query = |item: &str| {
            let res = g.query(item.to_string(), &doc);
            task.report(res).context("cannot retrieve metadata item(s)")
        };

        {
            #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
            pub enum InternalPackageMeta {
                /// The version of the package used by users
                #[serde(rename = "package")]
                Package { version: String },
            }

            let InternalPackageMeta::Package { version } = self.query_meta("<shiroa-internal-package-meta>", query)?
                .context("No package meta. are you using old book package?, please import @preview/shiroa:0.2.3; or do you forget the show rule `#show: book`?")?;

            if version != "0.2.3" {
                return Err(error_once!(
                    "outdated book package, please import @preview/shiroa:0.2.3", importing_version: version,
                ));
            }
        }

        self.book_meta = self
            .query_meta::<BookMeta>("<shiroa-book-meta>", query)?
            .context("no book meta in book.typ")?;
        if let Some(build_meta) = self.query_meta::<BuildMeta>("<shiroa-build-meta>", query)? {
            self.build_meta = Some(build_meta);
        }

        self.tr.ctx = task.ctx;
        Ok(())
    }

    fn query_meta<T: for<'a> serde::Deserialize<'a>>(
        &mut self,
        item: &str,
        f: impl FnOnce(&str) -> Result<Vec<Content>>,
    ) -> Result<Option<T>> {
        self.query_meta_::<T>(item, f)
            .with_context("while querying metadata", || {
                Some(Box::new([("label", item.to_string())]))
            })
    }

    fn query_meta_<T: for<'a> serde::Deserialize<'a>>(
        &mut self,
        item: &str,
        f: impl FnOnce(&str) -> Result<Vec<Content>>,
    ) -> Result<Option<T>> {
        #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
        struct QueryItem<T> {
            pub value: T,
        }

        let res = serde_json::to_value(&f(item)?).context("cannot convert metadata item(s)")?;
        let res: Vec<QueryItem<T>> =
            serde_json::from_value(res).context("cannot convert metadata item(s)")?;

        if res.len() > 1 {
            bail!("multiple metadata items in book.typ");
        }

        Ok(res.into_iter().next().map(|v| v.value))
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

        self.book_meta = BookMeta {
            title: title.unwrap_or("Typst Document").to_owned(),
            authors,
            language: "en".to_owned(),
            summary: chapters,
            ..Default::default()
        };

        self.tr.ctx = task.ctx;
        Ok(())
    }

    pub(crate) async fn watch(
        &mut self,
        // active_set: Arc<Mutex<HashMap<ImmutStr, usize>>>,
        mut hb_rx: mpsc::UnboundedReceiver<ServeEvent>,
        tx: broadcast::Sender<WatchSignal>,
        addr: Option<SocketAddr>,
    ) {
        let _ = self.build();
        let (dep_tx, dep_rx) = mpsc::unbounded_channel();
        let (fs_tx, mut fs_rx) = mpsc::unbounded_channel();
        tokio::spawn(watch_deps(dep_rx, move |event| {
            fs_tx.send(event).unwrap();
        }));

        let need_compile = self.need_compile();
        let finish = |world: &mut TypstSystemWorld| {
            // Notify the new file dependencies.
            let mut deps = vec![];
            world.iter_dependencies(&mut |dep| {
                if let Ok(x) = world.file_path(dep).and_then(|e| e.to_err()) {
                    deps.push(x.into())
                }
            });

            tui_info!("Watching {} files for changes...", deps.len());
            let _ = dep_tx.send(NotifyMessage::SyncDependency(Box::new(deps)));

            if need_compile {
                comemo::evict(10);
                world.evict_source_cache(30);
                world.evict_vfs(60);
            }

            if let Some(addr) = &addr {
                tui_hint!("Server started at http://{addr}");
            }
        };

        let mut snap = self.tr.snapshot();
        let mut world = snap.world.clone();
        // first report.
        finish(&mut world);

        let mut active_files: BTreeMap<ImmutStr, usize> = BTreeMap::new();
        loop {
            enum WatchEvent {
                Fs(FilesystemEvent),
                Serve(ServeEvent),
            }

            let event = tokio::select! {
                event = fs_rx.recv() => {
                    match event {
                        Some(e) => WatchEvent::Fs(e),
                        None => break,
                    }
                }
               Some(c) = hb_rx.recv() => WatchEvent::Serve(c),
            };

            // todo: reset_snapshot looks not good

            let is_heartbeat = matches!(event, WatchEvent::Serve(ServeEvent::HoldPath(..)));
            match event {
                WatchEvent::Fs(event) => {
                    self.tr.reset_snapshot();
                    self.tr.universe_mut().increment_revision(|verse| {
                        verse.vfs().notify_fs_event(event);
                    });

                    let _ = tui::clear();
                    let _ = self.build_meta();

                    snap = self.tr.snapshot();
                    world = snap.world.clone();
                }
                WatchEvent::Serve(ServeEvent::HoldPath(path, inc)) => {
                    let path = if path.as_ref() == "/" || path.is_empty() {
                        if let Some(f) = self.chapters.first() {
                            f.get("path").unwrap().as_str().unwrap().into()
                        } else {
                            continue;
                        }
                    } else if path.ends_with(".html") {
                        let path = path.trim_start_matches('/');
                        let typ_path = PathBuf::from(path);
                        unix_slash(&typ_path.with_extension("typ")).into()
                    } else {
                        path
                    };

                    let active_files = &mut active_files;
                    let mut changed = false;
                    if inc {
                        *active_files.entry(path).or_insert_with(|| {
                            changed = true;
                            0
                        }) += 1;
                    } else {
                        let count = active_files.entry(path);
                        // erase if the count is 1, otherwise decrement
                        match count {
                            std::collections::btree_map::Entry::Occupied(mut e) => {
                                if *e.get() > 1 {
                                    *e.get_mut() -= 1;
                                } else {
                                    changed = true;
                                    e.remove();
                                }
                            }
                            std::collections::btree_map::Entry::Vacant(_) => {}
                        }
                    }

                    if !changed {
                        // No changes, skip recompilation
                        continue;
                    }

                    let _ = tui::clear();
                    tui_info!("Recompiling changed chapters: {active_files:?}");

                    let _ = self.build_meta();
                }
            }

            // todo: blocking?
            let _ = self.compile_once(&active_files, SearchRenderer::new());

            if !is_heartbeat {
                let _ = tx.send(WatchSignal::Reload);
            }
            finish(&mut world);
        }
    }

    pub fn build(&mut self) -> Result<()> {
        let sr = SearchRenderer::new();
        self.extract_assets(&sr)?;
        self.compile_once(&Default::default(), sr)?;

        Ok(())
    }

    fn extract_assets(&mut self, sr: &SearchRenderer) -> Result<()> {
        // copy internal files
        create_dirs(self.dest_dir.join("internal"))?;
        write_file(
            self.dest_dir.join("internal/typst_ts_renderer_bg.wasm"),
            include_bytes!("../../assets/artifacts/typst_ts_renderer_bg.wasm"),
        )?;
        write_file(
            self.dest_dir.join("internal/svg_utils.js"),
            include_bytes!("../../assets/artifacts/svg_utils.js"),
        )?;
        write_file(
            self.dest_dir.join("internal/shiroa.js"),
            include_bytes!("../../assets/artifacts/shiroa.js"),
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

    fn compile_once(
        &mut self,
        ac: &BTreeMap<ImmutStr, usize>,
        mut sr: SearchRenderer,
    ) -> Result<()> {
        self.prepare_chapters();

        let serach_ctx = SearchCtx {
            config: &sr.config,
            items: Mutex::new(vec![]),
        };

        // Prepare the rendering settings early, to avoid pour {book,html}_meta type
        // details to the renderer.
        let mut data = DataDict::new();

        let book_meta = &self.book_meta;

        data.insert("title".to_owned(), json!(book_meta.title));
        data.insert("book_title".to_owned(), json!(book_meta.title));
        data.insert("authors".to_owned(), json!(book_meta.authors));
        data.insert("description".to_owned(), json!(book_meta.description));
        data.insert("language".to_owned(), json!(book_meta.language));
        // todo: is `repository` key ever used??
        data.insert("repository".to_owned(), json!(book_meta.repository));
        data.insert("git_repository_url".to_owned(), json!(book_meta.repository));

        data.insert("path_to_root".to_owned(), json!(self.path_to_root));

        // todo: we clone all chapters here, which looks inefficient.
        data.insert("chapters".to_owned(), json!(self.chapters));

        // Injects search configuration
        let search_config = &serach_ctx.config;
        data.insert("search_enabled".to_owned(), json!(search_config.enable));
        data.insert(
            "search_js".to_owned(),
            json!(search_config.enable && search_config.copy_js),
        );

        // Injects module path
        let renderer_module = format!("{}internal/typst_ts_renderer_bg.wasm", self.path_to_root);
        data.insert("renderer_module".to_owned(), json!(renderer_module));

        // dummy settings, will remove in future
        data.insert("fold_enable".to_owned(), json!(false));
        data.insert("fold_level".to_owned(), json!(0u64));
        data.insert("preferred_dark_theme".to_owned(), json!("ayu"));
        data.insert("default_theme".to_owned(), json!("light"));

        self.tr.render_chapters(
            HtmlRenderContext {
                book_data: &data,
                edit_url: &book_meta.repository_edit,
                search: &serach_ctx,
                dest_dir: &self.dest_dir,
            },
            &self.chapters, // todo: only render changed
            ac,
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
            MetaSource::Strict => self.chapters = self.generate_chapters(&self.book_meta.summary),
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
            // todo: divider
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
                    return format!("{c}");
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
        let task_doc = if self.need_compile() {
            Some(self.tr.compile_page(Path::new(path))?)
        } else {
            None
        };

        let auto_description = || {
            let full_digest = task_doc
                .as_ref()
                .map(|doc| &doc.1)
                .map(TypstRenderer::generate_desc)
                .transpose()?;
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
                let (task, html_doc) = match &task_doc {
                    Some((task, TypstDocument::Html(doc))) => (task, doc),
                    None => bail!("no task document for static html"),
                    _ => bail!("doc is not Html"),
                };

                let res = task
                    .report(static_html(html_doc))
                    .expect("failed to render static html");

                let content = task.report(res.html()).unwrap_or_default().to_owned();

                let description: Option<Result<String>> = res.description().map(From::from).map(Ok);
                (description.unwrap_or_else(auto_description)?, content)
            }
            RenderMode::DynPaged | RenderMode::StaticHtmlDynPaged => {
                // let content = self
                //     .hr
                //     .handlebars
                //     .render(
                //         "typst_load_trampoline",
                //         &json!({
                //             "rel_data_path": rel_data_path,
                //         }),
                //     )
                //     .map_err(map_string_err(
                //         "render typst_load_trampoline for compile_chapter",
                //     ))?;

                // (auto_description()?, content)

                todo!()
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ServeEvent {
    HoldPath(ImmutStr, bool),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum WatchSignal {
    Reload,
}
// ThemeAsset::Static(EmbeddedThemeAsset::MdBook) => {
//     copy_dir_embedded(
//         &include_dir!("$CARGO_MANIFEST_DIR/../themes/mdbook/css"),
//         &dest_dir.join("css"),
//     )?;
//     copy_dir_embedded(
//         &include_dir!("$CARGO_MANIFEST_DIR/../themes/mdbook/FontAwesome/css"
// ),         &dest_dir.join("FontAwesome/css"),
//     )?;
//     write_file(
//         dest_dir.join("index.js"),
//         include_bytes!("../../themes/mdbook/index.js"),
//     )?;
// }
