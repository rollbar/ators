use std::{fmt, path::PathBuf};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Opt {
    Object,
    LoadAddr,
    SlideAddr,
    Offset,
    Addr,
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

pub trait FromArgs<'a>
where
    Self: Sized,
{
    fn from_args(args: &'a clap::ArgMatches) -> Option<Self>;
}

impl<'a> FromArgs<'a> for atorsl::Context<'a> {
    fn from_args(args: &'a clap::ArgMatches) -> Option<Self> {
        Some(Self {
            path: args.get_one::<PathBuf>(&Opt::Object.to_string())?,

            loc: [Opt::LoadAddr, Opt::SlideAddr, Opt::Offset]
                .into_iter()
                .find_map(|opt| args.get_one(&opt.to_string()))?,

            addrs: args.get_many(&Opt::Addr.to_string())?.collect(),

            arch: args
                .get_one(&Opt::Arch.to_string())
                .map(String::as_str),

            include_inlined: args.get_flag(&Opt::Inline.to_string()),

            delimiter: args
                .get_one(&Opt::Delimiter.to_string())
                .map(String::as_str)
                .unwrap_or("\n"),
        })
    }
}
