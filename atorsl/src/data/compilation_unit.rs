use derive_builder::Builder;
use std::path::PathBuf;

#[derive(Builder, Clone, Debug, PartialEq, Eq)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "crate::Error"))]
pub struct CompilationUnit {
    pub name: String,
    pub dir: PathBuf,

    #[builder(setter(strip_option), default)]
    pub lang: Option<gimli::DwLang>,
}
