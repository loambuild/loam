use std::{collections::BTreeMap, path::Path};

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{File, ItemTrait, TraitItemFn};

/// Read a crate starting from a single file then parse into a file
pub fn parse_crate_as_file(path: &Path) -> Option<File> {
    syn_file_expand::read_crate(path)
        .map_err(|e| {
            println!("{e}");
            e
        })
        .ok()
        .map(|file| file.to_token_stream().to_string())
        .and_then(|file| syn::parse_file(&file).ok())
}

pub fn has_macro(attrs: &[syn::Attribute], macro_name: &str) -> bool {
    for attr in attrs {
        if format!("{:#?}", attr.path()).contains(macro_name) {
            return true;
        }
    }
    false
}

use syn::visit::Visit;

pub type Traits = BTreeMap<String, Vec<TokenStream>>;

#[derive(Default)]
pub struct TraitVisitor {
    pub traits: Traits,
}

impl TraitVisitor {
    pub fn find_traits_in_file(ast: &syn::File) -> Traits {
        let mut visitor = TraitVisitor::default();
        syn::visit::visit_file(&mut visitor, ast);
        visitor.traits
    }
}

impl<'ast> Visit<'ast> for TraitVisitor {
    fn visit_item_trait(&mut self, item: &'ast syn::ItemTrait) {
        if has_macro(&item.attrs, "subcontract") {
            self.traits
                .insert(item.ident.to_string(), generate_methods(item));
        }
    }
}

fn generate_methods(item: &ItemTrait) -> Vec<TokenStream> {
    item.items
        .iter()
        .filter_map(|item| {
            if let syn::TraitItem::Fn(TraitItemFn { sig, attrs, .. }) = item {
                let name = &sig.ident;
                Some(generate_method(sig, attrs, name))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

fn generate_method(
    sig: &syn::Signature,
    attrs: &[syn::Attribute],
    name: &syn::Ident,
) -> TokenStream {
    let output = &sig.output;
    let inputs = sig.inputs.iter().skip(1);
    let args_without_self = crate::subcontract::get_args_without_self(&sig.inputs);
    quote! {
        #(#attrs)*
        pub fn #name(env: loam_sdk::soroban_sdk::Env, #(#inputs),*) #output {
            loam_sdk::soroban_sdk::set_env(env);
            Contract::#name(#(#args_without_self),*)
        }
    }
}
