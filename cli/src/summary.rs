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
    #[serde(rename = "part")]
    Part { title: BookMetaContent, level: i32 },
    #[serde(rename = "chapter")]
    Chapter {
        title: BookMetaContent,
        link: Option<String>,
        #[serde(default)]
        sub: Vec<BookMetaElement>,
        section: Option<String>,
    },
    #[serde(rename = "separator")]
    Separator {},
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct BookMetaWrapper {
    pub content: Vec<BookMetaElement>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct QueryBookMetaJsonResultItem {
    pub value: BookMetaWrapper,
}

pub type QueryBookMetaJsonResults = Vec<QueryBookMetaJsonResultItem>;
