extern crate proc_macro;

use std::collections::HashMap;

use bae::FromAttributes;
use indexmap::IndexMap;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use syn::{
    parse2, Data, DataStruct, DeriveInput, Error, Expr, Field, Fields, FieldsNamed, Type,
    Visibility,
};

#[proc_macro_derive(AsyncDebug, attributes(async_debug))]
pub fn async_debug(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match async_debug_impl(input.into()) {
        Ok(output) => output.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn async_debug_impl(input: TokenStream) -> Result<TokenStream, Error> {
    let input: DeriveInput = parse2(input)?;

    match &input.data {
        Data::Struct(DataStruct { fields, .. }) => match fields {
            Fields::Named(FieldsNamed { named: fields, .. }) => {
                let fields = fields.iter().cloned().collect();

                AsyncDebugStructNamed::new(&input, fields)?.to_token_stream()
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
    fields: IndexMap<Ident, Field>,
}

type StructGenerics = IndexMap<Ident, (Ident, TokenStream)>;
type FieldsTs = HashMap<Ident, TokenStream>;

impl AsyncDebugStructNamed {
    pub fn new(input: &DeriveInput, fields: Vec<Field>) -> Result<Self, Error> {
        let fields = fields
            .iter()
            .map(|field| {
                let ident = field
                    .ident
                    .clone()
                    .ok_or_else(|| Error::new(Span::call_site(), "Missing field ident"))?;

                Ok((ident, field.clone()))
            })
            .collect::<Result<IndexMap<_, _>, Error>>()?;

        Ok(Self {
            vis: input.vis.clone(),
            ident: input.ident.clone(),
            fields,
        })
    }

    fn get_fields(&self) -> Result<(StructGenerics, FieldsTs), Error> {
        let mut struct_generics: StructGenerics = IndexMap::new();
        let mut fields_ts = HashMap::new();
        for (ident, field) in &self.fields {
            let Field { attrs, ty, .. } = field;

            struct_generics.insert(
                ident.clone(),
                (format_ident!("T_{}", ident), quote! { #ty }),
            );
            let mut custom_type = false;

            if let Some(debug_attribute) = AsyncDebug::try_from_attributes(attrs)? {
                if let Some(ty) = debug_attribute.ty {
                    struct_generics.insert(
                        ident.clone(),
                        (format_ident!("T_{}", ident), quote! { #ty }),
                    );
                    custom_type = true;
                }

                let mut field_ts = quote! { self.#ident };

                if let Some(parse_expr) = debug_attribute.parse {
                    field_ts = quote! { #parse_expr(&#field_ts).await };
                }

                if debug_attribute.copy.is_some() && debug_attribute.clone.is_some() {
                    panic!("copy and clone are exclusive");
                }

                if debug_attribute.copy.is_some() {
                    field_ts = quote! { *#field_ts };
                } else if debug_attribute.clone.is_some() {
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

        Ok((struct_generics, fields_ts))
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
            .keys()
            .map(|ident| {
                let ident_ty = format_ident!("T_{}", ident);

                quote! { #ident: #ident_ty, }
            })
            .collect()
    }

    fn get_assign_fields(&self, fields_ts: FieldsTs) -> Vec<TokenStream> {
        self.fields
            .keys()
            .map(|ident| {
                if let Some(ts) = fields_ts.get(ident) {
                    quote! { #ident: #ts, }
                } else {
                    quote! { #ident: &self.#ident, }
                }
            })
            .collect()
    }

    fn to_token_stream(&self) -> Result<TokenStream, Error> {
        let (struct_generics, fields_ts) = self.get_fields()?;
        let (struct_generic_names, struct_generic_types) = self.get_generics(struct_generics);
        let struct_async_fields = self.get_async_fields();
        let assign_fields = self.get_assign_fields(fields_ts);

        let vis = &self.vis;
        let ident = &self.ident;
        let debug_struct_ident = format_ident!("{}Debug", self.ident);

        Ok(quote! {
            impl AsyncDebug for #ident {}

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
        })
    }
}

#[derive(FromAttributes)]
struct AsyncDebug {
    parse: Option<Expr>,
    clone: Option<()>,
    copy: Option<()>,
    ty: Option<Type>,
}
