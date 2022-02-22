use bae::FromAttributes;
use proc_macro2::{Ident, Span};
use quote::format_ident;
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
    fn get_async_debug_ident(ident: &Ident) -> Ident {
        format_ident!("{}AsyncDebug", ident)
    }
}

#[derive(FromAttributes)]
pub struct AsyncDebug {
    pub async_call: Option<Expr>,
    pub clone: Option<()>,
    pub copy: Option<()>,
    pub ty: Option<Type>,
}
