use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{DeriveInput, Field, ImplGenerics, TypeGenerics, Visibility, WhereClause};

use crate::{
    fields::{AsyncDebugFields, AsyncDebugFieldsMap},
    Result,
};

pub struct AsyncDebugStructUnnamed<'a> {
    vis: Visibility,
    ident: Ident,
    generics_impl: ImplGenerics<'a>,
    generics_ty: TypeGenerics<'a>,
    where_clause: Option<&'a WhereClause>,
    fields: AsyncDebugFieldsMap,
}

impl<'a> AsyncDebugFields for AsyncDebugStructUnnamed<'a> {
    fn get_fields(&self) -> &AsyncDebugFieldsMap {
        &self.fields
    }
}

impl<'a> AsyncDebugStructUnnamed<'a> {
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
        let token_stream_impl_ident_body = self.to_token_stream_impl_ident_body(Some(quote! { self. }))?;

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
                    #debug_struct_ident (
                        #token_stream_impl_ident_body
                    )
                }
            }
        };

        let ts_struct = quote! {
            #[derive(Debug)]
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            #[automatically_derived]
            #vis struct #debug_struct_ident <#(#new_generics_names),*>
            (
                #fields_type
            );
        };

        Ok(quote! {
            #ts_impl_async_debug
            #ts_impl_ident
            #ts_struct
        })
    }
}
