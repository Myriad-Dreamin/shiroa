use std::{
    borrow::Cow,
    cell::RefCell,
    collections::HashMap,
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
};

use crate::{
    error::prelude::*,
    meta::BookMetaElem,
    outline::{OutlineItem, SpanInternerImpl},
    utils::{make_absolute, make_absolute_from, UnwrapOrExit},
    CompileArgs,
};
use typst::diag::SourceResult;
use typst_ts_compiler::{
    service::{
        features::WITH_COMPILING_STATUS_FEATURE, CompileDriver, CompileEnv, CompileReport,
        CompileReporter, Compiler, ConsoleDiagReporter, DynamicLayoutCompiler, FeatureSet,
    },
    TypstSystemWorld,
};
use typst_ts_core::{
    config::{compiler::EntryOpts, CompileOpts},
    path::PathClean,
    vector::{
        ir::{LayoutRegionNode, Module, Page, PageMetadata},
        pass::Typst2VecPass,
    },
    TakeAs, Transformer, TypstAbs, TypstDocument,
};
use typst_ts_svg_exporter::{ir::ToItemMap, MultiVecDocument};
// serialize_doc, LayoutRegionNode,

const THEME_LIST: [&str; 5] = ["light", "rust", "coal", "navy", "ayu"];

#[derive(Debug, Clone, Default)]
pub struct CompilePageSetting {
    pub with_outline: bool,
}

pub struct TypstRenderer {
    pub status_env: Arc<FeatureSet>,
    pub compiler: CompileReporter<DynamicLayoutCompiler<CompileDriver>>,
    pub root_dir: PathBuf,
    pub dest_dir: PathBuf,
}

impl TypstRenderer {
    pub fn new(args: CompileArgs) -> Self {
        let workspace_dir = make_absolute(Path::new(&args.workspace)).clean();
        let root_dir = make_absolute(Path::new(&args.dir)).clean();
        let dest_dir = make_absolute_from(Path::new(&args.dest_dir), || root_dir.clone()).clean();

        let world = TypstSystemWorld::new(CompileOpts {
            entry: EntryOpts::new_workspace(workspace_dir.clone()),
            font_paths: args.font_paths.clone(),
            with_embedded_fonts: typst_assets::fonts().map(Cow::Borrowed).collect(),
            ..CompileOpts::default()
        })
        .unwrap_or_exit();

        let driver = CompileDriver::new(world);

        let mut driver = DynamicLayoutCompiler::new(driver, Default::default()).with_enable(true);
        driver.set_extension("multi.sir.in".to_owned());
        driver.set_layout_widths([750., 650., 550., 450., 350.].map(TypstAbs::raw).to_vec());
        let driver =
            CompileReporter::new(driver).with_generic_reporter(ConsoleDiagReporter::default());

        Self {
            status_env: Arc::new(
                FeatureSet::default().configure(&WITH_COMPILING_STATUS_FEATURE, true),
            ),
            compiler: driver,
            root_dir,
            dest_dir,
        }
    }

    fn compiler_layer_mut(&mut self) -> &mut DynamicLayoutCompiler<CompileDriver> {
        &mut self.compiler.compiler
    }

    pub fn fix_dest_dir(&mut self, path: &Path) {
        let dest_dir = make_absolute_from(path, || self.root_dir.clone()).clean();
        self.dest_dir = dest_dir;
    }

    fn set_theme_target(&mut self, theme: &str) {
        self.compiler_layer_mut().set_target(if theme.is_empty() {
            "web".to_owned()
        } else {
            format!("web-{theme}")
        });

        self.compiler_layer_mut()
            .set_extension(if theme.is_empty() {
                "multi.sir.in".to_owned()
            } else {
                format!("{theme}.multi.sir.in")
            });
    }

