use crate::Addr;
use std::path::PathBuf;

/// The program's context, defines its behavior.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Context {
    /// The full path to the binary image, eg. the DWARF file.
    pub objpath: PathBuf,

    /// The load address of the binary image containing the addresses to symbolicate.
    pub loadaddr: Addr,

    /// The addresses to symbolicate.
    pub addrs: Vec<Addr>,

    /// The particular architecure of a binary image file in which to look up symbols.
    pub arch: Option<String>,

    /// Whether to expand inlined symbols.
    pub include_inlined: bool,
}
