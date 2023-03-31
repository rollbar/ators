use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Field, Fields, FieldsUnnamed};

fn destructure(ast: &DeriveInput) -> (&Ident, &Field) {
    let outer = &ast.ident;
    let inner = match &ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Unnamed(FieldsUnnamed { ref unnamed, .. }),
            ..
        }) if unnamed.len() == 1 => &unnamed[0],
        _ => panic!("Only trivial tuple structs wrapping over a single field can be derived."),
    };

    (outer, inner)
}

pub fn newtype(ast: &DeriveInput) -> TokenStream {
    TokenStream::from_iter([deref, deref_mut, deref_asref, from].map(|f| f(ast)))
}

pub fn from(ast: &DeriveInput) -> TokenStream {
    let (outer, inner) = destructure(ast);

    quote! {
        impl From<#inner> for #outer {
            fn from(value: #inner) -> Self {
                Self(value)
            }
        }
    }
    .into()
}

pub fn deref(ast: &DeriveInput) -> TokenStream {
    let (outer, inner) = destructure(ast);

    quote! {
        impl std::ops::Deref for #outer {
            type Target = #inner;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    }
    .into()
}

pub fn deref_mut(ast: &DeriveInput) -> TokenStream {
    let (outer, _) = destructure(ast);

    quote! {
        impl std::ops::DerefMut for #outer {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    }
    .into()
}

pub fn deref_asref(ast: &DeriveInput) -> TokenStream {
    let (outer, _) = destructure(ast);

    quote! {
        impl<T> AsRef<T> for #outer
        where
            T: ?Sized,
            <#outer as std::ops::Deref>::Target: AsRef<T> {

            fn as_ref(&self) -> &T {
                use std::ops::Deref;
                self.deref().as_ref()
            }
        }
    }
    .into()
}
