use std::path::Path;

use typst_ts_compiler::{service::CompileDriver, TypstSystemWorld};
use typst_ts_core::{config::CompileOpts, exporter_builtins::GroupExporter, path::PathClean};

use crate::{utils::UnwrapOrExit, CompileArgs};

pub fn create_driver(args: CompileArgs) -> CompileDriver {
    let workspace_dir = Path::new(args.workspace.as_str()).clean();
    // todo: toml config
    let entry_file_path = Path::new("github-pages/docs/summary.typ").clean();
    // let entry_file_path = Path::new(args.entry.as_str()).clean();

    let workspace_dir = if workspace_dir.is_absolute() {
        workspace_dir
    } else {
        let cwd = std::env::current_dir().unwrap_or_exit();
        cwd.join(workspace_dir)
    };

    let entry_file_path = if entry_file_path.is_absolute() {
        entry_file_path
    } else {
        let cwd = std::env::current_dir().unwrap_or_exit();
        cwd.join(entry_file_path)
    };

    if !entry_file_path.starts_with(&workspace_dir) {
        clap::Error::raw(
            clap::error::ErrorKind::InvalidValue,
            format!(
                "entry file path must be in workspace directory: {workspace_dir}\n",
                workspace_dir = workspace_dir.display()
            ),
        )
        .exit()
    }

    let world = TypstSystemWorld::new(CompileOpts {
        root_dir: workspace_dir.clone(),
        font_paths: args.font_paths.clone(),
        //  with_embedded_fonts: EMBEDDED_FONT.to_owned(),
        ..CompileOpts::default()
    })
    .unwrap_or_exit();

    CompileDriver {
        world,
        entry_file: entry_file_path.to_owned(),
        exporter: GroupExporter::new(vec![]),
    }
}
