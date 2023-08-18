use std::path::{Path, PathBuf};

use typst_ts_compiler::{
    service::{CompileDriver, DynamicLayoutCompiler},
    TypstSystemWorld,
};
use typst_ts_core::{config::CompileOpts, path::PathClean, TypstAbs};

use crate::{
    font::EMBEDDED_FONT,
    utils::{make_absolute, make_absolute_from, UnwrapOrExit},
    CompileArgs,
};

pub struct TypstRenderer {
    pub compiler: DynamicLayoutCompiler<CompileDriver>,
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

        Self {
            compiler: driver,
            root_dir,
            dest_dir,
        }
    }

    pub fn fix_dest_dir(&mut self, path: &Path) {
        let dest_dir = make_absolute_from(path, || self.root_dir.clone()).clean();
        self.dest_dir = dest_dir;
    }

    pub fn setup_entry(&mut self, path: &Path) {
        if path.is_absolute() {
            panic!("entry file must be relative to the workspace");
        }
        self.compiler.compiler.entry_file = self.root_dir.join(path).clean();
        let output_path = self.dest_dir.join(path).with_extension("").clean();
        std::fs::create_dir_all(output_path.parent().unwrap()).unwrap_or_exit();
        self.compiler.set_output(output_path);
    }
}
