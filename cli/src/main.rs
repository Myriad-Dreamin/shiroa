use std::{path::Path, process::exit};

use clap::{Args, Command, FromArgMatches};
use typst_book_cli::{
    project::Project, utils::async_continue, BuildArgs, Opts, ServeArgs, Subcommands,
};

fn get_cli(sub_command_required: bool) -> Command {
    let cli = Command::new("$").disable_version_flag(true);
    Opts::augment_args(cli).subcommand_required(sub_command_required)
}

fn help_sub_command() -> ! {
    Opts::from_arg_matches(&get_cli(true).get_matches()).unwrap();
    exit(0)
}

fn main() {
    let opts = Opts::from_arg_matches(&get_cli(false).get_matches())
        .map_err(|err| err.exit())
        .unwrap();

    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .filter_module("typst::", log::LevelFilter::Warn)
        .filter_module("typst_library::", log::LevelFilter::Warn)
        .init();

    let sub = if let Some(sub) = opts.sub {
        sub
    } else {
        help_sub_command();
    };

    match sub {
        Subcommands::Build(args) => build(args),
        Subcommands::Serve(args) => serve(args),
    };

    #[allow(unreachable_code)]
    {
        unreachable!("The subcommand must exit the process.");
    }
}

fn build(args: BuildArgs) -> ! {
    let mut proj = Project::new(args.compile);
    proj.summarize();

    let mut write_index = false;

    for ch in proj.iter_chapters() {
        if let Some(path) = ch.get("path") {
            let raw_path: String = serde_json::from_value(path.clone()).unwrap();
            let path = &proj.dest_dir.join(&raw_path);
            let path = Path::new(&path);

            let content = proj.render_chapter(ch, &raw_path);

            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path.with_extension("html"), &content).unwrap();
            if !write_index {
                std::fs::write(&proj.dest_dir.join("index.html"), content).unwrap();
                write_index = true;
            }
        }
    }

    exit(0)
}

fn serve(args: ServeArgs) -> ! {
    async_continue(async {
        typst_book_cli::serve::serve(args).await;
        exit(0)
    })
}
