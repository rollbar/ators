pub(crate) mod entry;
pub(crate) use entry::DebuggingInformationEntry;

use crate::data::Addr;

pub(crate) trait ArangeEntry {
    fn contains(&self, addr: &Addr) -> Result<bool, gimli::Error>;
}

impl ArangeEntry for gimli::ArangeEntry {
    fn contains(&self, addr: &Addr) -> Result<bool, gimli::Error> {
        self.address()
            .checked_add(self.length())
            .map(|address_end| (self.address()..address_end).contains(addr))
            .ok_or(gimli::Error::InvalidAddressRange)
    }
}

pub(crate) trait Range {
    fn contains(&self, addr: &Addr) -> bool;
}

impl Range for gimli::Range {
    fn contains(&self, addr: &Addr) -> bool {
        (self.begin..self.end).contains(addr)
    }
}
