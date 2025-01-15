use serde::{Deserialize, Serialize};

/// Typst content kind embedded in metadata nodes
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind")]
pub enum BookMetaContent {
    #[serde(rename = "raw")]
    Raw { content: serde_json::Value },
    #[serde(rename = "plain-text")]
    PlainText { content: String },
}

/// Content summary kind in summary.typ
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind")]
pub enum BookMetaElem {
    #[serde(rename = "part")]
    Part { title: BookMetaContent, level: i32 },
    #[serde(rename = "chapter")]
    Chapter {
        title: BookMetaContent,
        link: Option<String>,
        #[serde(default)]
        sub: Vec<BookMetaElem>,
        section: Option<String>,
    },
    #[serde(rename = "separator")]
    Separator {},
}

/// General information about your book.
/// Book metadata in summary.typ
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct BookMeta {
    /// The title of the book
    pub title: String,
    /// The author(s) of the book
    pub authors: Vec<String>,
    /// A description for the book, which is added as meta information in the
    /// html `<head>` of each page
    pub description: String,
    /// The github repository for the book
    pub repository: String,
    /// The github repository editing template for the book
    /// example: `https://github.com/Me/Book/edit/main/path/to/book/{path}`
    pub repository_edit: String,
    /// The main language of the book, which is used as a language attribute
    /// `<html lang="en">` for example.
    pub language: String,
    /// Content summary of the book
    pub summary: Vec<BookMetaElem>,
}

/// Build metadata in summary.typ
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct BuildMeta {
    /// The directory to put the rendered book in. By default this is book/ in
    /// the book's root directory. This can overridden with the --dest-dir CLI
    /// option.
    #[serde(rename = "dest-dir")]
    pub dest_dir: String,
}

/// Configuration of the search functionality of the HTML renderer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct Search {
    /// Enable the search feature. Default: `true`.
    pub enable: bool,
    /// Maximum number of visible results. Default: `30`.
    pub limit_results: u32,
    /// The number of words used for a search result teaser. Default: `30`.
    pub teaser_word_count: u32,
    /// Define the logical link between multiple search words.
    /// If true, all search words must appear in each result. Default: `false`.
    pub use_boolean_and: bool,
    /// Boost factor for the search result score if a search word appears in the
    /// header. Default: `2`.
    pub boost_title: u8,
    /// Boost factor for the search result score if a search word appears in the
    /// hierarchy. The hierarchy contains all titles of the parent documents
    /// and all parent headings. Default: `1`.
    pub boost_hierarchy: u8,
    /// Boost factor for the search result score if a search word appears in the
    /// text. Default: `1`.
    pub boost_paragraph: u8,
    /// True if the searchword `micro` should match `microwave`. Default:
    /// `true`.
    pub expand: bool,
    /// Documents are split into smaller parts, separated by headings. This
    /// defines, until which level of heading documents should be split.
    /// Default: `3`. (`### This is a level 3 heading`)
    pub heading_split_level: u8,
    /// Copy JavaScript files for the search functionality to the output
    /// directory? Default: `true`.
    pub copy_js: bool,
}

impl Default for Search {
    fn default() -> Search {
        // Please update the documentation of `Search` when changing values!
        Search {
            enable: true,
            limit_results: 30,
            teaser_word_count: 30,
            use_boolean_and: false,
            boost_title: 2,
            boost_hierarchy: 1,
            boost_paragraph: 1,
            expand: true,
            heading_split_level: 3,
            copy_js: true,
        }
    }
}
