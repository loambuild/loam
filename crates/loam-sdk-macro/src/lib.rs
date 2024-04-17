#![recursion_limit = "128"]
extern crate proc_macro;
use proc_macro::TokenStream;
use std::env;
use subcontract::derive_contract_impl;
use util::generate_soroban;

use quote::quote;
use syn::Item;

mod contract;
mod subcontract;
mod util;

/// Generates a companion Trait which has a default type `Impl`, which implements this trait.
///
#[proc_macro_attribute]
pub fn subcontract(_: TokenStream, item: TokenStream) -> TokenStream {
    let parsed: Item = syn::parse(item).unwrap();
    subcontract::generate(parsed).into()
}

#[proc_macro_derive(IntoKey)]
pub fn into_key(item: TokenStream) -> TokenStream {
    syn::parse::<Item>(item)
        .and_then(subcontract::into_key::from_item)
        .map_or_else(|e| e.to_compile_error().into(), Into::into)
}

#[proc_macro_derive(Lazy)]
pub fn lazy(item: TokenStream) -> TokenStream {
    syn::parse::<Item>(item)
        .and_then(subcontract::lazy::from_item)
        .map_or_else(|e| e.to_compile_error().into(), Into::into)
}

pub(crate) fn manifest() -> std::path::PathBuf {
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

/// Generates a contract made up of subcontracts
/// ```no_run
/// #[derive_contract(Core(Admin), Postable(StatusMessage))]
/// pub struct Contract;
/// ```
/// Generates
/// ```no_run
/// pub struct Contract;
/// impl Postable for Contract {
///     type Impl = StatusMessage;
/// }
/// impl Core for Contract {
///     type Impl = Admin;
/// }
/// #[loam_sdk::soroban_sdk::contract]
/// struct SorobanContract__;
///
/// #[loam_sdk::soroban_sdk::contract]
/// impl SorobanContract__ {
///  // Postable and Core methods exposed
/// }
///
///
/// ```
#[proc_macro_attribute]
pub fn derive_contract(args: TokenStream, item: TokenStream) -> TokenStream {
    let parsed: Item = syn::parse(item.clone()).unwrap();
    let methods = find_deps();
    println!("{:?}", methods);
    derive_contract_impl(proc_macro2::TokenStream::from(args), parsed, &methods).into()
}

fn find_deps() -> Vec<proc_macro2::TokenStream> {
    let cargo_file = manifest();
    loam_build::deps::riff(&cargo_file)
        .unwrap()
        .iter()
        .map(|i| i.0.to_path_buf().into_std_path_buf())
        .filter_map(|path| {
            println!("{:?}", path);
            let file = util::parse_crate_as_file(&path)?;
            Some(generate_soroban(&file))
        })
        .collect::<Vec<_>>()
}
