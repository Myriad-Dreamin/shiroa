use std::{path::Path, process::exit};

use clap::{Args, Command, FromArgMatches};
use reflexo_typst::path::{unix_slash, PathClean};
use shiroa::{
    error::prelude::*,
    project::Project,
    utils::{async_continue, create_dirs, make_absolute, write_file, UnwrapOrExit},
    version::intercept_version,
    BuildArgs, InitArgs, Opts, ServeArgs, Subcommands,
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
        Some(Subcommands::Init(args)) => {
            async_continue(async { init(args).await.unwrap_or_exit() })
        }
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

async fn init(args: InitArgs) -> Result<()> {
    let dir = make_absolute(Path::new(&args.compile.dir)).clean();

    if dir.exists() {
        clap::Error::raw(
            clap::error::ErrorKind::ValueValidation,
            format!("the init directory already exists: {dir:?}\n"),
        )
        .exit()
    }

    let wd = if args.compile.workspace.is_empty() {
        dir.clone()
    } else {
        make_absolute(Path::new(&args.compile.workspace)).clean()
    };
    let rel = pathdiff::diff_paths(&dir, &wd).unwrap();

    if rel.starts_with("..") {
        clap::Error::raw(
            clap::error::ErrorKind::ValueValidation,
            format!("bad workspace, which is sub-directory of book.typ's root: {dir:?} / {wd:?} -> {rel:?}"),
        )
        .exit()
    }

    let workspace_to_root = Path::new("/").join(rel);

    let page_template = unix_slash(&workspace_to_root.join("templates/page.typ"));
    let ebook_template = unix_slash(&workspace_to_root.join("templates/ebook.typ"));
    let book_typ = unix_slash(&workspace_to_root.join("book.typ"));

    let build_meta = if args.compile.dest_dir.is_empty() {
        String::default()
    } else {
        format!(
            r##"#build-meta(
  dest-dir: "{}",
)"##,
            args.compile.dest_dir
        )
    };

    create_dirs(&dir)?;
    create_dirs(dir.join("templates"))?;

    write_file(
        dir.join("book.typ"),
        format!(
            r##"
#import "@preview/shiroa:0.2.3": *

#show: book

#book-meta(
  title: "shiroa",
  summary: [
    #prefix-chapter("sample-page.typ")[Hello, typst]
  ]
)

{build_meta}

// re-export page template
#import "{page_template}": project
#let book-page = project
"##
        ),
    )?;
    write_file(
        dir.join("sample-page.typ"),
        format!(
            r##"#import "{book_typ}": book-page

#show: book-page.with(title: "Hello, typst")

= Hello, typst

Sample page
"##
        ),
    )?;
    write_file(
        dir.join("ebook.typ"),
        format!(
            r##"#import "@preview/shiroa:0.2.3": *

#import "{ebook_template}"

#show: ebook.project.with(title: "typst-book", spec: "book.typ")

// set a resolver for inclusion
#ebook.resolve-inclusion(it => include it)
"##
        ),
    )?;
    write_file(
        dir.join("templates/page.typ"),
        include_str!("../../contrib/typst/gh-pages.typ")
            .replace(
                r#""/contrib/typst/gh-pages.typ""#,
                &format!("{page_template:?}"),
            )
            .replace(r#""/github-pages/docs/book.typ""#, &format!("{book_typ:?}")),
    )?;
    write_file(
        dir.join("templates/ebook.typ"),
        include_str!("../../contrib/typst/gh-ebook.typ")
            .replace(r#""/github-pages/docs/book.typ""#, &format!("{book_typ:?}")),
    )?;
    write_file(
        dir.join("templates/theme-style.toml"),
        include_bytes!("../../contrib/typst/theme-style.toml"),
    )?;
    write_file(
        dir.join("templates/tokyo-night.tmTheme"),
        include_bytes!("../../contrib/typst/tokyo-night.tmTheme"),
    )?;

    serve(ServeArgs {
        compile: args.compile,
        addr: "127.0.0.1:25520".to_owned(),
        ..Default::default()
    })
    .await
}

fn build(args: BuildArgs) -> Result<()> {
    let mut proj = Project::new(args.compile)?;
    proj.build()?;

    exit(0)
}

pub async fn serve(args: ServeArgs) -> Result<()> {
    shiroa::serve(args).await?;
    exit(0);
}
