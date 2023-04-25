use proc_macro2::TokenStream;
use quote::quote;
use syn::{AttributeArgs, Item};

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

#[allow(clippy::unnecessary_wraps)]
fn inner_generate(item: Item, _attr: Option<AttributeArgs>) -> Result<TokenStream, Error> {
    Ok(match &item {
        Item::Const(_) => todo!(),
        Item::Enum(_) => todo!(),
        Item::ExternCrate(_) => todo!(),
        Item::Fn(_) => todo!(),
        Item::ForeignMod(_) => todo!(),
        Item::Impl(impl_) => {
            let name = &impl_.self_ty;
            quote! {
                #impl_
                mod test {
                    pub fn #name() {
                        todo!("It worked")
                    }
                }
            }
        }
        Item::Macro(_) => todo!(),
        Item::Macro2(_) => todo!(),
        Item::Mod(_) => todo!(),
        Item::Static(_) => todo!(),
        Item::Struct(strukt) => {
            let name = &strukt.ident;
            quote! {
                #strukt
                mod test {
                    pub fn #name() {
                        todo!("It worked")
                    }
                }
            }
        }
        Item::Trait(_) => todo!(),
        Item::TraitAlias(_) => todo!(),
        Item::Type(_) => todo!(),
        Item::Union(_) => todo!(),
        Item::Use(_) => todo!(),
        Item::Verbatim(_) => todo!(),
        _ => todo!(),
    })
}

#[cfg(test)]
mod tests {

    use std::{
        io::{Read, Write},
        process::Stdio,
    };

    use super::*;

    /// Format the given snippet. The snippet is expected to be *complete* code.
    /// When we cannot parse the given snippet, this function returns `None`.
    fn format_snippet(snippet: &str) -> String {
        let mut child = std::process::Command::new("rustfmt")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
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

    fn equal_tokens(expected: &TokenStream, actual: &TokenStream) {
        assert_eq!(
            format_snippet(&expected.to_string()),
            format_snippet(&actual.to_string())
        );
    }

    #[test]
    fn first() {
        let input: Item = syn::parse_quote! {
           struct Foo;
        };
        let result = generate(input, None);

        let output = quote! {
            struct Foo;
            mod test {
                pub fn Foo() {
                    todo!("It worked")
                }
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
