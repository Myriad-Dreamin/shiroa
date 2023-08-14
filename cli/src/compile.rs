use std::path::Path;

use typst_ts_compiler::{service::CompileDriver, TypstSystemWorld};
use typst_ts_core::{config::CompileOpts, path::PathClean};

use crate::{
    utils::{make_absolute, UnwrapOrExit},
    CompileArgs,
};

pub fn create_driver(args: CompileArgs) -> CompileDriver {
    let workspace_dir = make_absolute(Path::new(&args.workspace)).clean();
    let root_dir = make_absolute(Path::new(&args.dir)).clean();
    let summary_path = root_dir.join("summary.typ").clean();

    let world = TypstSystemWorld::new(CompileOpts {
        root_dir: workspace_dir.clone(),
        font_paths: args.font_paths.clone(),
        //  with_embedded_fonts: EMBEDDED_FONT.to_owned(),
        ..CompileOpts::default()
    })
    .unwrap_or_exit();

    CompileDriver {
        world,
        entry_file: summary_path.to_owned(),
    }
}
