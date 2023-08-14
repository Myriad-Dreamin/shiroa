use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::json;
use typst_ts_compiler::service::Compiler;

use crate::{
    render::{DataDict, HtmlRenderer, TypstRenderer},
    summary::{BookMeta, BookMetaContent, BookMetaElem},
    CompileArgs,
};

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
    pub hr: HtmlRenderer,
    pub conf: ProjectConfig,
    pub book_meta: Option<BookMeta>,

    pub dest_dir: PathBuf,
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
        let hr = HtmlRenderer::new();

        Self {
            dest_dir: tr.dest_dir.clone(),
            tr,
            hr,
            conf,
            book_meta: None,
        }
    }

    pub fn summarize(&mut self) {
        #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
        struct QueryItem {
            pub value: BookMeta,
        }

        type Json = Vec<QueryItem>;

        self.tr.setup_entry(Path::new("summary.typ"));
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

    pub fn iter_chapters(&self) -> Vec<DataDict> {
        let mut chapters = vec![];

        fn dfs_elem(elem: &BookMetaElem, chapters: &mut Vec<DataDict>) {
            // Create the data to inject in the template
            let mut chapter = DataDict::default();

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

        for item in self.book_meta.as_ref().unwrap().content.iter() {
            dfs_elem(item, &mut chapters);
        }

        chapters
    }

    pub fn compile_chapter(&mut self, _ch: DataDict, path: &str) -> Result<String, String> {
        let rel_data_path = std::path::Path::new("typst-book")
            .join(path)
            .with_extension("")
            .to_str()
            .unwrap()
            // windows
            .replace('\\', "/");

        self.tr.setup_entry(Path::new(path));

        self.tr.compiler.compile().unwrap();

        let dynamic_load_trampoline = self
            .hr
            .handlebars
            .render(
                "typst_load_trampoline",
                &json!({
                    "renderer_module" : "/typst-book/renderer/typst_ts_renderer_bg.wasm",
                    "rel_data_path": rel_data_path,
                }),
            )
            .unwrap();

        Ok(dynamic_load_trampoline.to_owned())
    }

    pub fn render_chapter(&mut self, chapter_data: DataDict, path: &str) -> String {
        let data = serde_json::to_value(self.conf.book.clone()).unwrap();
        let mut data: DataDict = serde_json::from_value(data).unwrap();

        // inject chapters
        data.insert("chapters".to_owned(), json!(self.iter_chapters()));

        // inject content
        data.insert(
            "content".to_owned(),
            serde_json::Value::String(self.compile_chapter(chapter_data, path).unwrap()),
        );

        self.hr.render_index(data, path)
    }

    // pub fn auto_order_section(&mut self) {
    //     fn dfs_elem(elem: &mut BookMetaElem, order: &mut Vec<u64>) {
    //         match elem {
    //             BookMetaElem::Chapter {
    //                 section, sub: subs, ..
    //             } => {
    //                 if section.is_none() {
    //                     *order.last_mut().unwrap() += 1;
    //                     *section = Some(format!("{}", order.last().unwrap()));
    //                 }
    //                 for sub in subs.iter_mut() {
    //                     order.push(0);
    //                     dfs_elem(sub, order);
    //                     order.pop();
    //                 }
    //             }
    //             BookMetaElem::Part { .. } | BookMetaElem::Separator {} => {}
    //         }
    //     }

    //     let mut order = vec![0];
    //     for elem in self.book_meta.content.iter_mut() {
    //         dfs_elem(elem, &mut order);
    //     }
    // }
}
