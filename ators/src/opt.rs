use std::{any::Any, fmt, path::PathBuf};

use clap::ArgMatches;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Opt {
    Object,
    LoadAddr,
    SlideAddr,
    Offset,
    Address,
    Arch,
    Inline,
    Delimiter,
}

impl fmt::Display for Opt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl From<Opt> for clap::Id {
    fn from(value: Opt) -> Self {
        Self::from(value.to_string())
    }
}

pub trait MatchesEither {
    fn get_either<T: Any + Clone + Send + Sync + 'static>(
        &self,
        ids: impl IntoIterator<Item = Opt>,
    ) -> Option<&T>;
}

impl MatchesEither for clap::ArgMatches {
    fn get_either<T>(&self, ids: impl IntoIterator<Item = Opt>) -> Option<&T>
    where
        T: Any + Clone + Send + Sync + 'static,
    {
        ids.into_iter()
            .find_map(|id| self.get_one(&id.to_string()))
    }
}

pub trait FromArgs<'a>
where
    Self: Sized,
{
    fn from_args(args: &'a ArgMatches) -> Option<Self>;
}

impl<'a> FromArgs<'a> for atorsl::Context<'a> {
    fn from_args(args: &'a ArgMatches) -> Option<Self> {
        Some(Self {
            path: args.get_one::<PathBuf>(&Opt::Object.to_string())?,
            loc: args.get_either([Opt::LoadAddr, Opt::SlideAddr, Opt::Offset])?,
            addrs: args.get_many(&Opt::Address.to_string())?.collect(),
            arch: args
                .get_one(&Opt::Arch.to_string())
                .map(String::as_str),
            include_inlined: args.get_flag(&Opt::Inline.to_string()),
            delimiter: args
                .get_one::<String>(&Opt::Delimiter.to_string())
                .map(String::as_str)
                .unwrap_or("\n"),
        })
    }
}
