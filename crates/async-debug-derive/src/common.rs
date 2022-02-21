use bae::FromAttributes;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{parse2, Error, Expr, Field, GenericArgument, Type};

use crate::Result;

#[derive(FromAttributes)]
pub struct AsyncDebug {
    parse: Option<Expr>,
    clone: Option<()>,
    copy: Option<()>,
    ty: Option<Type>,
}

pub struct AsyncDebugField {
    pub ident: Ident,
    pub field: Field,
    pub async_debug: Option<AsyncDebug>,
}

impl AsyncDebugField {
    pub fn new(field: Field) -> Result<Self> {
        let ident = field
            .ident
            .clone()
            .ok_or_else(|| Error::new(Span::call_site(), "Missing ident"))?;
        let async_debug = AsyncDebug::try_from_attributes(&field.attrs)?;

        Ok(Self {
            ident,
            field,
            async_debug,
        })
    }

    pub fn ty(&self) -> Result<Type> {
        if let Some(async_debug) = &self.async_debug {
            if let Some(ty) = &async_debug.ty {
                return Ok(ty.clone());
            }
        }
        let ty = &self.field.ty;
        parse2(quote! { &#ty })
    }

    pub fn custom_type(&self) -> bool {
        if let Some(async_debug) = &self.async_debug {
            return async_debug.ty.is_some();
        }
        false
    }

    pub fn generic_argument_ident(&self) -> Ident {
        format_ident!("T_AsyncDebug_{}", self.ident)
    }

    pub fn generic_argument(&self) -> Result<GenericArgument> {
        parse2(self.generic_argument_ident().to_token_stream())
    }

    pub fn to_token_stream(&self) -> Result<TokenStream> {
        let ident = &self.ident;

        let mut ts = quote! { self.#ident };

        if let Some(async_debug) = &self.async_debug {
            if let Some(parse) = &async_debug.parse {
                ts = quote! { #parse(&#ts).await };
            }

            if async_debug.copy.is_some() && async_debug.clone.is_some() {
                return Err(Error::new_spanned(ident, "copy and clone are exclusive"));
            }

            if async_debug.copy.is_some() {
                ts = quote! { *#ts };
            } else if async_debug.clone.is_some() {
                ts = quote! { #ts.clone() }
            }
        }

        if !self.custom_type() {
            ts = quote! { &#ts };
        }

        Ok(quote! { #ident: #ts, })
    }
}
