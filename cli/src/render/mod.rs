pub mod html;
use std::collections::BTreeMap;

pub use self::html::*;

pub mod typst;
pub use self::typst::*;

pub type DataDict = BTreeMap<String, serde_json::Value>;
