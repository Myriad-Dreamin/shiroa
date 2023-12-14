use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::{
    error::prelude::*,
    font::EMBEDDED_FONT,
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
use typst_ts_core::{config::CompileOpts, path::PathClean, TakeAs, TypstAbs, TypstDocument};
use typst_ts_svg_exporter::flat_ir::{LayoutRegionNode, PageMetadata};

const THEME_LIST: [&str; 5] = ["light", "rust", "coal", "navy", "ayu"];

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
            root_dir: workspace_dir.clone(),
            font_paths: args.font_paths.clone(),
            with_embedded_fonts: EMBEDDED_FONT.to_owned(),
            ..CompileOpts::default()
        })
        .unwrap_or_exit();

        let driver = CompileDriver {
            world,
            entry_file: Default::default(),
        };

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
        self.compiler_layer_mut().compiler.entry_file = self.root_dir.join(path).clean();
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

        self.compiler
            .pure_compile(&mut self.fork_env::<true>())
            .map_err(|_| error_once!("compile book.typ"))
    }

    pub fn compile_page(&mut self, path: &Path) -> ZResult<()> {
        self.setup_entry(path);

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

                    meta.push(PageMetadata::Custom(vec![(
                        "sema-label".into(),
                        labels.into(),
                    )]));

                    LayoutRegionNode::Pages(Arc::new((meta, pages)))
                });

            self.compiler
                .compile(&mut self.fork_env::<true>())
                .map_err(|_| error_once!("compile page theme", theme: theme))?;
        }

        Ok(())
    }
}
