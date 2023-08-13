use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind")]
pub enum BookMetaContent {
    #[serde(rename = "plain-text")]
    PlainText { content: String },
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind")]
pub enum BookMetaElement {
    #[serde(rename = "book")]
    Book { content: Vec<BookMetaElement> },
    #[serde(rename = "part")]
    Part { title: BookMetaContent, level: i32 },
    #[serde(rename = "chapter")]
    Chapter {
        title: BookMetaContent,
        link: String,
        #[serde(default)]
        subs: Vec<BookMetaElement>,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct QueryBookMetaJsonResultItem {
    pub value: BookMetaElement,
}

pub type QueryBookMetaJsonResults = Vec<QueryBookMetaJsonResultItem>;
