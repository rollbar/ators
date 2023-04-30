use super::Addr;
use itertools::Either;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceLoc {
    pub file: PathBuf,
    pub line: u64,
    pub col: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Symbol {
    pub addr: Addr,
    pub name: String,
    pub loc: Either<Option<SourceLoc>, Addr>,
}
