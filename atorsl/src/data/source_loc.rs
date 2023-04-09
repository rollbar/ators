use std::path::PathBuf;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct SourceLoc {
    pub file: PathBuf,
    pub line: u16,
    pub col: Option<u16>,
}
