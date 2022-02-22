mod named;
mod unnamed;

use indexmap::IndexMap;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{
    DeriveInput, Error, GenericArgument, ImplGenerics, TypeGenerics, Variant,
    Visibility, WhereClause, Fields, FieldsNamed, FieldsUnnamed,
};

use crate::{
    fields::AsyncDebugFields,
    Result, common::ErrorCallSite,
};

use self::{named::AsyncDebugVariantNamed, unnamed::AsyncDebugVariantUnnamed};

pub struct AsyncDebugEnum<'a> {
    vis: Visibility,
    ident: Ident,
    debug_ident: Ident,
    generics_impl: ImplGenerics<'a>,
    generics_ty: TypeGenerics<'a>,
    where_clause: Option<&'a WhereClause>,
    variants: IndexMap<Ident, AsyncDebugVariant>,
}

impl<'a> AsyncDebugEnum<'a> {
    pub fn new(input: &'a DeriveInput, variants: Vec<Variant>) -> Result<Self> {
        let debug_ident = format_ident!("{}Debug", input.ident);

        let variants = variants
            .iter()
            .map(|variant| {
                let ident = &variant.ident;

                Ok((
                    ident.clone(),
                    AsyncDebugVariant::new(variant.clone(), debug_ident.clone())?,
                ))
            })
            .collect::<Result<IndexMap<_, _>>>()?;

        let (generics_impl, generics_ty, where_clause) = input.generics.split_for_impl();

        Ok(Self {
            vis: input.vis.clone(),
            ident: input.ident.clone(),
            debug_ident,
            generics_impl,
            generics_ty,
            where_clause,
            variants,
        })
    }

    fn get_new_generics(&self) -> Result<(Vec<GenericArgument>, Vec<GenericArgument>)> {
        let mut names = Vec::new();
        let mut types = Vec::new();

        let iter = self
            .variants
            .values()
            .map(|variant| variant.get_new_generics())
            .collect::<Result<Vec<_>>>()?;

        for (variant_names, variant_types) in iter {
            names.extend(variant_names);
            types.extend(variant_types);
        }

        Ok((names, types))
    }

    fn get_variants(&self) -> Result<TokenStream> {
        self.variants
            .values()
            .map(|variant| variant.to_token_stream())
            .collect()
    }

    pub fn to_token_stream_impl_ident_bodies(&self) -> Result<TokenStream> {
        self.variants
            .values()
            .map(|variant| variant.to_token_stream_impl_ident_body())
            .collect()
    }

    pub fn to_token_stream(&self) -> Result<TokenStream> {
        let (new_generics_names, new_generics) = self.get_new_generics()?;
        let variants = self.get_variants()?;

        let vis = &self.vis;
        let ident = &self.ident;
        let debug_ident = &self.debug_ident;

        let generics_impl = &self.generics_impl;
        let generics_ty = &self.generics_ty;
        let where_clause = &self.where_clause;

        let ts_impl_async_debug = quote! {
            impl #generics_impl AsyncDebug for #ident #generics_ty #where_clause {}
        };

        let token_stream_impl_ident_bodies = self.to_token_stream_impl_ident_bodies()?;

        let ts_impl_ident = quote! {
            #[automatically_derived]
            impl #generics_impl #ident #generics_ty #where_clause {
                #vis async fn async_debug (&self) -> #debug_ident <#(#new_generics),*>
                #where_clause
                {
                    match self {
                        #token_stream_impl_ident_bodies
                    }
                }
            }
        };

        let ts_enum = quote! {
            #[derive(Debug)]
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            #[automatically_derived]
            #vis enum #debug_ident <#(#new_generics_names),*>
            {
                #variants
            }
        };

        Ok(quote! {
            #ts_impl_async_debug
            #ts_impl_ident
            #ts_enum
        })
    }
}

enum AsyncDebugVariant {
    Named(AsyncDebugVariantNamed),
    Unit,
    Unnamed(AsyncDebugVariantUnnamed),
}

impl AsyncDebugVariant {
    pub fn new(variant: Variant, enum_debug_ident: Ident) -> Result<Self> {
        Ok(match &variant.fields {
            Fields::Named(FieldsNamed { named: fields, .. }) => {
                let fields = fields.iter().cloned().collect::<Vec<_>>();

                Self::Named(AsyncDebugVariantNamed::new(variant, enum_debug_ident, fields)?)
            }
            Fields::Unit => Self::Unit,
            Fields::Unnamed(FieldsUnnamed {
                unnamed: fields, ..
            }) => {
                let fields = fields.iter().cloned().collect::<Vec<_>>();

                Self::Unnamed(AsyncDebugVariantUnnamed::new(
                    variant,
                    enum_debug_ident,
                    fields,
                )?)
            },
        })
    }

    fn get_new_generics(&self) -> Result<(Vec<GenericArgument>, Vec<GenericArgument>)> {
        match self {
            Self::Named(named) => named.get_new_generics(),
            Self::Unit => Err(Error::new_call_site("unreachable")),
            Self::Unnamed(unnamed) => unnamed.get_new_generics(),
        }
    }

    fn to_token_stream_impl_ident_body(&self) -> Result<TokenStream> {
        match self {
            Self::Named(named) => named.to_token_stream_impl_ident_body(),
            Self::Unit => Err(Error::new_call_site("unreachable")),
            Self::Unnamed(unnamed) => unnamed.to_token_stream_impl_ident_body(),
        }
    }

    pub fn to_token_stream(&self) -> Result<TokenStream> {
        match self {
            Self::Named(named) => named.to_token_stream(),
            Self::Unit => Err(Error::new_call_site("unit structs are not supported")),
            Self::Unnamed(unnamed) => unnamed.to_token_stream(),
        }
    }
}
