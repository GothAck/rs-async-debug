use indexmap::IndexMap;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{
    DeriveInput, Error, Fields, FieldsNamed, GenericArgument, ImplGenerics, TypeGenerics, Variant,
    Visibility, WhereClause,
};

use crate::{
    common::ErrorCallSite,
    fields::{AsyncDebugFields, AsyncDebugFieldsMap},
    Result,
};

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

pub struct AsyncDebugVariant {
    variant: Variant,
    enum_debug_ident: Ident,
    fields: AsyncDebugFieldsMap,
}

impl AsyncDebugFields for AsyncDebugVariant {
    fn get_fields(&self) -> &AsyncDebugFieldsMap {
        &self.fields
    }
}

impl AsyncDebugVariant {
    fn new(variant: Variant, enum_debug_ident: Ident) -> Result<Self> {
        let fields = {
            match &variant.fields {
                Fields::Named(FieldsNamed { named: fields, .. }) => {
                    Self::convert_fields(fields.iter().collect(), None)?
                }
                Fields::Unit => {
                    return Err(Error::new_call_site(
                        "unnamed field enum variants are not supported",
                    ))
                }
                Fields::Unnamed(..) => {
                    return Err(Error::new_call_site(
                        "unnamed field enum variants are not supported",
                    ))
                }
            }
        };

        Ok(Self {
            variant,
            enum_debug_ident,
            fields,
        })
    }

    fn to_token_stream_impl_ident_body(&self) -> Result<TokenStream> {
        let ident = &self.variant.ident;
        let enum_debug_ident = &self.enum_debug_ident;
        let field_idents = self
            .fields
            .keys()
            .collect::<Vec<_>>();

        let token_stream_impl_ident_body = <Self as AsyncDebugFields>::to_token_stream_impl_ident_body(self, None)?;

        Ok(quote! {
            Self::#ident { #(#field_idents),* } => #enum_debug_ident::#ident {
                #token_stream_impl_ident_body
            },
        })
    }

    fn to_token_stream(&self) -> Result<TokenStream> {
        let ident = &self.variant.ident;

        let fields_type = self.get_fields_type();

        Ok(quote! {
            #ident {
                #fields_type
            },
        })
    }
}
