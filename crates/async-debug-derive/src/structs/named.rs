use indexmap::IndexMap;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse2, DeriveInput, Field, GenericArgument, ImplGenerics, Type, TypeGenerics,
    Visibility, WhereClause,
};

use crate::{fields::{AsyncDebugField, AsyncDebugFieldIdent}, zip_result::ZipResult, Result};

pub struct AsyncDebugStructNamed<'a> {
    vis: Visibility,
    ident: Ident,
    generics_impl: ImplGenerics<'a>,
    generics_ty: TypeGenerics<'a>,
    where_clause: Option<&'a WhereClause>,
    fields: IndexMap<AsyncDebugFieldIdent, AsyncDebugField>,
}

impl<'a> AsyncDebugStructNamed<'a> {
    pub fn new(input: &'a DeriveInput, fields: Vec<&Field>) -> Result<Self> {
        let (generics_impl, generics_ty, where_clause) = input.generics.split_for_impl();

        let fields = fields
            .into_iter()
            .cloned()
            .enumerate()
            .map(|(index, field)| {
                let field = AsyncDebugField::new(field, None, index)?;
                Ok((field.ident.clone(), field))
            })
            .collect::<Result<IndexMap<_, _>>>()?;

        Ok(Self {
            vis: input.vis.clone(),
            ident: input.ident.clone(),
            generics_impl,
            generics_ty,
            where_clause,
            fields,
        })
    }

    fn get_new_generics(&self) -> Result<(Vec<GenericArgument>, Vec<GenericArgument>)> {
        let (names, types): (Vec<GenericArgument>, Vec<Type>) = self
            .fields
            .values()
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

    fn get_fields_type(&self) -> Vec<TokenStream> {
        self.fields
            .values()
            .map(|field| {
                let ident = &field.ident;
                let generic_argument = field.generic_argument_ident();

                quote! { #ident: #generic_argument, }
            })
            .collect()
    }

    fn get_fields_assign(&self) -> Result<Vec<TokenStream>> {
        self.fields
            .values()
            .map(|field| field.to_token_stream(Some(quote! { self. })))
            .collect()
    }

    pub fn to_token_stream(&self) -> Result<TokenStream> {
        let (new_generics_names, new_generics) = self.get_new_generics()?;
        let fields_type = self.get_fields_type();
        let fields_assign = self.get_fields_assign()?;

        let vis = &self.vis;
        let ident = &self.ident;

        let generics_impl = &self.generics_impl;
        let generics_ty = &self.generics_ty;
        let where_clause = &self.where_clause;

        let debug_struct_ident = format_ident!("{}Debug", self.ident);

        let ts_impl_async_debug = quote! {
            impl #generics_impl AsyncDebug for #ident #generics_ty #where_clause {}
        };

        let ts_impl_ident = quote! {
            #[automatically_derived]
            impl #generics_impl #ident #generics_ty #where_clause {
                #vis async fn async_debug (&self) -> #debug_struct_ident <#(#new_generics),*>
                #where_clause
                {
                    #debug_struct_ident {
                        #(#fields_assign)*
                    }
                }
            }
        };

        let ts_struct = quote! {
            #[derive(Debug)]
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            #[automatically_derived]
            #vis struct #debug_struct_ident <#(#new_generics_names),*>
            {
                #(#fields_type)*
            }
        };

        Ok(quote! {
            #ts_impl_async_debug
            #ts_impl_ident
            #ts_struct
        })
    }
}
