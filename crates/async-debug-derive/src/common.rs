use bae::FromAttributes;
use proc_macro2::Span;
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

#[derive(FromAttributes)]
pub struct AsyncDebug {
    pub parse: Option<Expr>,
    pub clone: Option<()>,
    pub copy: Option<()>,
    pub ty: Option<Type>,
}
