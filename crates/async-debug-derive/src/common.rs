use bae::FromAttributes;
use convert_case::{Case, Casing};
use proc_macro2::{Ident, Span};
use syn::{Error, Expr, Type};

pub trait ErrorCallSite {
    fn new_call_site<T>(message: T) -> Error
    where
        T: std::fmt::Display;
}

impl ErrorCallSite for Error {
    fn new_call_site<T>(message: T) -> Error
    where
        T: std::fmt::Display,
    {
        Error::new(Span::call_site(), message)
    }
}

pub trait AsyncDebugCommon {
    fn get_async_debug_mod_ident(ident: &Ident) -> Ident {
        Ident::new(
            &format!("async_debug_{}", ident.to_string().to_case(Case::Snake)),
            ident.span(),
        )
    }
}

#[derive(FromAttributes)]
pub struct AsyncDebug {
    pub async_call: Option<Expr>,
    pub clone: Option<()>,
    pub copy: Option<()>,
    pub ty: Option<Type>,
}
