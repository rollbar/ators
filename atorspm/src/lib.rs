use proc_macro::TokenStream;
use quote::quote;

mod derive;

#[proc_macro_attribute]
pub fn newtype(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);
    quote! {
        #[derive(zebra_proc_macro::Newtype, Clone, PartialEq, Eq, Hash, Debug)]
        #input
    }
    .into()
}

#[proc_macro_derive(Newtype)]
pub fn derive_newtype(tokens: TokenStream) -> TokenStream {
    syn::parse(tokens).map_or_else(error, |ast| derive::newtype(&ast))
}

#[proc_macro_derive(From)]
pub fn derive_from(tokens: TokenStream) -> TokenStream {
    syn::parse(tokens).map_or_else(error, |ast| derive::from(&ast))
}

#[proc_macro_derive(Deref)]
pub fn derive_deref(tokens: TokenStream) -> TokenStream {
    syn::parse(tokens).map_or_else(error, |ast| derive::deref(&ast))
}

#[proc_macro_derive(DerefMut)]
pub fn derive_deref_mut(tokens: TokenStream) -> TokenStream {
    syn::parse(tokens).map_or_else(error, |ast| derive::deref_mut(&ast))
}

fn error(err: syn::Error) -> TokenStream {
    err.to_compile_error().into()
}
