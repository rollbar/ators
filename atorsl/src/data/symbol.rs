use super::Addr;
use itertools::Either;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceLoc {
    pub file: PathBuf,
    pub line: u16,
    pub col: Option<u16>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Symbol {
    pub name: String,
    pub loc: Either<SourceLoc, Addr>,
}
