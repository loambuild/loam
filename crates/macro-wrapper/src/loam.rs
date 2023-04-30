use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{punctuated::Punctuated, Attribute, AttributeArgs, FnArg, Item, Signature, Token};

pub mod into_key;

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

pub fn generate(item: Item, attr: Option<AttributeArgs>) -> TokenStream {
    inner_generate(item, attr).unwrap_or_else(Into::into)
}
fn is_result_type(output: &syn::ReturnType) -> bool {
    if let syn::ReturnType::Type(_, ty) = output {
        if let syn::Type::Path(type_path) = &**ty {
            // Check if the return type is a Result.
            if let Some(segment) = type_path.path.segments.last() {
                return segment.ident == "Result";
            }
        }
    }
    false
}
fn generate_method(trait_item: &syn::TraitItem) -> Option<TokenStream> {
    if let syn::TraitItem::Method(method) = trait_item {
        let sig = &method.sig;
        let name = &sig.ident;
        let output = &sig.output;
        let self_ty = get_receiver(sig.inputs.iter().next()?)?;

        let is_result = is_result_type(output);
        let args_without_self = get_args_without_self(&sig.inputs);
        let attrs = &method.attrs;
        let return_question_mark = if is_result { Some(quote!(?)) } else { None };

        if is_mutable_method(self_ty) {
            Some(generate_mutable_method(
                sig,
                attrs,
                name,
                &args_without_self,
                &return_question_mark,
            ))
        } else {
            Some(generate_immutable_method(
                sig,
                attrs,
                name,
                &args_without_self,
            ))
        }
    } else {
        None
    }
}

fn get_receiver(arg: &syn::FnArg) -> Option<&syn::Receiver> {
    if let syn::FnArg::Receiver(receiver) = arg {
        Some(receiver)
    } else {
        None
    }
}