    fn setup_entry(&mut self, path: &Path) {
        if path.is_absolute() {
            panic!("entry file must be relative to the workspace");
        }
        let entry = self.root_dir.join(path).clean().as_path().into();
        let err = self.compiler_layer_mut().compiler.set_entry_file(entry);
        if err.is_err() {
            self.report(err);
            panic!("failed to set entry file");
        }
        let output_path = self.dest_dir.join(path).with_extension("").clean();
        std::fs::create_dir_all(output_path.parent().unwrap()).unwrap_or_exit();
        self.compiler_layer_mut().set_output(output_path);
    }

    pub fn fork_env<const REPORT_STATUS: bool>(&self) -> CompileEnv {
        let res = CompileEnv::default();
        if REPORT_STATUS {
            res.configure_shared(self.status_env.clone())
        } else {
            res
        }
    }

    pub fn report<T>(&self, may_value: SourceResult<T>) -> Option<T> {
        match may_value {
            Ok(v) => Some(v),
            Err(err) => {
                let rep =
                    CompileReport::CompileError(self.compiler.main_id(), err, Default::default());
                let rep = Arc::new((Default::default(), rep));
                // we currently ignore export error here
                let _ = self.compiler.reporter.export(self.compiler.world(), rep);
                None
            }
        }
    }

    pub fn compile_book(&mut self, path: &Path) -> ZResult<Arc<TypstDocument>> {
        self.setup_entry(path);
        self.set_theme_target("");

        let res = self.compiler.pure_compile(&mut self.fork_env::<true>());
        let res = self.report(res);

        res.ok_or_else(|| error_once!("compile book.typ"))
    }

    pub fn compile_pages_by_outline(&mut self, path: &Path) -> ZResult<Vec<BookMetaElem>> {
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

    fn compile_pages_by_outline_(&mut self, theme: &'static str) -> ZResult<Vec<BookMetaElem>> {
        // read ir from disk
        let module_output = self.compiler_layer_mut().module_dest_path();
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
                    panic!(
                        "cannot merge outline with different chapter
        count"
                    );
                }
                for (idx, item) in items.into_iter().enumerate() {
                    let chapter = &mut chapters[idx];

                    if chapter.item != item.item {
                        panic!(
                            "cannot merge outline with different
        chapter"
                        );
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

    pub fn compile_page(&mut self, path: &Path) -> ZResult<Arc<TypstDocument>> {
        self.compile_page_with(path, CompilePageSetting::default())
    }

    pub fn compile_page_with(
        &mut self,
        path: &Path,
        settings: CompilePageSetting,
    ) -> ZResult<Arc<TypstDocument>> {
        self.setup_entry(path);

        let mut any_doc = None;

        for theme in THEME_LIST {
            self.set_theme_target(theme);

            // let path = path.clone().to_owned();
            self.compiler_layer_mut()
                .set_post_process_layout(move |_m, doc, layout| {
                    // println!("post process {}", path.display());

                    let LayoutRegionNode::Pages(pages) = layout else {
                        unreachable!();
                    };

                    let (mut meta, pages) = pages.take();

                    let introspector = &doc.introspector;
                    let labels = doc
                        .introspector
                        .all()
                        .flat_map(|elem| elem.label().zip(elem.location()))
                        .map(|(label, elem)| {
                            (
                                label.clone().as_str().to_owned(),
                                introspector.position(elem),
                            )
                        })
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

            let res = self.compiler.compile(&mut self.fork_env::<true>());
            let doc = self
                .report(res)
                .ok_or_else(|| error_once!("compile page theme", theme: theme))?;
            any_doc = Some(doc.clone());
        }

        any_doc.ok_or_else(|| error_once!("compile page.typ"))
    }

    pub fn generate_desc(&mut self, doc: &TypstDocument) -> ZResult<String> {
        let e = typst_ts_text_exporter::TextExporter::default();
        let mut w = std::io::Cursor::new(Vec::new());
        e.export(self.compiler.world(), (Arc::new(doc.clone()), &mut w))
            .map_err(|e| error_once!("export text", error: format!("{e:?}")))?;

        let w = w.into_inner();

        String::from_utf8(w).map_err(|e| error_once!("export text", error: format!("{e:?}")))
    }
}
