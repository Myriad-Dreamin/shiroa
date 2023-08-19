use std::{net::SocketAddr, process::exit};

use clap::{Args, Command, FromArgMatches};
use typst_book_cli::{
    error::prelude::*,
    project::Project,
    utils::{async_continue, UnwrapOrExit},
    BuildArgs, Opts, ServeArgs, Subcommands,
};
use warp::{http::Method, Filter};

fn get_cli(sub_command_required: bool) -> Command {
    let cli = Command::new("$").disable_version_flag(true);
    Opts::augment_args(cli).subcommand_required(sub_command_required)
}

fn help_sub_command() -> ! {
    Opts::from_arg_matches(&get_cli(true).get_matches()).unwrap_or_exit();
    exit(0);
}

fn main() {
    let opts = Opts::from_arg_matches(&get_cli(false).get_matches()).unwrap_or_exit();

    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .filter_module("typst::", log::LevelFilter::Warn)
        .filter_module("typst_library::", log::LevelFilter::Warn)
        .init();

    match opts.sub {
        Some(Subcommands::Build(args)) => build(args).unwrap_or_exit(),
        Some(Subcommands::Serve(args)) => {
            async_continue(async { serve(args).await.unwrap_or_exit() })
        }
        None => help_sub_command(),
    };

    #[allow(unreachable_code)]
    {
        unreachable!("The subcommand must exit the process.");
    }
}

fn build(args: BuildArgs) -> ZResult<()> {
    let mut proj = Project::new(args.compile)?;
    proj.build()?;

    exit(0)
}

pub async fn serve(args: ServeArgs) -> ZResult<()> {
    let mut proj = Project::new(args.compile)?;

    // Build the book if it hasn't been built yet
    if !args.no_build {
        proj.build()?;
    }

    let http_addr: SocketAddr = args
        .addr
        .clone()
        .parse()
        .map_err(map_string_err("ParseServeAddr"))?;

    let server = warp::serve({
        let cors =
            warp::cors().allow_methods(&[Method::GET, Method::POST, Method::DELETE, Method::HEAD]);

        warp::fs::dir(proj.dest_dir)
            .with(cors)
            .with(warp::compression::gzip())
    });

    server.run(http_addr).await;

    exit(0);
}
