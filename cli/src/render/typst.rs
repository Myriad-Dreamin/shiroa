use std::{
    borrow::Cow,
    cell::RefCell,
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    rc::Rc,
    sync::{Arc, OnceLock},
};

use crate::{
    diag::print_diagnostics,
    error::prelude::*,
    meta::BookMetaElem,
    outline::{OutlineItem, SpanInternerImpl},
    utils::{make_absolute, make_absolute_from, UnwrapOrExit},
    CompileArgs, RenderMode,
};
use reflexo_typst::{
    config::CompileOpts,
    escape::{escape_str, AttributeEscapes},
    path::PathClean,
    static_html,
    system::SystemWorldComputeGraph,
    vector::{
        ir::{LayoutRegionNode, Module, Page, PageMetadata},
        pass::Typst2VecPass,
        IntoTypst,
    },
    world::EntryOpts,
    CompilationTask, DiagnosticFormat, DiagnosticHandler, DynSvgModuleExport, EntryReader,
    ExportDynSvgModuleTask, FlagTask, LazyHash, TakeAs, TaskInputs, TextExport, TypstAbs,
    TypstDict, TypstDocument, TypstHtmlDocument, TypstPagedDocument,
};
use reflexo_typst::{CompileReport, TypstSystemUniverse};
use reflexo_vec2svg::{
    ir::{SizedRawHtmlItem, ToItemMap, VecItem},
    MultiVecDocument,
};
use serde::Deserialize;
use typst::{
    diag::{SourceResult, Warned},
    ecow::{EcoString, EcoVec},
    foundations::{IntoValue, Regex},
};
// serialize_doc, LayoutRegionNode,

const THEME_LIST: [&str; 5] = ["light", "rust", "coal", "navy", "ayu"];

#[derive(Debug, Clone, Default)]
pub struct CompilePageSetting {
    pub with_outline: bool,
}

pub struct TypstRenderer {
    pub verse: TypstSystemUniverse,
    pub extension: EcoString,
    pub output: PathBuf,
    pub compiler: ExportDynSvgModuleTask,
    pub root_dir: PathBuf,
    pub dest_dir: PathBuf,
    static_html: bool,
    pub diag_handler: DiagnosticHandler,
}

impl TypstRenderer {
    pub fn new(args: CompileArgs) -> Self {
        let workspace_dir = make_absolute(Path::new(&args.workspace)).clean();
        let root_dir = make_absolute(Path::new(&args.dir)).clean();
        let dest_dir = make_absolute_from(Path::new(&args.dest_dir), || root_dir.clone()).clean();

        let verse = TypstSystemUniverse::new(CompileOpts {
            entry: EntryOpts::new_workspace(workspace_dir.clone()),
            font_paths: args.font_paths.clone(),
            with_embedded_fonts: typst_assets::fonts().map(Cow::Borrowed).collect(),
            ..CompileOpts::default()
        })
        .unwrap_or_exit();

        let mut compiler = ExportDynSvgModuleTask::new();
        compiler.html_format = matches!(
            args.mode,
            RenderMode::StaticHtmlDynPaged | RenderMode::StaticHtml
        );
        compiler.set_command_executor(Arc::new(ShiroaCommands(
            args.allowed_url_source
                .map(|s| Arc::new(Regex::new(&s).context("invalid regex").unwrap_or_exit())),
        )));
        // compiler.set_extension("multi.sir.in".to_owned());
        compiler.set_layout_widths([750., 650., 550., 450., 350.].map(TypstAbs::pt).into());
        // let compiler =
        //     CompileReporter::new(compiler).
        // with_generic_reporter(ConsoleDiagReporter::default());

        // let compiler = CompileDriver::new(compiler, verse);

        Self {
            output: dest_dir.clone(),
            verse,
            compiler,
            root_dir,
            dest_dir,
            extension: "multi.sir.in".into(),
            static_html: args.mode == RenderMode::StaticHtml,
            diag_handler: DiagnosticHandler {
                print_compile_status: true,
                diagnostic_format: Default::default(),
            },
        }
    }

