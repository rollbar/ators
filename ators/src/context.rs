use crate::{cli, Addr};
use anyhow::{Context as _, Result};
use atorsl::ext::object::Architecture as _;
use itertools::Either;
use object::Architecture;
use std::{
    fs,
    path::{Path, PathBuf},
};

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Mode {
    Symbolicate,
    PrintUuid,
}

/// The program's context, defines its behavior.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Context<'ctx> {
    /// Whether we want to symbolicate or print uuid of the binary
    pub mode: Mode,

    /// The full path to either a binary image, eg. the DWARF file, or a .dSYM.
    ///
    /// When provided with a .dSYM, the _first file in alphabetical descending order_ in
    /// `Contents/Resources/DWARF` is loaded. This emulates `atos` behavior.
    pub obj_path: PathBuf,

    /// The location address of the binary image containing the addresses to symbolicate.
    pub base_addr: &'ctx Loc,

    /// Either a list of addresses to symbolicate or an input file with white-separated
    /// numeric addresses.
    pub addrs: Either<Vec<Addr>, &'ctx Path>,

    /// The particular architecure of a binary image file in which to look up symbols.
    pub arch: Option<Architecture>,

    /// Whether to expand inlined symbols.
    pub include_inlined: bool,

    /// Output delimiter when expanding inlined frames, defaults to newline
    pub delimiter: &'ctx str,

    /// Print the full path of the source files
    pub show_full_path: bool,

    /// Prefix all symbols with their respective address
    pub prefix_addrs: bool,
}

impl<'a> Context<'a> {
    pub fn from_args(args: &'a clap::ArgMatches) -> Result<Self> {
        Ok(Self {
            mode: if args.get_flag(&cli::Opt::Uuid.to_string()) {
                Mode::PrintUuid
            } else {
                Mode::Symbolicate
            },

            obj_path: {
                let path = args
                    .get_one::<PathBuf>(&cli::Opt::Object.to_string())
                    .context("No binary image path")?;

                if let Some(true) = path.extension().map(|ext| ext == "dSYM") {
                    fs::read_dir(path.as_path().join("Contents/Resources/DWARF"))?
                        .next()
                        .context("No binary image path")??
                        .path()
                } else {
                    path.clone()
                }
            },

            base_addr: [cli::Opt::LoadAddr, cli::Opt::SlideAddr]
                .iter()
                .find_map(|opt| args.get_one(&opt.to_string()))
                .unwrap_or(&Loc::Offset),

            addrs: {
                let addr_list = args.get_many(&cli::Opt::Addr.to_string());
                let addr_file = args.get_one::<PathBuf>(&cli::Opt::AddrFile.to_string());

                match (addr_list, addr_file) {
                    (Some(list), _) => Either::Left(list.copied().collect()),
                    (_, Some(file)) => Either::Right(file),
                    _ => Either::Left(Vec::default()),
                }
            },

            arch: args
                .get_one(&cli::Opt::Arch.to_string())
                .map(String::as_str)
                .map(Architecture::from_name),

            include_inlined: args.get_flag(&cli::Opt::Inline.to_string()),

            delimiter: args
                .get_one(&cli::Opt::Delimiter.to_string())
                .map(String::as_str)
                .unwrap_or_default(),

            show_full_path: args.get_flag(&cli::Opt::FullPath.to_string()),

            prefix_addrs: args.get_flag(&cli::Opt::PrefixAddr.to_string()),
        })
    }
}
