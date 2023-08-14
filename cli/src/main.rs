use std::{collections::BTreeMap, path::Path, process::exit};

use clap::{Args, Command, FromArgMatches};
use serde_json::json;
use typst_book_cli::{
    project::Project,
    summary::{BookMeta, BookMetaContent, BookMetaElem},
    utils::async_continue,
    BuildArgs, Opts, ServeArgs, Subcommands,
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

    let book_meta = proj.book_meta.unwrap();
    let Project {
        tr: driver,
        conf: book_config,
        ..
    } = proj;
    let mut renderer =
        typst_book_cli::render::HtmlRenderer::new(book_config, driver.compiler, book_meta.clone());

    pub fn convert_chapters(book_meta: &BookMeta) -> Vec<BTreeMap<String, serde_json::Value>> {
        let mut chapters = vec![];

        fn dfs_elem(elem: &BookMetaElem, chapters: &mut Vec<BTreeMap<String, serde_json::Value>>) {
            // Create the data to inject in the template
            let mut chapter = BTreeMap::new();

            match elem {
                BookMetaElem::Part { title, .. } => {
                    let BookMetaContent::PlainText { content: title } = title;

                    chapter.insert("part".to_owned(), json!(title));
                }
                BookMetaElem::Chapter {
                    title,
                    section,
                    link,
                    sub: subs,
                } => {
                    let BookMetaContent::PlainText { content: title } = title;

                    if let Some(ref section) = section {
                        chapter.insert("section".to_owned(), json!(section));
                    }

                    chapter.insert(
                        "has_sub_items".to_owned(),
                        json!((!subs.is_empty()).to_string()),
                    );

                    chapter.insert("name".to_owned(), json!(title));
                    if let Some(ref path) = link {
                        chapter.insert("path".to_owned(), json!(path));
                    }
                }
                BookMetaElem::Separator {} => {
                    chapter.insert("spacer".to_owned(), json!("_spacer_"));
                }
            }

            chapters.push(chapter);

            if let BookMetaElem::Chapter { sub: subs, .. } = elem {
                for child in subs.iter() {
                    dfs_elem(child, chapters);
                }
            }
        }

        for item in book_meta.content.iter() {
            dfs_elem(item, &mut chapters);
        }

        chapters
    }

    std::fs::create_dir_all(&driver.dest_dir).unwrap();

    let chapters = convert_chapters(&book_meta);

    let mut write_index = false;

    for ch in chapters {
        if let Some(path) = ch.get("path") {
            let raw_path: String = serde_json::from_value(path.clone()).unwrap();
            let path = &driver.dest_dir.join(&raw_path);
            let path = Path::new(&path);

            let content = renderer.html_render(ch, raw_path);

            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path.with_extension("html"), &content).unwrap();
            if !write_index {
                std::fs::write(&driver.dest_dir.join("index.html"), content).unwrap();
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
