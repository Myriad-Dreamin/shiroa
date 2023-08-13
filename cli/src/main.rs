use std::process::exit;

use clap::{Args, Command, FromArgMatches};
use typst_book_cli::{
    compile::create_driver, summary::QueryBookMetaJsonResults, utils::async_continue, BuildArgs,
    Opts, ServeArgs, Subcommands,
};

fn get_cli(sub_command_required: bool) -> Command {
    let cli = Command::new("$").disable_version_flag(true);
    Opts::augment_args(cli).subcommand_required(sub_command_required)
}

fn help_sub_command() {
    Opts::from_arg_matches(&get_cli(true).get_matches()).unwrap();
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

    match opts.sub {
        Some(Subcommands::Build(args)) => build(args),
        Some(Subcommands::Serve(args)) => serve(args),
        None => help_sub_command(),
    };

    #[allow(unreachable_code)]
    {
        unreachable!("The subcommand must exit the process.");
    }
}

fn build(args: BuildArgs) -> ! {
    let mut driver = create_driver(args.compile);

    let doc = driver.compile().unwrap();
    let res = driver
        .query("<typst-book-book-meta>".to_string(), &doc)
        .unwrap();
    let res = serde_json::to_value(&res).unwrap();
    let res: QueryBookMetaJsonResults = serde_json::from_value(res).unwrap();

    println!("metadata: {:?}", res);

    assert!(res.len() == 1);

    let book_config = toml::from_str(
        std::fs::read_to_string("github-pages/docs/book.toml")
            .unwrap()
            .as_str(),
    )
    .unwrap();

    let renderer =
        typst_book_cli::render::Renderer::new(book_config, res.first().unwrap().value.clone());

    std::fs::create_dir_all("github-pages/dist/").unwrap();

    std::fs::write("github-pages/dist/index.html", renderer.html_render()).unwrap();

    exit(0)
}

fn serve(args: ServeArgs) -> ! {
    async_continue(async {
        typst_book_cli::serve::serve(args).await;
        exit(0)
    })
}
