use super::{symbol::SymbolBuilderError, Addr};
use std::{ffi, num::ParseIntError, str, string::FromUtf8Error};

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

    #[error("Address does not point to a symbol")]
    EntryInAddrNotSymbol,

    #[error("No debug offset in address: {0}")]
    AddrNoDebugOffset(Addr),

    #[error("Invalid address {0}")]
    AddrInvalid(Addr),

    #[error("Cannot demangle symbol: {0}")]
    CannotDemangleSymbol(String),

    #[error("An error occurred while building the Symbol: {0}")]
    ErrorBuildingSymbol(#[from] SymbolBuilderError),

    #[error("A string passed had an interior nul byte: {0}")]
    InteriorNul(#[from] ffi::NulError),

    #[error("Invalid UTF-8 in C string: {0}")]
    UnrepresentableString(#[from] str::Utf8Error),

    #[error("Invalid UTF-8 in byte vector: {0}")]
    UnrepresentableStringFromByteVector(#[from] FromUtf8Error),

    #[error("Integer is not a valid address: {0}")]
    UnrepresentableAddress(#[from] ParseIntError),
}
