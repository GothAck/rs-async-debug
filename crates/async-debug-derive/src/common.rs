use convert_case::{Case, Casing};
use proc_macro2::Ident;
use syn::Attribute;

use self::{attr_struct_enum::AsyncDebugAttrStructEnum, prelude::*};

pub trait AsyncDebugCommon {
    fn get_async_debug_mod_ident(ident: &Ident) -> Ident {
        Ident::new(
            &format!("async_debug_{}", ident.to_string().to_case(Case::Snake)),
            ident.span(),
        )
    }

    fn get_attr_struct_enum(attrs: &[Attribute]) -> Result<AsyncDebugAttrStructEnum> {
        Ok(AsyncDebugAttrStructEnum::try_from_attributes(attrs)?.unwrap_or_default())
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

            if let Some(async_call) = &self.async_call {
                if !matches!(async_call, Expr::Path(_)) {
                    return Err(Error::new(
                        spanned.span(),
                        "async_call must be a path to a function",
                    ));
                }
            }

            Ok(())
        }
    }

    pub use AsyncDebug as AsyncDebugAttrField;
}

pub mod attr_struct_enum {
    use bae::FromAttributes;

    #[derive(FromAttributes, Default)]
    pub struct AsyncDebug {
        pub disable_derive_debug: Option<()>,
    }

    pub use self::AsyncDebug as AsyncDebugAttrStructEnum;
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

    pub trait IterCombineSynErrors<T, I>
    where
        I: std::iter::Iterator<Item = Result<T>>,
    {
        fn collect_syn_error<B: FromIterator<T>>(self) -> Result<B>
        where
            B: Default;
    }

    impl<T, I> IterCombineSynErrors<T, I> for I
    where
        I: std::iter::Iterator<Item = Result<T>>,
    {
        fn collect_syn_error<B: FromIterator<T>>(self) -> Result<B>
        where
            B: Default,
        {
            let res_vec =
                self.fold::<Result<Vec<T>>, _>(Ok(Default::default()), |accum, res| {
                    match (accum, res) {
                        (Err(mut ea), Err(er)) => {
                            ea.combine(er);
                            Err(ea)
                        }
                        (Err(ea), Ok(_)) => Err(ea),
                        (Ok(_), Err(er)) => Err(er),
                        (Ok(mut va), Ok(vr)) => {
                            va.push(vr);
                            Ok(va)
                        }
                    }
                });

            res_vec.map(|vec| B::from_iter(vec))
        }
    }
}
