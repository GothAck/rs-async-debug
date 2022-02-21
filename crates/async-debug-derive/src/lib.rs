mod zip_result;

extern crate proc_macro;

use bae::FromAttributes;
use indexmap::IndexMap;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse2, Data, DataStruct, DeriveInput, Error, Expr, Field, Fields, FieldsNamed,
    GenericArgument, ImplGenerics, Type, TypeGenerics, Visibility, WhereClause,
};

use self::zip_result::ZipResult;

type Result<T> = std::result::Result<T, Error>;

#[proc_macro_derive(AsyncDebug, attributes(async_debug))]
pub fn async_debug(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match async_debug_impl(input.into()) {
        Ok(output) => output.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn async_debug_impl(input: TokenStream) -> Result<TokenStream> {
    let input: DeriveInput = parse2(input)?;

    match &input.data {
        Data::Struct(DataStruct { fields, .. }) => match fields {
            Fields::Named(FieldsNamed { named: fields, .. }) => {
                let fields = fields.iter().cloned().collect();

                AsyncDebugStructNamed::new(&input, fields)?.to_token_stream()
            }
            Fields::Unit => Err(Error::new(
                Span::call_site(),
                "unit structs are not supported",
            )),
            Fields::Unnamed(..) => Err(Error::new(
                Span::call_site(),
                "unnamed field structs are not supported",
            )),
        },
        Data::Enum(..) => Err(Error::new(Span::call_site(), "enums are not supported")),
        Data::Union(..) => Err(Error::new(Span::call_site(), "unions are not supported")),
    }
}

struct AsyncDebugStructNamed<'a> {
    vis: Visibility,
    ident: Ident,
    generics_impl: ImplGenerics<'a>,
    generics_ty: TypeGenerics<'a>,
    where_clause: Option<&'a WhereClause>,
    fields: IndexMap<Ident, AsyncDebugField>,
}

impl<'a> AsyncDebugStructNamed<'a> {
    pub fn new(input: &'a DeriveInput, fields: Vec<Field>) -> Result<Self> {
        let fields = fields
            .iter()
            .map(|field| {
                let ident = field
                    .ident
                    .clone()
                    .ok_or_else(|| Error::new(Span::call_site(), "Missing field ident"))?;

                Ok((ident, AsyncDebugField::new(field.clone())?))
            })
            .collect::<Result<IndexMap<_, _>>>()?;

        let (generics_impl, generics_ty, where_clause) = input.generics.split_for_impl();

        Ok(Self {
            vis: input.vis.clone(),
            ident: input.ident.clone(),
            generics_impl,
            generics_ty,
            where_clause,
            fields,
        })
    }

    fn get_generics(&self) -> Result<(Vec<GenericArgument>, Vec<GenericArgument>)> {
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
            .map(|field| field.to_token_stream())
            .collect()
    }

    fn to_token_stream(&self) -> Result<TokenStream> {
        let (struct_new_generics_names, struct_new_generics) = self.get_generics()?;
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
                #vis async fn async_debug (&self) -> #debug_struct_ident <#(#struct_new_generics),*>
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
            #vis struct #debug_struct_ident <#(#struct_new_generics_names),*>
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

#[derive(FromAttributes)]
struct AsyncDebug {
    parse: Option<Expr>,
    clone: Option<()>,
    copy: Option<()>,
    ty: Option<Type>,
}

struct AsyncDebugField {
    ident: Ident,
    field: Field,
    async_debug: Option<AsyncDebug>,
}

impl AsyncDebugField {
    fn new(field: Field) -> Result<Self> {
        let ident = field
            .ident
            .clone()
            .ok_or_else(|| Error::new(Span::call_site(), "Missing ident"))?;
        let async_debug = AsyncDebug::try_from_attributes(&field.attrs)?;

        Ok(Self {
            ident,
            field,
            async_debug,
        })
    }

    fn ty(&self) -> Result<Type> {
        if let Some(async_debug) = &self.async_debug {
            if let Some(ty) = &async_debug.ty {
                return Ok(ty.clone());
            }
        }
        let ty = &self.field.ty;
        parse2(quote! { &#ty })
    }

    fn custom_type(&self) -> bool {
        if let Some(async_debug) = &self.async_debug {
            return async_debug.ty.is_some();
        }
        false
    }

    fn generic_argument_ident(&self) -> Ident {
        format_ident!("T_AsyncDebug_{}", self.ident)
    }

    fn generic_argument(&self) -> Result<GenericArgument> {
        parse2(self.generic_argument_ident().to_token_stream())
    }

    fn to_token_stream(&self) -> Result<TokenStream> {
        let ident = &self.ident;

        let mut ts = quote! { self.#ident };

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
