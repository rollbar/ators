use gimli::{EndianSlice, RunTimeEndian};

pub mod dump;
pub mod format;
pub mod lookup;

pub use dump::Dump;
pub use lookup::Lookup;

pub type Dwarf<'a> = gimli::Dwarf<EndianSlice<'a, RunTimeEndian>>;
pub type Header<'a> = gimli::UnitHeader<EndianSlice<'a, RunTimeEndian>, usize>;
pub type Entry<'a> =
    gimli::DebuggingInformationEntry<'a, 'a, EndianSlice<'a, RunTimeEndian>, usize>;
pub type Attr<'a> = gimli::Attribute<EndianSlice<'a, RunTimeEndian>>;
pub type Unit<'a> = gimli::Unit<EndianSlice<'a, RunTimeEndian>, usize>;
