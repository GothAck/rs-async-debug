#![warn(rustdoc::missing_crate_level_docs)]
#![warn(missing_docs)]

//! Derive macro for [async-debug](https://crates.io/crates/async-debug)

mod common;
mod enums;
mod fields;
mod structs;
#[cfg(test)]
mod tests;
mod zip_result;

extern crate proc_macro;

use proc_macro2::{Span, TokenStream};
use syn::{parse2, Data, DataEnum, DataStruct, DeriveInput, Error};

use self::{enums::AsyncDebugEnum, structs::AsyncDebugStruct};

type Result<T> = std::result::Result<T, Error>;

#[proc_macro_derive(AsyncDebug, attributes(async_debug))]
/// `AsyncDebug` proc macro
///
/// This macro will use the `#[async_debug()]` attribute on properties of the struct or enum.
/// Attribute arguments can include:
///   async_call = some_function  - Call this async function to render the value
///   clone                       - Call `.clone()` on the value (exclusive of copy)
///   copy                        - Dereference the value to take a copy (exclusive of clone)
///   ty = SomeType               - Use this type as the type of thie property on the generated `*AsyncDebug` struct/enum.
pub fn async_debug(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match async_debug_impl(input.into()) {
        Ok(output) => output.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn async_debug_impl(input: TokenStream) -> Result<TokenStream> {
    let input: DeriveInput = parse2(input)?;

    match &input.data {
        Data::Struct(DataStruct { fields, .. }) => {
            AsyncDebugStruct::new(&input, fields)?.to_token_stream()
        }
        Data::Enum(DataEnum { variants, .. }) => {
            let variants = variants.iter().cloned().collect();

            AsyncDebugEnum::new(&input, variants)?.to_token_stream()
        }
        Data::Union(..) => Err(Error::new(Span::call_site(), "unions are not supported")),
    }
}
