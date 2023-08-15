use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind")]
pub enum BookMetaContent {
    #[serde(rename = "plain-text")]
    PlainText { content: String },
}

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

/// Book metadata in summary.typ
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct BookMeta {
    /// A list of all the content in the book
    pub summary: Vec<BookMetaElem>,
}
