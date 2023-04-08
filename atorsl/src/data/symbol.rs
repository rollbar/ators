use super::SourceLoc;
use derive_builder::Builder;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Builder)]
pub struct Symbol {
    pub module: String,
    pub linkage: String,
    pub lang: gimli::DwLang,

    pub loc: SourceLoc,
}
