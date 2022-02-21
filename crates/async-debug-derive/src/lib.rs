mod common;
mod enums;
mod struct_named;
mod zip_result;

extern crate proc_macro;

use proc_macro2::{Span, TokenStream};
use syn::{parse2, Data, DataEnum, DataStruct, DeriveInput, Error, Fields, FieldsNamed};

use self::{enums::AsyncDebugEnum, struct_named::AsyncDebugStructNamed};

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
        Data::Struct(DataStruct { fields, .. }) => match fields {
            Fields::Named(FieldsNamed { named: fields, .. }) => {
                let fields = fields.iter().cloned().collect();

                AsyncDebugStructNamed::new(&input, fields)?.to_token_stream()
            }
            Fields::Unit => Err(Error::new(
                Span::call_site(),
                "unit structs are not supported",
            )),
            Fields::Unnamed(..) => Err(Error::new(
                Span::call_site(),
                "unnamed field structs are not supported",
            )),
        },
        Data::Enum(DataEnum { variants, .. }) => {
            let variants = variants.iter().cloned().collect();

            AsyncDebugEnum::new(&input, variants)?.to_token_stream()
        }
        Data::Union(..) => Err(Error::new(Span::call_site(), "unions are not supported")),
    }
}
