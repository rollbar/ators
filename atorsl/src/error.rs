use crate::Addr;

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

    #[error("Address not found ({0})")]
    AddrNotFound(Addr),

    #[error("Address has no a symbols")]
    AddrHasNoSymbol,

    #[error("No debug offset in address ({0})")]
    AddrNoDebugOffset(Addr),
}
