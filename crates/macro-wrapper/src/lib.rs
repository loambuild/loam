#![recursion_limit = "128"]
extern crate proc_macro;

mod loam;

use proc_macro::TokenStream;

use syn::{AttributeArgs, Item};

/// The loam macro that generates interface needed for current target sdk.

/// ```
#[proc_macro_attribute]
pub fn loam(attr: TokenStream, item: TokenStream) -> TokenStream {
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
