use std::path::Path;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{File, ItemTrait, TraitItemFn};
use syn_file_expand::read_full_crate_source_code;

/// Read a crate starting from a single file then parse into a file
pub fn parse_crate_as_file(path: &Path) -> Option<File> {
    if let Ok(file) = read_full_crate_source_code(path, |_| Ok(false)) {
        let mut tokens = TokenStream::new();
        file.to_tokens(&mut tokens);
        syn::parse(tokens.into()).ok()
    } else {
        None
    }
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

#[derive(Default)]
pub struct TraitVisitor {
    pub functions: Vec<TokenStream>,
}

impl TraitVisitor {
    pub fn find_items_in_file(ast: &syn::File) -> Vec<TokenStream> {
        let mut visitor = TraitVisitor::default();
        syn::visit::visit_file(&mut visitor, ast);
        visitor.functions
    }
}

impl<'ast> Visit<'ast> for TraitVisitor {
    fn visit_item_trait(&mut self, item: &'ast syn::ItemTrait) {
        if has_macro(&item.attrs, "subcontract") {
            self.functions.extend(generate_methods(item));
        }
    }
}

pub fn generate_soroban(file: &syn::File) -> TokenStream {
    let methods = TraitVisitor::find_items_in_file(file);
    println!("methods: {methods:?}");
    quote! {
        #(#methods)*
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
