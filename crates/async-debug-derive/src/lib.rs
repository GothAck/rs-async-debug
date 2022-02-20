extern crate proc_macro;

use std::collections::HashMap;

use bae::FromAttributes;
use indexmap::IndexMap;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse_macro_input, Data, DataStruct, DeriveInput, Expr, Field, Fields, FieldsNamed, Type,
    Visibility,
};

#[proc_macro_derive(AsyncDebug, attributes(async_debug))]
pub fn async_debug(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    async_debug_impl(input).into()
}

fn async_debug_impl(input: DeriveInput) -> TokenStream {
    match &input.data {
        Data::Struct(DataStruct { fields, .. }) => match fields {
            Fields::Named(FieldsNamed { named: fields, .. }) => {
                let fields = fields.iter().cloned().collect();

                AsyncDebugStructNamed::new(&input, fields).to_token_stream()
            }
            Fields::Unit => {
                panic!("unit structs are not supported");
            }
            Fields::Unnamed(..) => {
                panic!("unnamed field structs are not supported");
            }
        },
        Data::Enum(..) => {
            panic!("enums are not supported");
        }
        Data::Union(..) => {
            panic!("unions are not supported");
        }
    }
}

struct AsyncDebugStructNamed {
    vis: Visibility,
    ident: Ident,
    fields: Vec<Field>,
}

type StructGenerics = IndexMap<Ident, (Ident, TokenStream)>;
type FieldsTs = HashMap<Ident, TokenStream>;

impl AsyncDebugStructNamed {
    pub fn new(input: &DeriveInput, fields: Vec<Field>) -> Self {
        Self {
            vis: input.vis.clone(),
            ident: input.ident.clone(),
            fields,
        }
    }

    fn get_fields(&self) -> (StructGenerics, FieldsTs) {
        let mut struct_generics: StructGenerics = IndexMap::new();
        let mut fields_ts = HashMap::new();
        // let mut fields_ty = HashMap::new();
        for field in &self.fields {
            let Field {
                attrs,
                vis: _,
                ident,
                ty,
                ..
            } = field;

            let ident = ident.as_ref().unwrap();

            struct_generics.insert(
                ident.clone(),
                (format_ident!("T_{}", ident), quote! { #ty }),
            );
            let mut custom_type = false;

            if let Some(debug_attribute) = AsyncDebug::try_from_attributes(attrs).unwrap() {
                if let Some(ty) = debug_attribute.ty {
                    // fields_ty.insert(ident.clone(), ty.clone());
                    struct_generics.insert(
                        ident.clone(),
                        (format_ident!("T_{}", ident), quote! { #ty }),
                    );
                    custom_type = true;
                } else {
                    // fields_ty.insert(ident.clone(), ty.clone());
                }

                let mut field_ts = quote! { self.#ident };

                if let Some(parse_expr) = debug_attribute.parse {
                    // async_fields.push(ident.clone());
                    field_ts = quote! { #parse_expr(&#field_ts).await };
                }

                if debug_attribute.copy.is_some() {
                    // TODO: Exclusive
                    field_ts = quote! { *#field_ts };
                } else if debug_attribute.clone.is_some() {
                    // TODO: Exclusive
                    field_ts = quote! { #field_ts.clone() };
                }

                fields_ts.insert(ident.clone(), field_ts);
            }

            if let Some((ident, (sg_ident, ty))) = struct_generics.remove_entry(ident) {
                if custom_type {
                    struct_generics.insert(ident, (sg_ident, ty));
                } else {
                    struct_generics.insert(ident, (sg_ident, quote! { &#ty }));
                }
            }
        }

        (struct_generics, fields_ts)
    }

    fn get_generics(&self, struct_generics: StructGenerics) -> (Vec<Ident>, Vec<TokenStream>) {
        struct_generics
            .values()
            .cloned()
            .map(|(a, b)| (a, b))
            .unzip()
    }

    fn get_async_fields(&self) -> Vec<TokenStream> {
        self.fields
            .iter()
            .map(|field| {
                let ident = field.ident.as_ref().unwrap();
                let ident_ty = format_ident!("T_{}", ident);

                quote! { #ident: #ident_ty, }
            })
            .collect()
    }

    fn get_assign_fields(&self, fields_ts: FieldsTs) -> Vec<TokenStream> {
        self.fields
            .iter()
            .map(|field| {
                let ident = field.ident.as_ref().unwrap();

                if let Some(ts) = fields_ts.get(ident) {
                    quote! { #ident: #ts, }
                } else {
                    quote! { #ident: &self.#ident, }
                }
            })
            .collect()
    }
}

impl ToTokens for AsyncDebugStructNamed {
    fn to_tokens(&self, ts: &mut TokenStream) {
        let (struct_generics, fields_ts) = self.get_fields();
        let (struct_generic_names, struct_generic_types) = self.get_generics(struct_generics);
        let struct_async_fields = self.get_async_fields();
        let assign_fields = self.get_assign_fields(fields_ts);

        let vis = &self.vis;
        let ident = &self.ident;
        let debug_struct_ident = format_ident!("{}Debug", self.ident);

        ts.extend(
            quote! {
                impl async_debug::AsyncDebug for #ident {}

                impl #ident {
                    #vis async fn async_debug(&self) -> #debug_struct_ident<#(#struct_generic_types),*> {
                        #debug_struct_ident {
                            #(#assign_fields)*
                        }
                    }
                }

                #[derive(Debug)]
                #[allow(dead_code)]
                #[allow(non_camel_case_types)]
                #vis struct #debug_struct_ident<#(#struct_generic_names),*> {
                    #(#struct_async_fields)*
                }
            }
        );
    }
}

#[derive(FromAttributes)]
struct AsyncDebug {
    parse: Option<Expr>,
    clone: Option<()>,
    copy: Option<()>,
    ty: Option<Type>,
}
