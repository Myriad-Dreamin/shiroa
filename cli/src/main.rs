use std::{net::SocketAddr, path::Path, process::exit};

use clap::{Args, Command, FromArgMatches};
use include_dir::include_dir;
use typst_book_cli::{
    project::Project,
    utils::{async_continue, copy_dir_embedded},
    BuildArgs, Opts, ServeArgs, Subcommands,
};
use warp::Filter;

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
    proj.compile_meta();

    let mut write_index = false;

    std::fs::create_dir_all(&proj.dest_dir).unwrap();
    copy_dir_embedded(
        include_dir!("$CARGO_MANIFEST_DIR/../themes/mdbook/css"),
        proj.dest_dir.join("css"),
    )
    .unwrap();
    copy_dir_embedded(
        include_dir!("$CARGO_MANIFEST_DIR/../themes/mdbook/fontAwesome"),
        proj.dest_dir.join("fontAwesome"),
    )
    .unwrap();

    // todo use themes in filesystem
    // copy_dir_all("themes/mdbook/css", proj.dest_dir.join("css")).unwrap();
    // copy_dir_all(
    //     "themes/mdbook/fontAwesome",
    //     proj.dest_dir.join("fontAwesome"),
    // )
    // .unwrap();

    // copy files
    std::fs::create_dir_all(&proj.dest_dir.join("renderer")).unwrap();
    std::fs::write(
        proj.dest_dir.join("renderer/typst_ts_renderer_bg.wasm"),
        include_bytes!(
            "../../frontend/node_modules/@myriaddreamin/typst-ts-renderer/typst_ts_renderer_bg.wasm"
        ),
    )
    .unwrap();
    std::fs::write(
        proj.dest_dir.join("typst-main.js"),
        include_bytes!("../../frontend/node_modules/@myriaddreamin/typst.ts/dist/main.js"),
    )
    .unwrap();
    std::fs::write(
        proj.dest_dir.join("svg_utils.js"),
        include_bytes!("../../frontend/src/svg_utils.cjs"),
    )
    .unwrap();
    std::fs::write(
        proj.dest_dir.join("typst-book.js"),
        include_bytes!("../../frontend/dist/main.js"),
    )
    .unwrap();

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
    pub async fn serve_inner(args: ServeArgs) {
        use warp::http::Method;

        let proj = Project::new(args.compile);

        let http_addr: SocketAddr = args.addr.clone().parse().unwrap();

        let cors =
            warp::cors().allow_methods(&[Method::GET, Method::POST, Method::DELETE, Method::HEAD]);

        let routes = warp::fs::dir(proj.dest_dir)
            .with(cors)
            .with(warp::compression::gzip());

        let server = warp::serve(routes);

        server.run(http_addr).await
    }

    async_continue(async {
        serve_inner(args).await;
        exit(0)
    })
}
