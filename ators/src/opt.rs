use std::{any::Any, fmt};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Opt {
    Object,
    LoadAddr,
    SlideAddr,
    Offset,
    Address,
    Arch,
    Inline,
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

pub trait OptMatching {
    fn get_one<T: Any + Clone + Send + Sync + 'static>(&self, id: Opt) -> Option<T>;
    fn get_either<T: Any + Clone + Send + Sync + 'static>(
        &self,
        ids: impl IntoIterator<Item = Opt>,
    ) -> Option<T>;
    fn get_flag(&self, id: Opt) -> bool;
    fn get_many<T: Any + Clone + Send + Sync + 'static>(&self, id: Opt) -> Vec<T>;
}

impl OptMatching for clap::ArgMatches {
    fn get_one<T>(&self, id: Opt) -> Option<T>
    where
        T: Any + Clone + Send + Sync + 'static,
    {
        self.get_one(&id.to_string()).map(Clone::clone)
    }

    fn get_either<T>(&self, ids: impl IntoIterator<Item = Opt>) -> Option<T>
    where
        T: Any + Clone + Send + Sync + 'static,
    {
        ids.into_iter()
            .find_map(|id| self.get_one(&id.to_string()))
            .map(Clone::clone)
    }

    fn get_flag(&self, id: Opt) -> bool {
        self.get_flag(&id.to_string())
    }

    fn get_many<T>(&self, id: Opt) -> Vec<T>
    where
        T: Any + Clone + Send + Sync + 'static,
    {
        self.get_many(&id.to_string())
            .unwrap()
            .cloned()
            .collect()
    }
}

pub trait FromArgs
where
    Self: Sized,
{
    fn from_args<M: OptMatching>(args: M) -> Option<Self>;
}

impl FromArgs for atorsl::Context {
    fn from_args<M: OptMatching>(args: M) -> Option<Self> {
        Some(Self {
            path: args.get_one(Opt::Object)?,
            loc: args.get_either([Opt::LoadAddr, Opt::SlideAddr, Opt::Offset])?,
            addrs: args.get_many(Opt::Address),
            arch: args.get_one(Opt::Arch),
            include_inlined: args.get_flag(Opt::Inline),
        })
    }
}