    pub fn universe(&self) -> &TypstSystemUniverse {
        &self.verse
    }

    pub fn universe_mut(&mut self) -> &mut TypstSystemUniverse {
        &mut self.verse
    }

    pub fn fix_dest_dir(&mut self, path: &Path) {
        let dest_dir = make_absolute_from(path, || self.root_dir.clone()).clean();
        self.dest_dir = dest_dir;
    }

    fn module_dest_path(&self) -> PathBuf {
        self.dest_dir
            .join(&self.output)
            .with_extension(self.extension.as_str())
    }

    fn set_theme_target(&mut self, theme: &str) {
        let prefix = if self.static_html { "html" } else { "web" };

        self.compiler.set_target(if theme.is_empty() {
            prefix.to_owned()
        } else {
            format!("{prefix}-{theme}")
        });

        if self.static_html {
            self.extension = if theme.is_empty() {
                "html".into()
            } else {
                format!("{theme}.html").into()
            };
        } else {
            self.extension = if theme.is_empty() {
                "multi.sir.in".into()
            } else {
                format!("{theme}.multi.sir.in").into()
            };
        };
    }

    fn setup_entry(&mut self, path: &Path) {
        if path.is_absolute() {
            panic!("entry file must be relative to the workspace");
        }
        let entry = self.root_dir.join(path).clean().as_path().into();
        let err = self
            .universe_mut()
            .increment_revision(|v| v.set_entry_file(entry));
        if err.is_err() {
            self.report(err);
            panic!("failed to set entry file");
        }
        let output_path = self.dest_dir.join(path).with_extension("").clean();
        std::fs::create_dir_all(output_path.parent().unwrap()).unwrap_or_exit();
        self.output = output_path;
    }

    // todo: we should use same snapshot as that compiled documents
    pub fn report<T>(&self, may_value: SourceResult<T>) -> Option<T> {
        self.report_with_warnings(Warned {
            output: may_value,
            warnings: Default::default(),
        })
    }

    // todo: we should use same snapshot as that compiled documents
    pub fn report_with_warnings<T>(&self, may_value: Warned<SourceResult<T>>) -> Option<T> {
        let (res, diag, rep) = match may_value.output {
            Ok(v) => {
                let rep = CompileReport::CompileSuccess(
                    self.universe().main_id().unwrap(),
                    may_value.warnings.len(),
                    Default::default(),
                );

                (Some(v), EcoVec::default(), rep)
            }
            Err(err) => {
                let rep = CompileReport::CompileError(
                    self.universe().main_id().unwrap(),
                    err.len(),
                    Default::default(),
                );
                (None, err, rep)
            }
        };

        // todo: use same world as the reportee
        let world = self.verse.snapshot();

        // We currently ignore export error here
        // We lock it once to avoid concurrent write
        // todo: merge to upstream
        // self.diag_handler
        //     .report(&world, diag.iter().chain(may_value.warnings.iter()));

        let diag = diag.iter().chain(may_value.warnings.iter());
        let diagnostics = diag.filter(no_foreign_obj_diag);
        let _ = print_diagnostics(
            &world,
            diagnostics,
            DiagnosticFormat::Human,
            &mut crate::tui::out().lock(),
        );

        self.diag_handler.status(&rep);
        res
    }

    // todo: we should use same snapshot as that compiled documents
    pub fn compile_book(
        &mut self,
        path: &Path,
    ) -> Result<(Arc<SystemWorldComputeGraph>, TypstDocument)> {
        self.setup_entry(path);
        self.set_theme_target("");

        let graph = self.universe().computation();
        let res = typst::compile::<TypstPagedDocument>(&graph.snap.world);
        let res = self
            .report_with_warnings(res)
            .map(Arc::new)
            .map(TypstDocument::Paged);

        Ok((graph, res.ok_or_else(|| error_once!("compile book.typ"))?))
    }

