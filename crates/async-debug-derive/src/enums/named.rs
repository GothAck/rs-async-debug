use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Field, Variant};

use crate::{
    fields::{AsyncDebugFields, AsyncDebugFieldsMap},
    Result,
};

pub struct AsyncDebugVariantNamed {
    variant: Variant,
    enum_debug_ident: Ident,
    fields: AsyncDebugFieldsMap,
}

impl AsyncDebugFields for AsyncDebugVariantNamed {
    fn get_fields(&self) -> &AsyncDebugFieldsMap {
        &self.fields
    }
}

impl AsyncDebugVariantNamed {
    pub fn new(variant: Variant, enum_debug_ident: Ident, fields: Vec<Field>) -> Result<Self> {
        Ok(Self {
            fields: Self::convert_fields(fields.iter().collect(), Some(variant.ident.clone()))?,
            variant,
            enum_debug_ident,
        })
    }

    pub fn to_token_stream_impl_ident_body(&self) -> Result<TokenStream> {
        let ident = &self.variant.ident;
        let enum_debug_ident = &self.enum_debug_ident;
        let field_idents = self.fields.keys().collect::<Vec<_>>();

        let token_stream_impl_ident_body =
            <Self as AsyncDebugFields>::to_token_stream_impl_ident_body(self, None)?;

        Ok(quote! {
            Self::#ident { #(#field_idents),* } => #enum_debug_ident::#ident {
                #token_stream_impl_ident_body
            },
        })
    }

    pub fn to_token_stream(&self) -> Result<TokenStream> {
        let ident = &self.variant.ident;

        let fields_type = self.get_fields_type();

        Ok(quote! {
            #ident {
                #fields_type
            },
        })
    }
}
