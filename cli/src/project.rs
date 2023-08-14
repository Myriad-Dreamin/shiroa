use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::{render::TypstRenderer, CompileArgs};

/// General information about your book.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct BookConfig {
    /// The title of the book
    title: String,
    /// The author(s) of the book
    authors: Vec<String>,
    /// A description for the book, which is added as meta information in the
    /// html <head> of each page
    description: String,
    /// The github repository for the book
    repository: String,
    /// The main language of the book, which is used as a language attribute
    /// <html lang="en"> for example.
    language: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct BuildConfig {
    /// The directory to put the rendered book in. By default this is book/ in
    /// the book's root directory. This can overridden with the --dest-dir CLI
    /// option.
    #[serde(rename = "dest-dir")]
    dest_dir: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProjectConfig {
    book: BookConfig,
    build: BuildConfig,
}

pub struct Project {
    pub tr: TypstRenderer,
}

impl Project {
    pub fn new(mut args: CompileArgs) -> Self {
        let book_config: ProjectConfig = toml::from_str(
            std::fs::read_to_string(Path::new(&args.dir).join("book.toml"))
                .unwrap()
                .as_str(),
        )
        .unwrap();

        if args.workspace.is_empty() {
            args.workspace = args.dir.clone();
        }

        if args.dest_dir.is_empty() {
            args.dest_dir = book_config.build.dest_dir;
        }

        if args.dest_dir.is_empty() {
            args.dest_dir = "dist".to_owned();
        }

        let tr = TypstRenderer::new(args);

        Self { tr }
    }

    pub fn typst_renderer(self) -> TypstRenderer {
        self.tr
    }
}
