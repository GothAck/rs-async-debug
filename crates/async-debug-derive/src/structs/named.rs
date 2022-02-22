use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{DeriveInput, Field, ImplGenerics, TypeGenerics, Visibility, WhereClause};

use crate::{
    common::{prelude::*, AsyncDebugCommon},
    fields::{AsyncDebugFields, AsyncDebugFieldsMap},
};

pub struct AsyncDebugStructNamed<'a> {
    vis: Visibility,
    ident: Ident,
    generics_impl: ImplGenerics<'a>,
    generics_ty: TypeGenerics<'a>,
    where_clause: Option<&'a WhereClause>,
    fields: AsyncDebugFieldsMap,
}

impl<'a> AsyncDebugCommon for AsyncDebugStructNamed<'a> {}

impl<'a> AsyncDebugFields for AsyncDebugStructNamed<'a> {
    fn get_fields(&self) -> &AsyncDebugFieldsMap {
        &self.fields
    }
}

impl<'a> AsyncDebugStructNamed<'a> {
    pub fn new(input: &'a DeriveInput, fields: Vec<&Field>) -> Result<Self> {
        let (generics_impl, generics_ty, where_clause) = input.generics.split_for_impl();

        let fields = Self::convert_fields(fields, None)?;

        Ok(Self {
            vis: input.vis.clone(),
            ident: input.ident.clone(),
            generics_impl,
            generics_ty,
            where_clause,
            fields,
        })
    }

    pub fn to_token_stream(&self) -> Result<TokenStream> {
        let (new_generics_names, new_generics) = self.get_new_generics()?;
        let fields_type = self.get_fields_type();
        let token_stream_impl_ident_body =
            self.to_token_stream_impl_ident_body(Some(quote! { self. }))?;

        let vis = &self.vis;
        let ident = &self.ident;

        let generics_impl = &self.generics_impl;
        let generics_ty = &self.generics_ty;
        let where_clause = &self.where_clause;

        let async_debug_mod_ident = Self::get_async_debug_mod_ident(ident);

        let ts_impl_async_debug = quote! {
            impl #generics_impl AsyncDebug for #ident #generics_ty #where_clause {}
        };

        let ts_impl_ident = quote! {
            #[automatically_derived]
            impl #generics_impl #ident #generics_ty #where_clause {
                #vis async fn async_debug (&self) -> #async_debug_mod_ident::#ident <#(#new_generics),*>
                #where_clause
                {
                    #async_debug_mod_ident::#ident {
                        #token_stream_impl_ident_body
                    }
                }
            }
        };

        let ts_struct = quote! {
            #vis mod #async_debug_mod_ident {
                use super::*;

                #[derive(Debug)]
                #[allow(dead_code)]
                #[allow(non_camel_case_types)]
                #[automatically_derived]
                pub struct #ident <#(#new_generics_names),*>
                {
                    #fields_type
                }
            }
        };

        Ok(quote! {
            #ts_impl_async_debug
            #ts_impl_ident
            #ts_struct
        })
    }
}
