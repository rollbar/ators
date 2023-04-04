use crate::demangle::swift;
use gimli::{EndianSlice, RunTimeEndian};
use std::{
    fmt,
    ops::{Add, Deref},
    str::FromStr,
};

/// A 64-bit address.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Symbol(String);

impl Symbol {
    pub fn empty() -> Self {
        Self("".to_string())
    }

    pub fn demangle(self) -> Symbol {
        if swift::is_mangled(&self) {
            swift::demangle(&self)
                .map(Symbol::from)
                .unwrap_or(self)
        } else {
            self
        }
    }
}

impl Default for Symbol {
    fn default() -> Self {
        Self::empty()
    }
}

impl Deref for Symbol {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Symbol {
    type Err = <String as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl From<String> for Symbol {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl<'input> From<EndianSlice<'input, RunTimeEndian>> for Symbol {
    fn from(value: EndianSlice<'input, RunTimeEndian>) -> Self {
        Self(value.to_string_lossy().to_string())
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl fmt::Debug for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl Add<Symbol> for Symbol {
    type Output = Self;

    fn add(self, rhs: Symbol) -> Self {
        Self(self.0 + rhs.as_str())
    }
}

impl Add<String> for Symbol {
    type Output = Self;

    fn add(self, rhs: String) -> Self {
        Self(self.0 + rhs.as_str())
    }
}

impl Add<&str> for Symbol {
    type Output = Self;

    fn add(self, rhs: &str) -> Self {
        Self(self.0 + rhs)
    }
}

impl Add<Symbol> for String {
    type Output = Self;

    fn add(self, rhs: Symbol) -> Self {
        self + rhs.0.as_ref()
    }
}

pub trait JoinInlinedSymbols {
    fn join(self) -> String;
}

impl<I> JoinInlinedSymbols for I
where
    I: Iterator<Item = Symbol>,
{
    fn join(self) -> String {
        self.fold("".to_string(), |acc, s| acc + "\n" + s)
    }
}
