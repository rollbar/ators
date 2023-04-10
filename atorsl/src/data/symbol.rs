use super::Addr;
use derive_builder::Builder;
use itertools::Either;
use std::path::PathBuf;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct SourceLoc {
    pub file: PathBuf,
    pub line: u16,
    pub col: Option<u16>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Builder)]
pub struct Symbol {
    pub module: String,
    pub name: String,
    pub lang: gimli::DwLang,
    pub loc: Either<SourceLoc, Addr>,
}
