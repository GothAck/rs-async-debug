mod common;
mod enums;
mod structs;
mod zip_result;

extern crate proc_macro;

use proc_macro2::{Span, TokenStream};
use syn::{parse2, Data, DataEnum, DataStruct, DeriveInput, Error};

use self::{enums::AsyncDebugEnum, structs::AsyncDebugStruct};

type Result<T> = std::result::Result<T, Error>;

#[proc_macro_derive(AsyncDebug, attributes(async_debug))]
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
