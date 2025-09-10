pub mod typst;
use ::typst::ecow::EcoString;
use reflexo_typst::ImmutStr;

pub use self::typst::*;
pub mod search;
pub use self::search::*;

pub struct ChapterItem {
    pub title: EcoString,
    pub path: Option<ImmutStr>,
}
