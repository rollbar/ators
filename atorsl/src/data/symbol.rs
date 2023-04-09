use super::{Addr, SourceLoc};
use derive_builder::Builder;
use itertools::Either;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Builder)]
pub struct Symbol {
    pub module: String,
    pub linkage: String,
    pub lang: gimli::DwLang,

    pub loc: Either<SourceLoc, Addr>,
}
