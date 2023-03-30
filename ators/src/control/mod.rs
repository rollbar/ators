use gimli::{Attribute, DebuggingInformationEntry, EndianSlice, RunTimeEndian, UnitHeader};

pub mod dump;
pub mod format;
pub mod symbol;

pub use dump::{dump_object, dump_sections};

pub type Header<'a> = UnitHeader<EndianSlice<'a, RunTimeEndian>, usize>;
pub type Entry<'a> = DebuggingInformationEntry<'a, 'a, EndianSlice<'a, RunTimeEndian>, usize>;
pub type Attr<'a> = Attribute<EndianSlice<'a, RunTimeEndian>>;
pub type Unit<'a> = gimli::Unit<EndianSlice<'a, RunTimeEndian>, usize>;
