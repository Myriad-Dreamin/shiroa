pub mod meta;
pub mod outline;

use reflexo_typst::ImmutStr;
use typst::ecow::EcoString;

pub struct ChapterItem {
    pub title: EcoString,
    pub path: Option<ImmutStr>,
}
