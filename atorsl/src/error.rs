use crate::{symbolicator, Addr};
use std::{ffi, str};

/// An atorsl error.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to open file: {0}")]
    Io(#[from] std::io::Error),

    #[error("Error reading DWARF: {0}")]
    Gimli(#[from] gimli::Error),

    #[error("Error reading binary image object: {0}")]
    Object(#[from] object::read::Error),

    #[error("vmaddr: __TEXT segment not found")]
    VmAddrTextSegmentNotFound,

    #[error("Address not found: {0}")]
    AddrNotFound(Addr),

    #[error("Address has no a symbols")]
    EntryHasNoSymbol,

    #[error("No debug offset in address: {0}")]
    AddrNoDebugOffset(Addr),

    #[error("Address {0} overflown by offset {1}")]
    AddrOffsetOverflow(Addr, Addr),

    #[error("Cannot demangle symbol: {0}")]
    CannotDemangleSymbol(String),

    #[error("An error occurred while building the Symbol: {0}")]
    ErrorBuildingSymbol(#[from] symbolicator::SymbolBuilderError),

    #[error("A string passed had an interior nul byte: {0}")]
    InteriorNul(#[from] ffi::NulError),

    #[error("Invalid UTF-8 in C string: {0}")]
    UnrepresentableString(#[from] str::Utf8Error),
}
