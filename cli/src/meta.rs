use std::{collections::HashMap, path::PathBuf};

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

/// Configuration for the HTML renderer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct HtmlMeta {
    /// The theme directory, if specified.
    pub theme: Option<PathBuf>,
    /// The default theme to use, defaults to 'light'
    pub default_theme: Option<String>,
    /// The theme to use if the browser requests the dark version of the site.
    /// Defaults to 'navy'.
    pub preferred_dark_theme: Option<String>,
    /// Whether to fonts.css and respective font files to the output directory.
    pub copy_fonts: bool,
    /// Additional CSS stylesheets to include in the rendered page's `<head>`.
    pub additional_css: Vec<PathBuf>,
    /// Additional JS scripts to include at the bottom of the rendered page's
    /// `<body>`.
    pub additional_js: Vec<PathBuf>,
    /// Fold settings.
    pub fold: Fold,
    /// Don't render section labels.
    pub no_section_label: bool,
    /// Search settings. If `None`, the default will be used.
    pub search: Option<Search>,
    /// Git repository url. If `None`, the git button will not be shown.
    pub git_repository_url: Option<String>,
    /// FontAwesome icon class to use for the Git repository link.
    /// Defaults to `fa-github` if `None`.
    pub git_repository_icon: Option<String>,
    /// Edit url template, when set shows a "Suggest an edit" button for
    /// directly jumping to editing the currently viewed page.
    /// Contains {path} that is replaced with chapter source file path
    pub edit_url_template: Option<String>,
    /// Input path for the 404 file, defaults to 404.md, set to "" to disable 404 file output
    pub input_404: Option<String>,
    /// Absolute url to site, used to emit correct paths for the 404 page, which might be accessed in a deeply nested directory
    pub site_url: Option<String>,
    /// The DNS subdomain or apex domain at which your book will be hosted. This
    /// string will be written to a file named CNAME in the root of your site,
    /// as required by GitHub Pages (see [*Managing a custom domain for your
    /// GitHub Pages site*][custom domain]).
    ///
    /// [custom domain]: https://docs.github.com/en/github/working-with-github-pages/managing-a-custom-domain-for-your-github-pages-site
    pub cname: Option<String>,
    /// The mapping from old pages to new pages/URLs to use when generating
    /// redirects.
    pub redirect: HashMap<String, String>,
}

impl Default for HtmlMeta {
    fn default() -> HtmlMeta {
        HtmlMeta {
            theme: None,
            default_theme: None,
            preferred_dark_theme: None,
            copy_fonts: true,
            additional_css: Vec::new(),
            additional_js: Vec::new(),
            fold: Fold::default(),
            no_section_label: false,
            search: None,
            git_repository_url: None,
            git_repository_icon: None,
            edit_url_template: None,
            input_404: None,
            site_url: None,
            cname: None,
            redirect: HashMap::new(),
        }
    }
}

/// Configuration for how to fold chapters of sidebar.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct Fold {
    /// When off, all folds are open. Default: `false`.
    pub enable: bool,
    /// The higher the more folded regions are open. When level is 0, all folds
    /// are closed.
    /// Default: `0`.
    pub level: u8,
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
