#![recursion_limit = "128"]
extern crate proc_macro;
use proc_macro::TokenStream;
use std::env;

use quote::quote;
use syn::Item;

mod contract;
mod riff;
mod util;

/// Generates a companion Trait which has a default type `Impl`, which implements this trait.
///
#[proc_macro_attribute]
pub fn riff(_: TokenStream, item: TokenStream) -> TokenStream {
    let parsed: Item = syn::parse(item).unwrap();
    riff::generate(parsed).into()
}

#[proc_macro_derive(IntoKey)]
pub fn into_key(item: TokenStream) -> TokenStream {
    syn::parse::<Item>(item)
        .and_then(riff::into_key::from_item)
        .map_or_else(|e| e.to_compile_error().into(), Into::into)
}

#[proc_macro_derive(Lazy)]
pub fn lazy(item: TokenStream) -> TokenStream {
    syn::parse::<Item>(item)
        .and_then(riff::lazy::from_item)
        .map_or_else(|e| e.to_compile_error().into(), Into::into)
}

/// Generates the soroban contract code combining all Riffs
#[proc_macro]
pub fn soroban_contract(_: TokenStream) -> TokenStream {
    let cargo_file = manifest();
    let riffs = loam_build::deps::riff(&cargo_file).unwrap();

    let deps = riffs
        .iter()
        .map(|i| i.0.to_path_buf().into_std_path_buf())
        .collect::<Vec<_>>();

    contract::generate(&deps).into()
}

fn manifest() -> std::path::PathBuf {
    std::path::PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("Cargo.toml")
}

/// Generates a contract Client for a given contract.
/// It is expected that the name should be the same as the published contract or a contract in your current workspace.
#[proc_macro]
pub fn import_contract(tokens: TokenStream) -> TokenStream {
    let cargo_file = manifest();
    let mut dir = loam_build::get_target_dir(&cargo_file)
        .unwrap()
        .join(tokens.to_string());
    let name = syn::parse::<syn::Ident>(tokens).expect("The input must be a valid identifier");
    dir.set_extension("wasm");
    let binding = dir.canonicalize().unwrap();
    let file = binding.to_str().unwrap();
    quote! {
        mod #name {
            use loam_sdk::soroban_sdk;
            loam_sdk::soroban_sdk::contractimport!(file = #file);
        }
    }
    .into()
}
