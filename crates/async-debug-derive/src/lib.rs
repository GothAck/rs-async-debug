extern crate proc_macro;

use std::collections::HashMap;

use bae::FromAttributes;
use proc_macro2::TokenStream;
use quote::{quote, format_ident};

use syn::{parse_macro_input, Expr, DeriveInput, Data, DataStruct, Fields, FieldsNamed, punctuated::Punctuated, Field, token::Comma, Type};

#[proc_macro_derive(AsyncDebug, attributes(async_debug))]
pub fn async_debug(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    async_debug_impl(input).into()
}

fn async_debug_impl(input: DeriveInput) -> TokenStream {
    match &input.data {
        Data::Struct(DataStruct {fields, ..}) => match fields {
            Fields::Named(FieldsNamed { named: fields, .. }) => {
                async_debug_struct_named(&input, fields)
            }
            Fields::Unit => {
                panic!("unit structs are not supported");
            }
            Fields::Unnamed(..) => {
                panic!("unnamed field structs are not supported");
            }
        }
        Data::Enum(..) => {
            panic!("enums are not supported");
        }
        Data::Union(..) => {
            panic!("unions are not supported");
        }
    }
}

fn async_debug_struct_named(input: &DeriveInput, fields: &Punctuated<Field, Comma>) -> TokenStream {
    let DeriveInput {
        attrs: _,
        vis,
        ident,
        generics: _,
        data: _
    } = input;

    let debug_struct_ident = format_ident!("{}Debug", ident);

    // let mut async_fields = vec![];
    let mut fields_ts = HashMap::new();
    let mut fields_ty = HashMap::new();
    for field in fields {
        let Field {
            attrs,
            vis: _,
            ident,
            ty,
            ..
        } = field;

        if let Some(debug_attribute) = AsyncDebug::try_from_attributes(attrs).unwrap() {
            let ident = ident.as_ref().unwrap();

            if let Some(ty) = debug_attribute.ty {
                fields_ty.insert(ident.clone(), ty);
            } else {
                fields_ty.insert(ident.clone(), ty.clone());
            }

            if debug_attribute.copy.is_some() {
                // TODO: Exclusive
                fields_ts.insert(ident.clone(), quote! { *self.#ident });
            } else if debug_attribute.clone.is_some() {
                // TODO: Exclusive
                fields_ts.insert(ident.clone(), quote! { self.#ident.clone() });
            } else if let Some(parse_expr) = debug_attribute.parse {
                // async_fields.push(ident.clone());
                fields_ts.insert(ident.clone(),quote! { #parse_expr(&self.#ident).await });
            }
        }
    }

    let struct_async_fields = fields.iter()
        .map(|field| {
            let ident = field.ident.as_ref().unwrap();
            let ty = &fields_ty.get(ident).unwrap_or(&field.ty);

            quote! { #ident: #ty, }
        });

    let assign_fields = fields.iter()
        .map(|field| {
            let ident = field.ident.as_ref().unwrap();

            if let Some(ts) = fields_ts.get(ident) {
                quote! { #ident: #ts, }
            } else {
                quote! { #ident: self.#ident, }
            }
        });

    quote! {
        #[async_debug::async_trait]
        impl async_debug::AsyncDebug<#debug_struct_ident> for #ident {
            async fn async_debug(&self) -> #debug_struct_ident {
                #debug_struct_ident {
                    #(#assign_fields)*
                }
            }
        }
        #[derive(Debug)]
        #[allow(dead_code)]
        #vis struct #debug_struct_ident {
            #(#struct_async_fields)*
        }
    }
}

#[derive(FromAttributes)]
struct AsyncDebug {
    parse: Option<Expr>,
    clone: Option<()>,
    copy: Option<()>,
    ty: Option<Type>,
}
