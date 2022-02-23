use std::num::TryFromIntError;

use indexmap::IndexMap;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, IdentFragment, ToTokens};
use syn::{parse2, spanned::Spanned, Error, Field, GenericArgument, Index, Type};

use crate::{
    common::{attr_prop::AsyncDebug, prelude::*},
    zip_result::ZipResult,
};

pub type AsyncDebugFieldsMap = IndexMap<AsyncDebugFieldIdent, AsyncDebugField>;

pub trait AsyncDebugFields {
    fn get_fields(&self) -> &AsyncDebugFieldsMap;

    fn convert_fields(
        fields: Vec<&Field>,
        variant_ident: Option<Ident>,
    ) -> Result<AsyncDebugFieldsMap> {
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

    fn get_new_generics(&self) -> Result<(Vec<GenericArgument>, Vec<GenericArgument>)> {
        let (names, types): (Vec<GenericArgument>, Vec<Type>) = self
            .get_fields()
            .values()
            .filter(|field| field.attr.skip.is_none())
            .map(|field| field.generic_argument().zip_result(field.ty()))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .unzip();

        let types = types
            .into_iter()
            .map(|ts| parse2(ts.to_token_stream()))
            .collect::<Result<Vec<_>>>()?;

        Ok((names, types))
    }

    fn get_fields_type(&self) -> TokenStream {
        self.get_fields()
            .values()
            .filter(|field| field.attr.skip.is_none())
            .map(|field| {
                let ident = &field.ident;
                let generic_argument = field.generic_argument_ident();

                let vis = {
                    if field.variant_ident.is_none() {
                        Some(quote! { pub(super) })
                    } else {
                        None
                    }
                };

                match ident {
                    AsyncDebugFieldIdent::Ident(ident) => {
                        quote! { #vis #ident: #generic_argument, }
                    }
                    AsyncDebugFieldIdent::Index(_) => quote! { #vis #generic_argument, },
                }
            })
            .collect()
    }

    fn to_token_stream_impl_ident_body(&self, prefix: Option<TokenStream>) -> Result<TokenStream> {
        self.get_fields()
            .values()
            .filter(|field| field.attr.skip.is_none())
            .map(|field| field.to_token_stream(prefix.clone()))
            .collect()
    }
}

pub struct AsyncDebugField {
    pub field: Field,
    pub variant_ident: Option<Ident>,
    pub ident: AsyncDebugFieldIdent,
    pub attr: AsyncDebug,
}

impl AsyncDebugField {
    pub fn new(field: Field, variant_ident: Option<Ident>, index: usize) -> Result<Self> {
        let ident = field
            .ident
            .clone()
            .map(|ident| -> Result<_> { Ok(AsyncDebugFieldIdent::Ident(ident)) })
            .unwrap_or_else(|| {
                let index = index
                    .try_into()
                    .map_err(|e: TryFromIntError| Error::new(field.span(), e.to_string()))?;
                Ok(AsyncDebugFieldIdent::Index(Index {
                    index,
                    span: field.span(),
                }))
            })?;

        let attr = AsyncDebug::try_from_attributes(&field.attrs)?.unwrap_or_default();

        attr.validate(&field.ident)?;

        Ok(Self {
            field,
            variant_ident,
            ident,
            attr,
        })
    }

    pub fn ty(&self) -> Result<Type> {
        if let Some(ty) = &self.attr.ty {
            return Ok(ty.clone());
        }
        let ty = &self.field.ty;
        parse2(quote! { &#ty })
    }

    pub fn custom_type(&self) -> bool {
        self.attr.ty.is_some()
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
        let ts_ident = {
            match ident {
                AsyncDebugFieldIdent::Ident(_) => ident.clone(),
                AsyncDebugFieldIdent::Index(index) => {
                    if self.variant_ident.is_some() {
                        AsyncDebugFieldIdent::Ident(format_ident!("self_{}", index.index))
                    } else {
                        ident.clone()
                    }
                }
            }
        };

        let mut ts = quote! { #prefix #ts_ident };

        if let Some(async_call) = &self.attr.async_call {
            ts = quote! { #async_call(&#ts).await };
        }

        if self.attr.copy.is_some() {
            ts = quote! { *#ts };
        } else if self.attr.clone.is_some() {
            ts = quote! { #ts.clone() }
        }

        if !self.custom_type() {
            ts = quote! { &#ts };
        }

        Ok(match ident {
            AsyncDebugFieldIdent::Ident(ident) => quote! { #ident: #ts, },
            AsyncDebugFieldIdent::Index(_) => quote! { #ts, },
        })
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
