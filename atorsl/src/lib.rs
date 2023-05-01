#![allow(unstable_name_collisions)]

pub mod data;
pub mod demangler;
pub mod ext;
pub mod symbolicator;

pub use data::Error;
pub use symbolicator::{atos_dwarf, atos_map};

pub(crate) mod prelude;
pub(crate) use prelude::*;

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

/// Executes a `LineProgram` to iterate over the rows in the matrix of line number information.
///
/// "The hypothetical machine used by a consumer of the line number information
/// to expand the byte-coded instruction stream into a matrix of line number
/// information." -- Section 6.2.1
pub(crate) type IncompleteLineProgramRows<'input> = gimli::LineRows<
    gimli::EndianSlice<'input, gimli::RunTimeEndian>,
    gimli::IncompleteLineProgram<gimli::EndianSlice<'input, gimli::RunTimeEndian>, usize>,
    usize,
>;

/// A header for a line number program in the `.debug_line` section, as defined
/// in section 6.2.4 of the standard.
pub(crate) type LineProgramHeader<'input> =
    gimli::LineProgramHeader<gimli::EndianSlice<'input, gimli::RunTimeEndian>, usize>;