    pub fn compile_pages_by_outline(&mut self, path: &Path) -> Result<Vec<BookMetaElem>> {
        // compile entry file as a single webpage
        self.compile_page_with(path, CompilePageSetting { with_outline: true })?;
        self.setup_entry(path);

        let mut res = None;
        for theme in THEME_LIST {
            self.set_theme_target(theme);
            let incoming = self.compile_pages_by_outline_(theme)?;

            // todo: compare incoming with res
            res = Some(incoming);
        }

        res.ok_or_else(|| error_once!("compile pages by outline"))
    }

    fn compile_pages_by_outline_(&mut self, theme: &'static str) -> Result<Vec<BookMetaElem>> {
        // read ir from disk
        let module_output = self.module_dest_path();
        let module_bin = std::fs::read(module_output).unwrap_or_exit();

        let doc = MultiVecDocument::from_slice(&module_bin);
        // println!("layouts: {:#?}", doc.layouts);

        // todo(warning): warn if the relationship is not stable across layouts //
        // todo(warning): warn if there is a single layout
        // todo: deduplicate layout if possible

        type PagesRef = Rc<RefCell<Vec<usize>>>;

        #[derive(Default)]
        struct ModuleInterner {
            inner: Typst2VecPass,
            pages_list: Vec<Vec<usize>>,
        }

        impl ModuleInterner {
            fn finalize(self, origin: &MultiVecDocument) -> MultiVecDocument {
                let Self { inner, pages_list } = self;

                let fonts = inner.glyphs.used_fonts;
                let glyphs = inner.glyphs.used_glyphs;

                let font_mapping = fonts
                    .iter()
                    .map(|e| (e.hash, origin.module.get_font(e)))
                    .collect::<HashMap<_, _>>();

                let glyphs = glyphs
                    .into_iter()
                    .flat_map(|k| {
                        Some((k, {
                            let font = (*(font_mapping.get(&k.font_hash)?))?;
                            font.get_glyph(k.glyph_idx)?.as_ref().clone()
                        }))
                    })
                    .collect();

                // Keep all fonts so that we doesn't have to change the font indices
                let fonts = origin.module.fonts.clone();

                let module = Module {
                    fonts,
                    glyphs,
                    items: inner.items.to_item_map(),
                };

                log::trace!(
                    "module: {:?} {:?} {:?}",
                    module.fonts.len(),
                    module.glyphs.len(),
                    module.items.len()
                );

                let mut pages_list = pages_list.into_iter();
                let layouts = origin.layouts.iter().cloned().map(|l| {
                    l.mutate_pages(&mut |(meta, pages)| {
                        // delete outline
                        for c in meta {
                            if let PageMetadata::Custom(c) = c {
                                c.retain(|(k, _)| k.as_ref() != "outline");
                            }
                        }

                        let page_idxs = pages_list.next();
                        if let Some(page_idxs) = page_idxs {
                            *pages = page_idxs
                                .into_iter()
                                .map(|idx| pages[idx - 1].clone())
                                .collect::<Vec<_>>();
                        }
                    })
                });

                MultiVecDocument {
                    module,
                    layouts: layouts.collect(),
                }
            }
        }

        #[derive(Debug)]
        struct OutlineItemRef {
            item: BookMetaElem,
            pages: PagesRef,
            children: Vec<OutlineItemRef>,
        }

        struct OutlineChapter {
            item: BookMetaElem,
            content: Option<ModuleInterner>,
            children: Vec<OutlineChapter>,
        }

        struct BuiltOutline {
            prefix: Option<ModuleInterner>,
            chapters: Vec<OutlineChapter>,
        }
        impl BuiltOutline {
            fn intern_pages(
                interner: &mut Option<ModuleInterner>,
                module: &Module,
                pages: &[Page],
                page_idxs: impl Iterator<Item = usize>,
            ) {
                let mut builder = interner.take().unwrap_or_default();
                let page_idxs = page_idxs.collect::<Vec<_>>();
                for idx in &page_idxs {
                    builder.inner.intern(module, &pages[*idx - 1].content);
                }
                builder.pages_list.push(page_idxs);
                *interner = Some(builder);
            }

            fn init(
                module: &Module,
                builder: ItemRefBuilder,
                pages: &[Page],
                items: Vec<OutlineItemRef>,
            ) -> BuiltOutline {
                let mut prefix = None;
                Self::intern_pages(
                    &mut prefix,
                    module,
                    pages,
                    builder.prefix.borrow().iter().cloned(),
                );

                let chapters = Self::init_items(module, pages, items);

                BuiltOutline { prefix, chapters }
            }

            fn init_items(
                module: &Module,
                pages: &[Page],
                items: Vec<OutlineItemRef>,
            ) -> Vec<OutlineChapter> {
                items
                    .into_iter()
                    .map(|item| {
                        let mut content = None;
                        Self::intern_pages(
                            &mut content,
                            module,
                            pages,
                            item.pages.borrow().iter().cloned(),
                        );

                        OutlineChapter {
                            item: item.item,
                            content,
                            children: Self::init_items(module, pages, item.children),
                        }
                    })
                    .collect()
            }

            fn merge(
                &mut self,
                module: &Module,
                builder: ItemRefBuilder,
                pages: &[Page],
                items: Vec<OutlineItemRef>,
            ) {
                Self::intern_pages(
                    &mut self.prefix,
                    module,
                    pages,
                    builder.prefix.borrow().iter().cloned(),
                );

                Self::merge_items(module, pages, &mut self.chapters, items);
            }

            fn merge_items(
                module: &Module,
                pages: &[Page],
                chapters: &mut [OutlineChapter],
                items: Vec<OutlineItemRef>,
            ) {
                if items.len() != chapters.len() {
                    panic!("cannot merge outline with different chapter count");
                }
                for (idx, item) in items.into_iter().enumerate() {
                    let chapter = &mut chapters[idx];

                    if chapter.item != item.item {
                        panic!("cannot merge outline with different chapter");
                    }

                    Self::intern_pages(
                        &mut chapter.content,
                        module,
                        pages,
                        item.pages.borrow().iter().cloned(),
                    );

                    Self::merge_items(module, pages, &mut chapter.children, item.children);
                }
            }
        }

        #[derive(Default)]
        struct ItemRefBuilder {
            prefix: PagesRef,
            first: HashMap<usize, PagesRef>,
            lasts: HashMap<usize, PagesRef>,
        }

        impl ItemRefBuilder {
            fn collect_item(&mut self, item: &OutlineItem) -> OutlineItemRef {
                let pages = Rc::new(RefCell::new(Vec::new()));

                if let Some(pos) = item.position.as_ref() {
                    let page_no = pos.page_no;
                    self.first
                        .entry(page_no)
                        .or_insert_with(|| Rc::clone(&pages));
                    self.lasts.insert(page_no, Rc::clone(&pages));
                }

                OutlineItemRef {
                    item: BookMetaElem::Chapter {
                        title: crate::meta::BookMetaContent::PlainText {
                            content: item.title.clone(),
                        },
                        link: None,
                        sub: vec![],
                        section: None,
                    },
                    pages: pages.clone(),
                    children: self.collect_items(&item.children),
                }
            }

            fn collect_items(&mut self, item: &[OutlineItem]) -> Vec<OutlineItemRef> {
                item.iter()
                    .map(|item| self.collect_item(item))
                    .collect::<Vec<_>>()
            }
        }

        let mut built_outline: Option<BuiltOutline> = None;

        for l in doc.layouts.iter() {
            l.visit_pages(&mut |t| {
                let mut builder = ItemRefBuilder::default();
                let outline = LayoutRegionNode::customs(&t.0)
                    .find(|(k, _)| k.as_ref() == "outline")
                    .unwrap();
                let outline =
                    serde_json::from_slice::<crate::outline::Outline>(outline.1.as_ref()).unwrap();
                let items = builder.collect_items(&outline.items);
                builder
                    .first
                    .entry(1)
                    .or_insert_with(|| Rc::clone(&builder.prefix));
                for idx in 1..=t.1.len() {
                    if let Some(pages) = builder.first.get(&idx) {
                        pages.borrow_mut().push(idx);
                    } else if let Some(pages) = builder.lasts.get(&idx) {
                        pages.borrow_mut().push(idx);
                    }

                    if let Some(pages) = builder.lasts.get(&idx).cloned() {
                        builder.lasts.entry(idx + 1).or_insert(pages);
                    }
                }
                // println!("{:#?} of pages {:#?}", items, t.1);
                if let Some(built_outline) = built_outline.as_mut() {
                    built_outline.merge(&doc.module, builder, &t.1, items);
                } else {
                    built_outline = Some(BuiltOutline::init(&doc.module, builder, &t.1, items));
                }
            });
        }

        let built_outline = built_outline.unwrap();

        #[derive(Default)]
        struct SeparatedChapters {
            theme: String,
            content: HashMap<String, MultiVecDocument>,
        }

        impl SeparatedChapters {
            fn finalize(
                &mut self,
                origin: MultiVecDocument,
                outline: BuiltOutline,
                inferred: &mut Vec<BookMetaElem>,
            ) {
                if let Some(prefix) = outline.prefix {
                    self.content.insert(
                        format!("pre.{theme}.multi.sir.in", theme = self.theme),
                        prefix.finalize(&origin),
                    );
                    inferred.push(BookMetaElem::Chapter {
                        title: crate::meta::BookMetaContent::PlainText {
                            content: "Preface".into(),
                        },
                        link: Some("pre.typ".to_owned()),
                        sub: vec![],
                        section: None,
                    });

                    inferred.push(BookMetaElem::Separator {});
                }

                let mut numbering = vec![];
                self.finalize_items(&origin, outline.chapters, inferred, &mut numbering);
            }

            fn finalize_items(
                &mut self,
                origin: &MultiVecDocument,
                items: Vec<OutlineChapter>,
                inferred: &mut Vec<BookMetaElem>,
                numbering: &mut Vec<usize>,
            ) {
                numbering.push(0);
                for OutlineChapter {
                    mut item,
                    content,
                    children,
                } in items
                {
                    let BookMetaElem::Chapter {
                        title: _,
                        link,
                        sub,
                        section,
                    } = &mut item
                    else {
                        unreachable!();
                    };

                    if let Some(prefix) = content {
                        let link_path = format!("{}", self.content.len());
                        self.content.insert(
                            format!("{link_path}.{theme}.multi.sir.in", theme = self.theme),
                            prefix.finalize(origin),
                        );
                        *link = Some(format!("{}.typ", link_path));
                    }

                    *numbering.last_mut().unwrap() += 1;
                    self.finalize_items(origin, children, sub, numbering);
                    *section = Some(
                        numbering
                            .iter()
                            .map(|s| s.to_string())
                            .collect::<Vec<_>>()
                            .join("."),
                    );
                    inferred.push(item);
                }
                numbering.pop();
            }
        }

        let mut separated_chapters = SeparatedChapters {
            theme: theme.to_owned(),
            ..Default::default()
        };
        let mut inferred = Vec::new();
        separated_chapters.finalize(doc, built_outline, &mut inferred);

        // write multiple files to disk
        for chp in separated_chapters.content {
            let mut path = self.dest_dir.clone();
            path.push(chp.0);
            std::fs::write(path, chp.1.to_bytes()).unwrap_or_exit();
        }

        Ok(inferred)
    }

