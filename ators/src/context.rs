use anyhow::{Context as _, Result};

use crate::{cli, Addr};
use std::path::{Path, PathBuf};

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

impl<'a> Context<'a> {
    pub fn from_args(args: &'a clap::ArgMatches) -> Result<Self> {
        Ok(Self {
            path: args
                .get_one::<PathBuf>(&cli::Opt::Object.to_string())
                .context("No binary image path")?,

            loc: [cli::Opt::LoadAddr, cli::Opt::SlideAddr, cli::Opt::Offset]
                .into_iter()
                .find_map(|opt| args.get_one(&opt.to_string()))
                .context("No location address")?,

            addrs: args
                .get_many(&cli::Opt::Addr.to_string())
                .context("No address to symbolicate")?
                .collect(),

            arch: args
                .get_one(&cli::Opt::Arch.to_string())
                .map(String::as_str),

            include_inlined: args.get_flag(&cli::Opt::Inline.to_string()),

            delimiter: args
                .get_one(&cli::Opt::Delimiter.to_string())
                .map(String::as_str)
                .context("No delimiter")?,
        })
    }
}
