#![recursion_limit = "128"]
extern crate proc_macro;

mod loam;
mod util;

use std::{env, path::PathBuf};

use proc_macro::TokenStream;

use syn::{AttributeArgs, Item};

mod contract;

/// Generates a companion Trait which has a default type `Impl`, which implements this trait.

/// ```
#[proc_macro_attribute]
pub fn riff(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr: AttributeArgs = syn::parse_macro_input!(attr);
    let parsed: Item = syn::parse(item).unwrap();
    loam::generate(parsed, Some(attr)).into()
}

#[proc_macro_derive(IntoKey)]
pub fn into_key(item: TokenStream) -> TokenStream {
    syn::parse::<Item>(item)
        .and_then(loam::into_key::from_item)
        .map_or_else(|e| e.to_compile_error().into(), Into::into)
}

#[proc_macro_derive(Lazy)]
pub fn lazy(item: TokenStream) -> TokenStream {
    syn::parse::<Item>(item)
        .and_then(loam::lazy::from_item)
        .map_or_else(|e| e.to_compile_error().into(), Into::into)
}

#[proc_macro]
pub fn soroban_contract(_: TokenStream) -> TokenStream {
    let dir = std::path::PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("Cargo.toml");
    let deps = contract::get_loam_deps(&dir).unwrap();

    let deps = deps
        .iter()
        .map(|i| PathBuf::from(i.0.to_string()))
        .collect::<Vec<_>>();

    contract::generate(&deps).into()
}
