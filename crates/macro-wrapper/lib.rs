#![recursion_limit = "128"]
extern crate proc_macro;

mod core_impl;
mod loam;

use core_impl::ext::generate_ext_structs;
use core_impl::utils::into_key_from_item;
use proc_macro::TokenStream;

use self::core_impl::*;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::visit::Visit;
use syn::{parse_quote, File, Item, ItemEnum, ItemImpl, ItemStruct, ItemTrait, WhereClause, AttributeArgs};

/// The loam macro that generates interface needed for current target sdk.

/// ```
#[proc_macro_attribute]
pub fn loam(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr: AttributeArgs = syn::parse_macro_input!(attr);
    let item: Item = syn::parse(item).unwrap();
    loam::generate(item, Some(attr)).into()
}

/// `PanicOnDefault` generates implementation for `Default` trait that panics with the following
/// message `Riff must be initialized` when `default()` is called.
#[proc_macro_derive(PanicOnDefault)]
pub fn derive_no_default(item: TokenStream) -> TokenStream {
    if let Ok(input) = syn::parse::<ItemStruct>(item) {
        let name = &input.ident;
        TokenStream::from(quote! {
            impl Default for #name {
                fn default() -> Self {
                    loam_sdk::env::panic_str("Riff must be initialized.");
                }
            }
        })
    } else {
        TokenStream::from(
            syn::Error::new(
                Span::call_site(),
                "PanicOnDefault can only be used on type declarations sections.",
            )
            .to_compile_error(),
        )
    }
}

#[proc_macro_derive(IntoKey)]
pub fn into_key(item: TokenStream) -> TokenStream {
    syn::parse::<Item>(item)
        .and_then(into_key_from_item)
        .map(Into::into)
        .unwrap_or_else(|e| e.to_compile_error().into())
}
