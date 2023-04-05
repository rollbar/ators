use crate::Addr;
use std::path::Path;

/// The location address of the binary image containing symbol addresses.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Loc {
    /// The load address of the binary image.
    ///
    /// Load addresses for binary images can be found in the Binary Images: section at the
    /// bottom of the crash, sample, leaks, and malloc_history reports.
    Load(Addr),

    /// The slide value of the binary image.
    ///
    /// This is the difference between the load address of a binary image, and the address at
    /// which the binary image was built.
    ///
    /// This slide value is subtracted from the input addresses.
    Slide(Addr),

    /// Treat all given addresses as offsets into the binary.
    Offset,
}

/// The program's context, defines its behavior.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Context<'ctx> {
    /// The full path to either a binary image, eg. the DWARF file, or a .dSYM.
    pub path: &'ctx Path,

    /// The location address of the binary image containing the addresses to symbolicate.
    pub loc: &'ctx Loc,

    /// The addresses to symbolicate.
    pub addrs: Vec<&'ctx Addr>,

    /// The particular architecure of a binary image file in which to look up symbols.
    pub arch: Option<&'ctx str>,

    /// Whether to expand inlined symbols.
    pub include_inlined: bool,

    /// Output delimiter, defaults to newline
    pub delimiter: &'ctx str,
}
