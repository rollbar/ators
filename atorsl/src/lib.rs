mod format;

pub mod addr;
pub mod ext;
pub mod lookup;

pub use addr::Addr;
pub use lookup::Lookup;

/// The program's context, defines its behavior.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Context {
    pub objpath: std::path::PathBuf,
    pub loadaddr: Addr,
    pub addrs: Vec<Addr>,
    pub arch: Option<String>,
    pub inline: bool,
    pub verbose: bool,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to open file: {0}")]
    Io(#[from] std::io::Error),

    #[error("Error reading DWARF: {0}")]
    Gimli(#[from] gimli::Error),

    #[error("Error reading binary image object: {0}")]
    Object(#[from] object::read::Error),

    #[error("__TEXT segment not found in DWARF")]
    TextSegmentNotFound,

    #[error("Address not found ({0})")]
    AddressNotFound(Addr),

    #[error("Address is not a symbol")]
    AddressHasNoSymbol,

    #[error("No debug offset in address ({0})")]
    NoDebugOffsetInAddress(Addr),
}

#[macro_export]
macro_rules! load_object {
    ($path:expr, $binding:ident) => {{
        $binding = unsafe { memmap2::Mmap::map(&std::fs::File::open(&$path)?) }?;
        Result::<object::File, atorsl::Error>::Ok(object::File::parse(&*$binding)?)
    }};
}

#[macro_export]
macro_rules! load_dwarf {
    ($object:expr, $binding:ident) => {{
        $binding = gimli::Dwarf::load(|sid| -> gimli::Result<std::borrow::Cow<[u8]>> {
            Ok(
                <object::File as object::Object>::section_by_name(&$object, sid.name())
                    .and_then(|s| {
                        <object::Section as object::ObjectSection>::uncompressed_data(&s).ok()
                    })
                    .unwrap_or(std::borrow::Cow::Borrowed(&[][..])),
            )
        })?;

        $binding.borrow(|section| {
            gimli::EndianSlice::new(
                &*section,
                <object::File as atorsl::ext::object::File>::runtime_endian(&$object),
            )
        })
    }};
}
