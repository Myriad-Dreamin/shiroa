use std::path::Path;

use serde::{Deserialize, Serialize};
use typst_ts_compiler::service::Compiler;

use crate::{render::TypstRenderer, summary::BookMeta, CompileArgs};

/// General information about your book.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct BookConfig {
    /// The title of the book
    pub title: String,
    /// The author(s) of the book
    pub authors: Vec<String>,
    /// A description for the book, which is added as meta information in the
    /// html <head> of each page
    pub description: String,
    /// The github repository for the book
    pub repository: String,
    /// The main language of the book, which is used as a language attribute
    /// <html lang="en"> for example.
    pub language: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct BuildConfig {
    /// The directory to put the rendered book in. By default this is book/ in
    /// the book's root directory. This can overridden with the --dest-dir CLI
    /// option.
    #[serde(rename = "dest-dir")]
    pub dest_dir: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProjectConfig {
    pub book: BookConfig,
    pub build: BuildConfig,
}

pub struct Project {
    pub tr: TypstRenderer,
    pub conf: ProjectConfig,
    pub book_meta: Option<BookMeta>,
}

impl Project {
    pub fn new(mut args: CompileArgs) -> Self {
        let conf: ProjectConfig = toml::from_str(
            std::fs::read_to_string(Path::new(&args.dir).join("book.toml"))
                .unwrap()
                .as_str(),
        )
        .unwrap();

        if args.workspace.is_empty() {
            args.workspace = args.dir.clone();
        }

        if args.dest_dir.is_empty() {
            args.dest_dir = conf.build.dest_dir.clone();
        }

        if args.dest_dir.is_empty() {
            args.dest_dir = "dist".to_owned();
        }

        let tr = TypstRenderer::new(args);

        Self {
            tr,
            conf,
            book_meta: None,
        }
    }

    pub fn typst_renderer(&self) -> &TypstRenderer {
        &self.tr
    }

    pub fn summarize(&mut self) {
        #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
        struct QueryItem {
            pub value: BookMeta,
        }

        type Json = Vec<QueryItem>;

        self.tr.set_entry_file(Path::new("summary.typ"));
        let doc = self.tr.compiler.pure_compile().unwrap();
        let res = self
            .tr
            .compiler
            .query("<typst-book-book-meta>".to_string(), &doc)
            .unwrap();

        // convert to native struct
        let res = serde_json::to_value(&res).unwrap();
        let res: Json = serde_json::from_value(res).unwrap();

        println!("metadata: {:?}", res);

        assert!(res.len() == 1);

        let book_meta = res.first().unwrap().value.clone();

        self.book_meta = Some(book_meta);
    }
}