    pub fn compile_page(&mut self, path: &Path) -> Result<TypstDocument> {
        self.compile_page_with(path, CompilePageSetting::default())
    }

    fn pure_compile<D: typst::Document + Send + Sync + 'static>(
        &self,
        g: &Arc<SystemWorldComputeGraph>,
    ) -> Result<Arc<D>> {
        let _ = g.provide::<FlagTask<CompilationTask<D>>>(Ok(FlagTask::flag(true)));

        let res = g.compute::<CompilationTask<D>>()?.as_ref().clone().unwrap();
        self.report_with_warnings(res)
            .ok_or_else(|| error_once!("compile page failed"))
    }

    pub fn compile_page_with(
        &mut self,
        path: &Path,
        settings: CompilePageSetting,
    ) -> Result<TypstDocument> {
        if self.static_html && settings.with_outline {
            return Err(error_once!("outline is not supported in static paged mode"));
        }

        self.setup_entry(path);

        let inputs = TaskInputs {
            entry: None,
            inputs: Some({
                let mut dict = TypstDict::new();
                self.set_theme_target("");
                dict.insert("x-target".into(), self.compiler.target.clone().into_value());

                Arc::new(LazyHash::new(dict))
            }),
        };
        let graph = self.verse.computation_with(inputs);
        let doc = if self.static_html {
            TypstDocument::Html(self.pure_compile::<TypstHtmlDocument>(&graph)?)
        } else {
            TypstDocument::Paged(self.pure_compile::<TypstPagedDocument>(&graph)?)
        };

        for theme in THEME_LIST {
            if self.static_html {
                self.compile_html_page_with(theme)?;
            } else {
                self.compile_paged_page_with(theme, &graph, settings.clone())?;
            }
        }

        Ok(doc)
    }

    pub fn compile_html_page_with(&mut self, theme: &str) -> Result<TypstDocument> {
        let inputs = TaskInputs {
            entry: None,
            inputs: Some({
                let mut dict = TypstDict::new();
                self.set_theme_target(theme);
                dict.insert("x-target".into(), self.compiler.target.clone().into_value());

                Arc::new(LazyHash::new(dict))
            }),
        };
        let graph = self.verse.computation_with(inputs);
        let doc = self.pure_compile::<TypstHtmlDocument>(&graph)?;

        let html_doc = doc.as_ref();

        let res = self
            .report(static_html(html_doc))
            .expect("failed to render static html");

        let dest = self.module_dest_path();
        std::fs::write(&dest, res.body).unwrap_or_exit();

        Ok(TypstDocument::Html(doc.clone()))
    }

    pub fn compile_paged_page_with(
        &mut self,
        theme: &str,
        graph: &Arc<SystemWorldComputeGraph>,
        settings: CompilePageSetting,
    ) -> Result<()> {
        self.set_theme_target(theme);

        // let path = path.clone().to_owned();
        self.compiler
            .set_post_process_layout(move |_m, doc, layout| {
                // println!("post process {}", path.display());

                let LayoutRegionNode::Pages(pages) = layout else {
                    unreachable!();
                };

                let (mut meta, pages) = pages.take();

                let introspector = &doc.introspector();
                let labels = doc
                    .introspector()
                    .all()
                    .flat_map(|elem| elem.label().zip(elem.location()))
                    .map(|(label, elem)| (label.resolve().to_owned(), introspector.position(elem)))
                    .map(|(label, pos)| {
                        (
                            label,
                            format!(
                                "p{}x{:.2}y{:.2}",
                                pos.page,
                                pos.point.x.to_pt(),
                                pos.point.y.to_pt()
                            ),
                        )
                    })
                    .collect::<Vec<_>>();
                // println!("{:#?}", labels);

                let labels = serde_json::to_vec(&labels).unwrap_or_exit();
                let sema_label_meta = ("sema-label".into(), labels.into());

                let mut custom = vec![sema_label_meta];

                if settings.with_outline {
                    let mut spans = SpanInternerImpl::default();

                    let outline = crate::outline::outline(&mut spans, &doc);
                    let outline = serde_json::to_vec(&outline).unwrap_or_exit();
                    let outline_meta = ("outline".into(), outline.into());
                    custom.push(outline_meta);
                }

                meta.push(PageMetadata::Custom(custom));

                LayoutRegionNode::Pages(Arc::new((meta, pages)))
            });

        let res = DynSvgModuleExport::run(graph, &self.compiler)?;
        if let Some(doc) = res {
            let content = doc.to_bytes();
            let dest = self.module_dest_path();
            std::fs::write(&dest, content).unwrap_or_exit();
        }

        Ok(())
    }

    // todo: we should use same snapshot as that compiled documents
    pub fn generate_desc(doc: &TypstDocument) -> Result<String> {
        TextExport::run_on_doc(doc).context("export text for html description")
    }
}

