use ators_proc_macro::Newtype;
use std::{cmp::Ordering, fmt, str::FromStr};

#[derive(Newtype, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Address(u64);

impl FromStr for Address {
    type Err = <u64 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<u64>()
            .or_else(|_| u64::from_str_radix(s.trim_start_matches("0x"), 16))
            .map(Address::from)
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("{:#018x}", self.0))
    }
}

impl fmt::Debug for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl PartialEq<u64> for Address {
    fn eq(&self, other: &u64) -> bool {
        self.0 == *other
    }
}

impl PartialEq<Address> for u64 {
    fn eq(&self, other: &Address) -> bool {
        other.0 == *self
    }
}

impl PartialOrd<u64> for Address {
    fn partial_cmp(&self, other: &u64) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialOrd<Address> for u64 {
    fn partial_cmp(&self, other: &Address) -> Option<Ordering> {
        other.0.partial_cmp(self)
    }
}
