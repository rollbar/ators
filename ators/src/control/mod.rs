use gimli::{Attribute, DebuggingInformationEntry, EndianSlice, RunTimeEndian, UnitHeader};

pub mod dump;
pub mod format;

pub use dump::dump_object;

pub type Header<'a> = UnitHeader<EndianSlice<'a, RunTimeEndian>, usize>;
pub type Entry<'a> = DebuggingInformationEntry<'a, 'a, EndianSlice<'a, RunTimeEndian>, usize>;
pub type Attr<'a> = Attribute<EndianSlice<'a, RunTimeEndian>>;
pub type Unit<'a> = gimli::Unit<EndianSlice<'a, RunTimeEndian>, usize>;
