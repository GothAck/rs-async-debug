use convert_case::{Case, Casing};
use proc_macro2::Ident;
use syn::Attribute;

use self::{attr_struct_enum::AsyncDebug as AsyncDebugStructEnum, prelude::*};

pub trait AsyncDebugCommon {
    fn get_async_debug_mod_ident(ident: &Ident) -> Ident {
        Ident::new(
            &format!("async_debug_{}", ident.to_string().to_case(Case::Snake)),
            ident.span(),
        )
    }

    fn get_attr_struct_enum(attrs: &[Attribute]) -> Result<AsyncDebugStructEnum> {
        Ok(AsyncDebugStructEnum::try_from_attributes(attrs)?.unwrap_or_default())
    }
}

pub mod attr_prop {
    use bae::FromAttributes;
    use syn::{spanned::Spanned, Expr, Type};

    use crate::common::prelude::*;

    #[derive(FromAttributes, Default)]
    pub struct AsyncDebug {
        pub async_call: Option<Expr>,
        pub clone: Option<()>,
        pub copy: Option<()>,
        pub ty: Option<Type>,

        pub skip: Option<()>,
    }

    impl AsyncDebug {
        pub fn validate(&self, spanned: &impl Spanned) -> Result<()> {
            if self.skip.is_some()
                && (self.async_call.is_some() || self.clone.is_some() || self.copy.is_some())
            {
                return Err(Error::new(spanned.span(), "skip can only be used alone"));
            }

            if self.clone.is_some() && self.copy.is_some() {
                return Err(Error::new(
                    spanned.span(),
                    "clone and copy are mutually exclusive",
                ));
            }

            Ok(())
        }
    }
}

pub mod attr_struct_enum {
    use bae::FromAttributes;

    #[derive(FromAttributes, Default)]
    pub struct AsyncDebug {
        pub disable_derive_debug: Option<()>,
    }
}

pub mod prelude {
    use proc_macro2::Span;
    pub use syn::Error;

    pub type Result<T> = std::result::Result<T, Error>;

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
}
