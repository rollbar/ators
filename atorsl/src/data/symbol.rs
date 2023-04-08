use derive_builder::Builder;
use std::path::PathBuf;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Builder)]
pub struct Symbol {
    pub module: String,
    pub linkage: String,

    pub lang: gimli::DwLang,

    pub file: PathBuf,
    pub line: u16,
    pub col: Option<u16>,
}
