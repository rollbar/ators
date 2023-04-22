use crate::AttrValue;
use derive_builder::Builder;
use std::{borrow::Cow, path::PathBuf};

#[derive(Builder, Clone, Debug, PartialEq, Eq)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "crate::Error"))]
pub struct CompilationUnit<'input> {
    pub name: (AttrValue<'input>, Cow<'input, str>),
    pub dir: (AttrValue<'input>, PathBuf),

    #[builder(setter(strip_option), default)]
    pub lang: Option<gimli::DwLang>,
}
