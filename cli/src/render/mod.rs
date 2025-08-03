pub mod typst;
pub use self::typst::*;
pub mod search;
pub use self::search::*;

use std::collections::BTreeMap;
pub type DataDict = BTreeMap<String, serde_json::Value>;
