use std::{
    cmp::Ordering,
    fmt,
    ops::{Deref, DerefMut},
    str::{self, FromStr},
};

/// An offset from a base address.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Offset(usize);

impl Deref for Offset {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Offset {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> AsRef<T> for Offset
where
    T: ?Sized,
    <Self as Deref>::Target: AsRef<T>,
{
    fn as_ref(&self) -> &T {
        self.deref().as_ref()
    }
}

impl From<usize> for Offset {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<u64> for Offset {
    fn from(value: u64) -> Self {
        Self(value as usize)
    }
}

impl FromStr for Offset {
    type Err = <usize as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<usize>().map(Offset::from)
    }
}

impl fmt::Display for Offset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}

impl PartialEq<usize> for Offset {
    fn eq(&self, other: &usize) -> bool {
        self.0 == *other
    }
}

impl PartialEq<Offset> for usize {
    fn eq(&self, other: &Offset) -> bool {
        other.0 == *self
    }
}

impl PartialEq<usize> for &Offset {
    fn eq(&self, other: &usize) -> bool {
        self.0 == *other
    }
}

impl PartialEq<&Offset> for usize {
    fn eq(&self, other: &&Offset) -> bool {
        other.0 == *self
    }
}

impl PartialOrd<usize> for Offset {
    fn partial_cmp(&self, other: &usize) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialOrd<usize> for &Offset {
    fn partial_cmp(&self, other: &usize) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialOrd<Offset> for usize {
    fn partial_cmp(&self, other: &Offset) -> Option<Ordering> {
        self.partial_cmp(&other.0)
    }
}

impl PartialOrd<&Offset> for usize {
    fn partial_cmp(&self, other: &&Offset) -> Option<Ordering> {
        self.partial_cmp(&other.0)
    }
}
