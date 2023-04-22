use super::Addr;
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

    #[error("Error building structure: {0}")]
    Builder(#[from] derive_builder::UninitializedFieldError),

    #[error("vmaddr: __TEXT segment not found")]
    VmAddrTextSegmentNotFound,

    #[error("Compilation unit for address has no name: {0}")]
    CompUnitNameMissing(Addr),

    #[error("Compilation unit for address has no path: {0}")]
    CompUnitDirMissing(Addr),

    #[error("Compilation unit for address has no line program: {0}")]
    CompUnitLineProgramMissing(Addr),

    #[error("Address not found: {0}")]
    AddrNotFound(Addr),

    #[error("Address does not point to a symbol")]
    AddrNotSymbol,

    #[error("Address does not point to a named entry")]
    AddrNotNamed,

    #[error("Address has no line information: {0}")]
    AddrLineInfoMissing(Addr),

    #[error("Address has no file information: {0}")]
    AddrFileInfoMissing(Addr),

    #[error("No debug offset in address: {0}")]
    AddrNoDebugOffset(Addr),

    #[error("Invalid address: {0}")]
    AddrInvalid(Addr),

    #[error("Cannot demangle symbol: {0}")]
    CannotDemangleSymbol(String),

    #[error("A string passed had an interior nul byte: {0}")]
    InteriorNul(#[from] ffi::NulError),

    #[error("Invalid UTF-8 in C string: {0}")]
    UnrepresentableString(#[from] str::Utf8Error),

    #[error("Invalid UTF-8 in byte vector: {0}")]
    UnrepresentableStringFromByteVector(#[from] FromUtf8Error),

    #[error("Integer is not a valid address: {0}")]
    UnrepresentableAddress(#[from] ParseIntError),

    #[error("Cannot load symbols")]
    CannotLoadSymbols,

    #[error("Cannot load symbols for architecture {0:?}")]
    CannotLoadSymbolsForArch(object::Architecture),

    #[error("Found no UUID in the given object")]
    ObjectHasNoUuid,
}
