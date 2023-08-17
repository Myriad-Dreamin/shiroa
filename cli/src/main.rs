use std::{net::SocketAddr, path::Path, process::exit};

use clap::{Args, Command, FromArgMatches};
use include_dir::include_dir;
use typst_book_cli::{
    error::prelude::*,
    project::Project,
    utils::{async_continue, copy_dir_embedded, create_dirs, write_file, UnwrapOrExit},
    BuildArgs, Opts, ServeArgs, Subcommands,
};
use warp::Filter;

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

    let sub = if let Some(sub) = opts.sub {
        sub
    } else {
        help_sub_command();
    };

    match sub {
        Subcommands::Build(args) => build(args).unwrap_or_exit(),
        Subcommands::Serve(args) => async_continue(async { serve(args).await.unwrap_or_exit() }),
    };

    #[allow(unreachable_code)]
    {
        unreachable!("The subcommand must exit the process.");
    }
}

fn build(args: BuildArgs) -> ZResult<()> {
    let mut proj = Project::new(args.compile);

    let mut write_index = false;

    create_dirs(&proj.dest_dir)?;
    copy_dir_embedded(
        include_dir!("$CARGO_MANIFEST_DIR/../themes/mdbook/css"),
        proj.dest_dir.join("css"),
    )?;
    copy_dir_embedded(
        include_dir!("$CARGO_MANIFEST_DIR/../themes/mdbook/FontAwesome/css"),
        proj.dest_dir.join("FontAwesome/css"),
    )?;
    copy_dir_embedded(
        include_dir!("$CARGO_MANIFEST_DIR/../themes/mdbook/FontAwesome/fonts"),
        proj.dest_dir.join("FontAwesome/fonts"),
    )?;

    // todo use themes in filesystem
    // copy_dir_all("themes/mdbook/css", proj.dest_dir.join("css")).unwrap();
    // copy_dir_all(
    //     "themes/mdbook/fontAwesome",
    //     proj.dest_dir.join("fontAwesome"),
    // )
    // .unwrap();

    // copy files
    create_dirs(&proj.dest_dir.join("renderer"))?;
    write_file(
        proj.dest_dir.join("renderer/typst_ts_renderer_bg.wasm"),
        include_bytes!(
            "../../frontend/node_modules/@myriaddreamin/typst-ts-renderer/typst_ts_renderer_bg.wasm"
        ),
    )?;
    write_file(
        proj.dest_dir.join("typst-main.js"),
        include_bytes!("../../frontend/node_modules/@myriaddreamin/typst.ts/dist/main.js"),
    )?;
    write_file(
        proj.dest_dir.join("svg_utils.js"),
        include_bytes!("../../frontend/src/svg_utils.cjs"),
    )?;
    write_file(
        proj.dest_dir.join("typst-book.js"),
        include_bytes!("../../frontend/dist/main.js"),
    )?;

    for ch in proj.iter_chapters() {
        if let Some(path) = ch.get("path") {
            let raw_path: String = serde_json::from_value(path.clone()).map_err(|err| {
                error_once_map!("retrieve path in book.toml", value: path)(err.to_string())
            })?;
            let path = &proj.dest_dir.join(&raw_path);
            let path = Path::new(&path);

            let content = proj.render_chapter(ch, &raw_path);

            create_dirs(path.parent().unwrap())?;
            write_file(path.with_extension("html"), &content)?;
            if !write_index {
                write_file(&proj.dest_dir.join("index.html"), content)?;
                write_index = true;
            }
        }
    }

    exit(0)
}

pub async fn serve(args: ServeArgs) -> ZResult<()> {
    use warp::http::Method;

    let proj = Project::new(args.compile);

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