struct ShiroaCommands(Option<Arc<Regex>>);

impl reflexo_typst::vector::pass::CommandExecutor for ShiroaCommands {
    fn execute(
        &self,
        cmd: typst::foundations::Bytes,
        size: Option<typst::layout::Size>,
    ) -> Option<VecItem> {
        let text = std::str::from_utf8(cmd.as_slice()).ok()?;
        // log::info!("executing svg: {}", text);

        let content = text
            .find("<!-- embedded-content")
            .and_then(|start| {
                let text = &text[start + "<!-- embedded-content".len()..];
                text.find("embedded-content -->").map(|end| &text[0..end])
            })?
            .trim();
        let (cmd, payload) = content.split_once(',')?;

        match cmd {
            "html" => {
                let args = serde_json::from_str::<HtmlCommandArgs>(payload).ok()?;

                // todo: disallow iframe?
                let allowed_tags = TAGS_META.get_or_init(|| {
                    HashMap::from_iter([
                        (
                            "iframe",
                            (
                                "",
                                HashSet::from_iter([
                                    "id",
                                    "class",
                                    "src",
                                    "allowfullscreen",
                                    "scrolling",
                                    "framespacing",
                                    "frameborder",
                                    "border",
                                    "width",
                                    "height",
                                ]),
                            ),
                        ),
                        ("div", ("", HashSet::from_iter(["id", "class"]))),
                        (
                            "audio",
                            (
                                "audio.",
                                HashSet::from_iter(["id", "class", "src", "controls"]),
                            ),
                        ),
                        (
                            "video",
                            (
                                "video.",
                                HashSet::from_iter(["id", "class", "src", "controls"]),
                            ),
                        ),
                    ])
                });

                let tag = args.tag;
                let Some((hint, allowed_attrs)) = allowed_tags.get(tag.as_str()) else {
                    log::warn!("disallowed tag: {tag}");
                    return None;
                };
                let allow_attr = |k: &str| k.starts_with("data-") || allowed_attrs.contains(k);

                let attributes = args.attributes;

                let mut attrs = String::new();
                for (k, v) in attributes {
                    if k.contains(|c: char| !c.is_ascii_alphanumeric()) || !allow_attr(&k) {
                        log::warn!("disallowed attribute: {k} on tag {tag}");
                        return None;
                    }

                    if k == "src" {
                        let Some(v) = url::Url::parse(&v).ok() else {
                            log::warn!("invalid source url: {v} on tag {tag}");
                            return None;
                        };

                        if v.scheme() != "http" && v.scheme() != "https" {
                            log::warn!("invalid source url scheme: {v} on tag {tag}");
                            return None;
                        }

                        let allowed = self
                            .0
                            .as_ref()
                            .map(|re| v.host_str().is_some_and(|host| re.is_match(host)))
                            .unwrap_or(false);
                        if !allowed {
                            log::warn!("disallowed source url: {v} on tag {tag}");
                            return None;
                        }
                    }

                    attrs.push_str(&format!(" {k}=\"{}\"", escape_str::<AttributeEscapes>(&v)));
                }

                let html = format!("<{tag}{attrs}>{hint}</{tag}>");
                return Some(VecItem::SizedRawHtml(SizedRawHtmlItem {
                    html: html.into(),
                    size: size.unwrap_or_default().into_typst(),
                }));
            }
            "ping" => {}
            _ => {}
        }

        None
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
struct HtmlCommandArgs {
    tag: String,
    attributes: HashMap<String, String>,
}

static TAGS_META: OnceLock<HashMap<&str, (&str, HashSet<&str>)>> = OnceLock::new();

fn no_foreign_obj_diag(diag: &&typst::diag::SourceDiagnostic) -> bool {
    if diag.severity == typst::diag::Severity::Error {
        return true;
    }

    !diag.message.contains("image contains foreign object")
}
