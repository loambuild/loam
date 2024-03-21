use std::path::PathBuf;

use proc_macro2::TokenStream;
use quote::quote;

use crate::util::{self, generate_soroban};

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
pub fn generate(paths: &[PathBuf]) -> TokenStream {
    let methods = paths
        .iter()
        .filter_map(|path| {
            let file = util::parse_crate_as_file(path)?;
            Some(generate_soroban(&file))
        })
        .collect::<Vec<_>>();
    quote! {
    #[loam_sdk::soroban_sdk::contract(crate_path = "loam_sdk::soroban_sdk")]
    pub struct SorobanContract;
    #[loam_sdk::soroban_sdk::contractimpl(crate_path = "loam_sdk::soroban_sdk")]
    impl SorobanContract {
            #(#methods)*
    }}
}
