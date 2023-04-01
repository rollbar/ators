pub mod addr;
pub mod context;
pub mod error;
pub mod ext;
pub mod lookup;

pub use addr::Addr;
pub use context::Context;
pub use error::Error;
pub use lookup::Symbolicate;

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
