pub mod addr;
pub mod context;
pub mod error;
pub mod ext;
pub mod load;
pub mod lookup;

pub use addr::Addr;
pub use context::Context;
pub use error::Error;
pub use lookup::Symbolicate;

use gimli::{EndianSlice, RunTimeEndian};

pub(crate) type Dwarf<'input> = gimli::Dwarf<EndianSlice<'input, RunTimeEndian>>;
pub(crate) type Unit<'input> = gimli::Unit<EndianSlice<'input, RunTimeEndian>, usize>;
pub(crate) type UnitHeader<'input> = gimli::UnitHeader<EndianSlice<'input, RunTimeEndian>, usize>;
pub(crate) type Entry<'abbrev, 'unit, 'input> =
    gimli::DebuggingInformationEntry<'abbrev, 'unit, EndianSlice<'input, RunTimeEndian>, usize>;
pub(crate) type AttrValue<'input> = gimli::AttributeValue<EndianSlice<'input, RunTimeEndian>>;
