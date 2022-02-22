use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{Field, Variant};

use crate::{
    fields::{AsyncDebugFieldIdent, AsyncDebugFields, AsyncDebugFieldsMap},
    Result,
};

pub struct AsyncDebugVariantUnnamed {
    variant: Variant,
    enum_ident: Ident,
    fields: AsyncDebugFieldsMap,
}

impl AsyncDebugFields for AsyncDebugVariantUnnamed {
    fn get_fields(&self) -> &AsyncDebugFieldsMap {
        &self.fields
    }
}

impl AsyncDebugVariantUnnamed {
    pub fn new(variant: Variant, enum_ident: Ident, fields: Vec<Field>) -> Result<Self> {
        Ok(Self {
            fields: Self::convert_fields(fields.iter().collect(), Some(variant.ident.clone()))?,
            variant,
            enum_ident,
        })
    }

    pub fn to_token_stream_impl_ident_body(&self, mod_ident: &Ident) -> Result<TokenStream> {
        let ident = &self.variant.ident;
        let enum_ident = &self.enum_ident;
        let field_idents = self
            .fields
            .keys()
            .map(|ident| match ident {
                AsyncDebugFieldIdent::Ident(ident) => ident.clone(),
                AsyncDebugFieldIdent::Index(index) => format_ident!("self_{}", index.index),
            })
            .collect::<Vec<_>>();

        let token_stream_impl_ident_body =
            <Self as AsyncDebugFields>::to_token_stream_impl_ident_body(self, None)?;

        Ok(quote! {
            Self::#ident ( #(#field_idents),* ) => #mod_ident::#enum_ident::#ident (
                #token_stream_impl_ident_body
            ),
        })
    }

    pub fn to_token_stream(&self) -> Result<TokenStream> {
        let ident = &self.variant.ident;

        let fields_type = self.get_fields_type();

        Ok(quote! {
            #ident (
                #fields_type
            ),
        })
    }
}
