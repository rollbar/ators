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
}

/// The program's context, defines its behavior.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Context<'ctx> {
    /// The full path to either a binary image, eg. the DWARF file, or a .dSYM.
    ///
    /// When provided with a .dSYM, the _first file in alphabetical descending order_ in
    /// `Contents/Resources/DWARF` is loaded. This emulates `atos` behavior.
    pub obj_path: PathBuf,

    /// The location address of the binary image containing the addresses to symbolicate.
    pub base_addr: &'ctx Loc,

    /// The addresses to symbolicate.
    pub addrs: Option<Vec<Addr>>,

    /// Input file with white-separated numeric addresses.
    pub input_addr_file: Option<&'ctx Path>,

    /// The particular architecure of a binary image file in which to look up symbols.
    pub arch: Option<&'ctx str>,

    /// Whether to expand inlined symbols.
    pub include_inlined: bool,

    /// Output delimiter, defaults to newline
    pub delimiter: &'ctx str,

    /// Print the full path of the source files
    pub show_full_path: bool,
}

impl<'a> Context<'a> {
    pub fn from_args(args: &'a clap::ArgMatches) -> Result<Self> {
        Ok(Self {
            obj_path: {
                let path = args
                    .get_one::<PathBuf>(&cli::Opt::Object.to_string())
                    .context("No binary image path")?;

                if path.extension().map(|ext| ext == "dSYM") == Some(true) {
                    std::fs::read_dir(path.as_path().join("Contents/Resources/DWARF"))?
                        .next()
                        .context("No binary image path")??
                        .path()
                } else {
                    path.clone()
                }
            },

            base_addr: [cli::Opt::LoadAddr, cli::Opt::SlideAddr, cli::Opt::Offset]
                .into_iter()
                .find_map(|opt| args.get_one(&opt.to_string()))
                .context("No location address")?,

            addrs: args
                .get_many(&cli::Opt::Addr.to_string())
                .map(|addrs| addrs.copied().collect()),

            input_addr_file: args
                .get_one::<PathBuf>(&cli::Opt::AddrFile.to_string())
                .map(|file| file.as_path()),

            arch: args
                .get_one(&cli::Opt::Arch.to_string())
                .map(String::as_str),

            include_inlined: args.get_flag(&cli::Opt::Inline.to_string()),

            delimiter: args
                .get_one(&cli::Opt::Delimiter.to_string())
                .map(String::as_str)
                .context("No delimiter")?,

            show_full_path: args.get_flag(&cli::Opt::FullPath.to_string()),
        })
    }
}
