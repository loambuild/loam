use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Stream(TokenStream),
}
impl From<Error> for TokenStream {
    fn from(value: Error) -> Self {
        match value {
            Error::Stream(ts) => ts,
        }
    }
}

/// Find all riff deps then use `syn_file_expand` to generate the needed functions from each dep
pub fn generate(contract: &Ident, methods: &[&TokenStream]) -> TokenStream {
    quote! {
        struct #contract;
        #[loam_sdk::soroban_sdk::contract(crate_path = "loam_sdk::soroban_sdk")]
        struct SorobanContract__;
        #[loam_sdk::soroban_sdk::contractimpl(crate_path = "loam_sdk::soroban_sdk")]
        impl SorobanContract__ {
                #(#methods)*
        }
    }
}

pub fn generate_boilerplate(name: syn::Ident, methods: &[&TokenStream]) -> TokenStream {
    generate(&name, methods)
}
