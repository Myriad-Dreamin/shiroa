use std::process::exit;

use clap::{Args, Command, FromArgMatches};
use shiroa::{
    args::{BuildArgs, InitArgs, Opts, ServeArgs, Subcommands},
    commands,
    error::prelude::*,
    project::Project,
    utils::{async_continue, UnwrapOrExit},
    version::intercept_version,
};

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

    if opts.verbose {
        env_logger::builder()
            .filter_level(log::LevelFilter::Info)
            .filter_module("typst", log::LevelFilter::Warn)
            .filter_module("reflexo", log::LevelFilter::Info)
            .filter_module("tracing::", log::LevelFilter::Off)
            .init();
    } else {
        env_logger::builder()
            .filter_level(log::LevelFilter::Warn)
            .init();
    }

    intercept_version(opts.version, opts.vv);

    match opts.sub {
        Some(Subcommands::Init(mut args)) => {
            args.compile.compat();
            async_continue(async { init(args).await.unwrap_or_exit() })
        }
        Some(Subcommands::Build(mut args)) => {
            args.compile.compat();
            build(args).unwrap_or_exit()
        }
        Some(Subcommands::Serve(mut args)) => {
            args.compile.compat();
            async_continue(async { serve(args).await.unwrap_or_exit() })
        }
        None => help_sub_command(),
    };

    #[allow(unreachable_code)]
    {
        unreachable!("The subcommand must exit the process.");
    }
}

async fn init(args: InitArgs) -> Result<()> {
    commands::init(&args)?;

    serve(ServeArgs {
        compile: args.compile,
        addr: "127.0.0.1:25520".to_owned(),
        ..Default::default()
    })
    .await?;

    exit(0)
}

fn build(args: BuildArgs) -> Result<()> {
    let mut proj = Project::new(args.compile)?;
    proj.build()?;

    exit(0)
}

async fn serve(args: ServeArgs) -> Result<()> {
    commands::serve(args).await?;
    exit(0);
}
