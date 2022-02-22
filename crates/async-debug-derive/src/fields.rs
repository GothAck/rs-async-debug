use std::num::TryFromIntError;

use indexmap::IndexMap;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, format_ident, ToTokens, IdentFragment};
use syn::{Field, Error, Type, parse2, GenericArgument, Index, spanned::Spanned};

use crate::{common::AsyncDebug, Result};

pub trait AsyncDebugFields {
    fn get_fields(fields: Vec<&Field>, variant_ident: Option<Ident>) -> Result<IndexMap<AsyncDebugFieldIdent, AsyncDebugField>> {
        fields
            .into_iter()
            .cloned()
            .enumerate()
            .map(|(index, field)| {
                AsyncDebugField::new(field, variant_ident.clone(), index)
                    .map(|field| (field.ident.clone(), field))
            })
            .collect::<Result<IndexMap<_, _>>>()
    }
}

pub struct AsyncDebugField {
    pub field: Field,
    pub variant_ident: Option<Ident>,
    pub ident: AsyncDebugFieldIdent,
    pub async_debug: Option<AsyncDebug>,
}

impl AsyncDebugField {
    pub fn new(field: Field, variant_ident: Option<Ident>, index: usize) -> Result<Self> {
        let ident = field
            .ident
            .clone()
            .map(|ident| -> Result<_> {
                Ok(AsyncDebugFieldIdent::Ident(ident))
            })
            .unwrap_or_else(|| {
                let index = index.try_into()
                    .map_err(|e: TryFromIntError| {
                        Error::new(field.span(), e.to_string())
                    })?;
                Ok(AsyncDebugFieldIdent::Index(Index { index, span: field.span() }))
            })?;

        Ok(Self {
            async_debug: AsyncDebug::try_from_attributes(&field.attrs)?,
            field,
            variant_ident,
            ident,
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
        if let Some(variant_ident) = &self.variant_ident {
            format_ident!("T_AsyncDebug_{}_{}", variant_ident, self.ident)
        } else {
            format_ident!("T_AsyncDebug_{}", self.ident)
        }
    }

    pub fn generic_argument(&self) -> Result<GenericArgument> {
        parse2(self.generic_argument_ident().to_token_stream())
    }

    pub fn to_token_stream(&self, prefix: Option<TokenStream>) -> Result<TokenStream> {
        let ident = &self.ident;

        let mut ts = quote! { #prefix #ident };

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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum AsyncDebugFieldIdent {
    Ident(Ident),
    Index(Index),
}

impl IdentFragment for AsyncDebugFieldIdent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Ident(ident) => <Ident as IdentFragment>::fmt(ident, f),
            Self::Index(index) => <Index as IdentFragment>::fmt(index, f),
        }
    }
}

impl ToTokens for AsyncDebugFieldIdent {
    fn to_tokens(&self, ts: &mut TokenStream) {
        match self {
            Self::Ident(ident) => ident.to_tokens(ts),
            Self::Index(index) => index.to_tokens(ts),
        }
    }
}

impl std::fmt::Display for AsyncDebugFieldIdent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Ident(ident) => f.write_str(&ident.to_string()),
            Self::Index(index) => f.write_str(&index.index.to_string()),
        }
    }
}
