use super::Addr;
use std::{ffi, fmt, io, num::ParseIntError, str, string::FromUtf8Error};
use thiserror::Error;

/// An atorsl error.
#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to open file: {0}")]
    Io(#[from] io::Error),

    #[error("Formatting error: {0}")]
    Fmt(#[from] fmt::Error),

    #[error("Error reading DWARF: {0}")]
    Gimli(#[from] gimli::Error),

    #[error("Error reading binary image object: {0}")]
    Object(#[from] object::read::Error),

    #[error("Error building structure: {0}")]
    Builder(#[from] derive_builder::UninitializedFieldError),

    #[error("vmaddr: __TEXT segment not found")]
    VmAddrTextSegmentNotFound,

    #[error("Compilation unit for address has no path: {0}")]
    CompUnitDirMissing(Addr),

    #[error("Compilation unit for address has no line program: {0}")]
    CompUnitLineProgramMissing(Addr),

    #[error("Address not found: {0}")]
    AddrNotFound(Addr),

    #[error("Address does not point to a symbol: {0}")]
    AddrSymbolMissing(Addr),

    #[error("Address does not point to a named entry: {0}")]
    AddrNameMissing(Addr),

    #[error("DebugInfoRef offset not found: {0}")]
    AddrDebugInfoRefOffsetNofFound(Addr),

    #[error("DebugInfoRef offset out of bounds: {0}")]
    AddrDebugInfoRefOffsetOutOfBounds(Addr),

    #[error("Address has no line information: {0}")]
    AddrLineInfoMissing(Addr),

    #[error("Address has no file information: {0}")]
    AddrFileInfoMissing(Addr),

    #[error("No debug offset in address: {0}")]
    AddrDebugInfoOffsetMissing(Addr),

    #[error("Invalid address: {0}")]
    AddrInvalid(Addr),

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

    #[error("Cannot demangle symbol of unknown language: {0}")]
    DemangleUnknownLanguage(String),

    #[error("Cannot demangle MSVC/C++ symbol: {0}")]
    DemangleErrorMsvc(#[from] msvc_demangler::Error),

    #[error("Cannot demangle C/C++ symbol: {0}")]
    DemangleErrorCppParse(#[from] cpp_demangle::error::Error),

    #[error("Cannot demangle Rust symbol: {0}")]
    DemangleErrorRust(String),

    #[error("Cannot demangle Swift symbol: {0}")]
    DemangleErrorSwift(String),
}
