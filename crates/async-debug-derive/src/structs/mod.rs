mod named;

use proc_macro2::TokenStream;
use syn::{DeriveInput, Error, Fields, FieldsNamed};

use self::named::AsyncDebugStructNamed;
use super::{common::*, Result};

pub enum AsyncDebugStruct<'a> {
    Named(AsyncDebugStructNamed<'a>),
    Unit,
    Unnamed,
}

impl<'a> AsyncDebugStruct<'a> {
    pub fn new(input: &'a DeriveInput, fields: &Fields) -> Result<Self> {
        Ok(match fields {
            Fields::Named(FieldsNamed { named: fields, .. }) => {
                Self::Named(AsyncDebugStructNamed::new(input, fields.iter().collect())?)
            }
            Fields::Unit => Self::Unit,
            Fields::Unnamed(..) => Self::Unnamed,
        })
    }

    pub fn to_token_stream(&self) -> Result<TokenStream> {
        match self {
            Self::Named(named) => named.to_token_stream(),
            Self::Unit => Err(Error::new_call_site("unit structs are not supported")),
            Self::Unnamed => Err(Error::new_call_site("unnamed structs are not supported")),
        }
    }
}