pub fn get_args_without_self(inputs: &Punctuated<FnArg, Token!(,)>) -> Vec<Ident> {
    inputs
        .iter()
        .skip(1)
        .filter_map(|arg| {
            if let syn::FnArg::Typed(syn::PatType { pat, .. }) = arg {
                match &**pat {
                    syn::Pat::Ident(pat_ident) => Some(pat_ident.ident.clone()),
                    _ => None,
                }
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

fn is_mutable_method(receiver: &syn::Receiver) -> bool {
    receiver.reference.is_some() && receiver.mutability.is_some()
}
fn generate_immutable_method(
    sig: &Signature,
    attrs: &[Attribute],
    name: &Ident,
    args_without_self: &[Ident],
) -> TokenStream {
    let inputs = sig.inputs.iter().skip(1);
    let output = &sig.output;
    quote! {
        #(#attrs)*
        fn #name(#(#inputs),*) #output {
            Self::Impl::get_lazy().unwrap_or_default().#name(#(#args_without_self),*)
        }
    }
}

fn generate_mutable_method(
    sig: &Signature,
    attrs: &[Attribute],
    name: &Ident,
    args_without_self: &[Ident],
    return_question_mark: &Option<TokenStream>,
) -> TokenStream {
    let inputs = sig.inputs.iter().skip(1);
    let output = &sig.output;
    let result = if return_question_mark.is_some() {
        quote!(Ok(res))
    } else {
        quote!(res)
    };
    quote! {
        #(#attrs)*
        fn #name(#(#inputs),*) #output {
            let mut impl_ = Self::Impl::get_lazy().unwrap_or_default();
            let res = impl_.#name(#(#args_without_self),*) #return_question_mark;
            Self::Impl::set_lazy(impl_);
            #result
        }
    }
}

fn inner_generate(item: Item, _attr: Option<AttributeArgs>) -> Result<TokenStream, Error> {
    if let Item::Trait(input_trait) = &item {
        let generated_methods = input_trait
            .items
            .iter()
            .filter_map(generate_method)
            .collect::<Vec<_>>();

        let trait_ident = &input_trait.ident;
        let new_trait_ident = syn::Ident::new(
            trait_ident.to_string().strip_prefix("Is").ok_or_else(|| {
                Error::Stream(quote! { compile_error!("Trait must start with `Is`"); })
            })?,
            trait_ident.span(),
        );
        let (_, ty_generics, _) = input_trait.generics.split_for_impl();

        let attrs = input_trait.attrs.as_slice();
        let output = quote! {
            #item
            #(#attrs)*
            pub trait #new_trait_ident #ty_generics {
                /// Type that implments the instance type
                type Impl: Lazy + #trait_ident #ty_generics + Default;
                #(#generated_methods)*
            }

        };
        Ok(output)
    } else if let Item::Struct(_) = &item {
        let res = quote! {
                #item
                #[macro_export]
        macro_rules! soroban_contract {
            ($( $macr:tt )*) => {
                struct SorobanContract;
                #[soroban_sdk::contractimpl]
                impl SorobanContract {
                    paste::item! {
                        $( $macr )*
                    }
                }
            };
        }
            };
        Ok(res)
    } else {
        Err(Error::Stream(
            quote! { compile_error!("Input must be a trait"); },
        ))
    }
}

#[cfg(test)]
mod tests {

    use std::{
        io::{Read, Write},
    };

    /// Format the given snippet. The snippet is expected to be *complete* code.
    /// When we cannot parse the given snippet, this function returns `None`.
    fn format_snippet(snippet: &str) -> String {
        let mut child = std::process::Command::new("rustfmt")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .unwrap();
        child
            .stdin
            .as_mut()
            .unwrap()
            .write_all(snippet.as_bytes())
            .map_err(p_e)
            .unwrap();
        child.wait().unwrap();
        let mut buf = String::new();
        child.stdout.unwrap().read_to_string(&mut buf).unwrap();
        buf
    }
    use super::*;

    fn equal_tokens(expected: &TokenStream, actual: &TokenStream) {
        assert_eq!(
            format_snippet(&expected.to_string()),
            format_snippet(&actual.to_string())
        );
    }

    #[test]
    fn first() {
        let input: Item = syn::parse_quote! {
            pub trait IsOwnable {
                /// Get current owner
                fn owner_get(&self) -> Option<Address>;
                fn owner_set(&mut self, new_owner: Address) -> Result<(), Error>;
                fn owner_set_two(&mut self, new_owner: Address);
            }
        };
        let result = generate(input, None);
        println!("{}", format_snippet(&result.to_string()));

        let output = quote! {
                    pub trait IsOwnable {
                        /// Get current owner
                        fn owner_get(&self) -> Option<Address>;
                        fn owner_set(&mut self, new_owner: Address) -> Result<(), Error>;
                        fn owner_set_two(&mut self, new_owner: Address);
                    }
                    pub trait Ownable {
                        type Impl: Lazy + IsOwnable + Default;
                        /// Get current owner
                        fn owner_get() -> Option<Address> {
                            Self::Impl::get_lazy().unwrap_or_default().owner_get()
                        }
                        fn owner_set(new_owner: Address) -> Result<(), Error> {
                            let mut impl_ = Self::Impl::get_lazy().unwrap_or_default();
                            let res = impl_.owner_set(new_owner)?;
                            Self::Impl::set_lazy(impl_);
                            Ok(res)
                        }
                        fn owner_set_two(new_owner: Address) {
                            let mut impl_ = Self::Impl::get_lazy().unwrap_or_default();
                            let res = impl_.owner_set_two(new_owner);
                            Self::Impl::set_lazy(impl_);
                            res
                        }
                    }
                    #[macro_export]
                    macro_rules! create_hello_world_function {
                        ($contract_name:ident) => {
                            pub fn owner_get(env: loam_sdk::soroban_sdk::Env) -> Option<Address> {
                                set_env(env);
                                $contract_name::owner_get()
                            }
                            pub fn owner_set(env: loam_sdk::soroban_sdk::Env, new_owner: Address) -> Result<(), Error>{
                                set_env(env);
                                $contract_name::owner_set(new_owner)
                            }
                            pub fn owner_set_two(env: loam_sdk::soroban_sdk::Env, new_owner: Address) {
                                set_env(env);
                                $contract_name::owner_set_two(new_owner)
                            }
                        };
        }

                };
        equal_tokens(&output, &result);
        // let impl_ = syn::parse_str::<ItemImpl>(result.as_str()).unwrap();
        // println!("{impl_:#?}");
    }

    #[test]
    fn second() {
        let input: Item = syn::parse_quote! {
            pub trait IsRiff {
                /// Get current owner
                fn riff_get(&self) -> String;
                fn riff_set(&mut self, new_riff: Address) -> Result<(), Error>;
                fn riff_set_two(&mut self, new_riff: Address);
            }
        };
        let result = generate(input, None);
        println!("{}", format_snippet(&result.to_string()));

        let output = quote! {
                    pub trait IsRiff {
                        /// Get current owner
                        fn owner_get(&self) -> Option<Address>;
                        fn owner_set(&mut self, new_owner: Address) -> Result<(), Error>;
                        fn owner_set_two(&mut self, new_owner: Address);
                    }
                    pub trait Riff {
                        type Impl: Lazy + IsRiff + Default;
                        /// Get current owner
                        fn owner_get() -> Option<Address> {
                            Self::Impl::get_lazy().unwrap_or_default().owner_get()
                        }
                        fn owner_set(new_owner: Address) -> Result<(), Error> {
                            let mut impl_ = Self::Impl::get_lazy().unwrap_or_default();
                            let res = impl_.owner_set(new_owner)?;
                            Self::Impl::set_lazy(impl_);
                            Ok(res)
                        }
                        fn owner_set_two(new_owner: Address) {
                            let mut impl_ = Self::Impl::get_lazy().unwrap_or_default();
                            let res = impl_.owner_set_two(new_owner);
                            Self::Impl::set_lazy(impl_);
                            res
                        }
                    }
                    #[macro_export]
                    macro_rules! Riff_macro {
                        ($contract_name:ident) => {
                            pub fn owner_get(env: loam_sdk::soroban_sdk::Env) -> Option<Address> {
                                set_env(env);
                                $contract_name::owner_get()
                            }
                            pub fn owner_set(env: loam_sdk::soroban_sdk::Env, new_owner: Address) -> Result<(), Error>{
                                set_env(env);
                                $contract_name::owner_set(new_owner)
                            }
                            pub fn owner_set_two(env: loam_sdk::soroban_sdk::Env, new_owner: Address) {
                                set_env(env);
                                $contract_name::owner_set_two(new_owner)
                            }
                        };
        }

                };
        equal_tokens(&output, &result);
        // let impl_ = syn::parse_str::<ItemImpl>(result.as_str()).unwrap();
        // println!("{impl_:#?}");
    }
    fn p_e(e: std::io::Error) -> std::io::Error {
        eprintln!("{e:#?}");
        e
    }
}
