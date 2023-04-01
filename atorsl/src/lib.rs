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

    #[error("vmaddr: __TEXT segment not found")]
    VmAddrTextSegmentNotFound,

    #[error("Address not found ({0})")]
    AddrNotFound(Addr),

    #[error("Address has no a symbols")]
    AddrHasNoSymbol,

    #[error("No debug offset in address ({0})")]
    AddrNoDebugOffset(Addr),
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
                if <object::File as object::Object>::is_little_endian($object) {
                    gimli::RunTimeEndian::Little
                } else {
                    gimli::RunTimeEndian::Big
                },
            )
        })
    }};
}

use gimli::{EndianSlice, RunTimeEndian};

pub(crate) type Dwarf<'input> = gimli::Dwarf<EndianSlice<'input, RunTimeEndian>>;
pub(crate) type Unit<'input> = gimli::Unit<EndianSlice<'input, RunTimeEndian>, usize>;
pub(crate) type UnitHeader<'input> = gimli::UnitHeader<EndianSlice<'input, RunTimeEndian>, usize>;
pub(crate) type Entry<'abbrev, 'unit, 'input> =
    gimli::DebuggingInformationEntry<'abbrev, 'unit, EndianSlice<'input, RunTimeEndian>, usize>;
pub(crate) type AttrValue<'input> = gimli::AttributeValue<EndianSlice<'input, RunTimeEndian>>;
