#![allow(unstable_name_collisions)]

pub mod data;
pub mod demangler;
pub mod ext;
pub mod symbolicator;

pub use data::Error;
pub use symbolicator::{atos_dwarf, atos_obj};

pub(crate) mod prelude;
pub(crate) use prelude::{IsOkAnd, IsSomeAnd};

/// Loads a binary image object as DWARF.
#[macro_export]
macro_rules! load_dwarf {
    ($object:expr, $binding:ident) => {{
        $binding = gimli::Dwarf::load(|section_id| -> gimli::Result<std::borrow::Cow<[u8]>> {
            Ok(
                <object::File as object::Object>::section_by_name(&$object, section_id.name())
                    .and_then(|section| object::ObjectSection::uncompressed_data(&section).ok())
                    .unwrap_or(std::borrow::Cow::Borrowed(&[][..])),
            )
        })?;

        $binding.borrow(|section| {
            gimli::EndianSlice::new(
                &*section,
                if object::Object::is_little_endian($object) {
                    gimli::RunTimeEndian::Little
                } else {
                    gimli::RunTimeEndian::Big
                },
            )
        })
    }};
}

/// Commonly used DWARF sections, and other common information.
pub type Dwarf<'input> = gimli::Dwarf<gimli::EndianSlice<'input, gimli::RunTimeEndian>>;

/// Commonly used information for a unit in the `.debug_info` or `.debug_types` sections.
pub(crate) type Unit<'input> =
    gimli::Unit<gimli::EndianSlice<'input, gimli::RunTimeEndian>, usize>;

/// The value of an attribute in a `DebuggingInformationEntry`.
pub(crate) type AttrValue<'input> =
    gimli::AttributeValue<gimli::EndianSlice<'input, gimli::RunTimeEndian>>;

/// A Debugging Information Entry (DIE).
///
/// DIEs have a set of attributes and optionally have children DIEs as well.
pub(crate) type Entry<'abbrev, 'unit, 'input> = gimli::DebuggingInformationEntry<
    'abbrev,
    'unit,
    gimli::EndianSlice<'input, gimli::RunTimeEndian>,
    usize,
>;
