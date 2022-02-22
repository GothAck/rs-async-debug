mod named;
mod unnamed;

use proc_macro2::TokenStream;
use syn::{DeriveInput, Error, Fields, FieldsNamed, FieldsUnnamed};

use self::{named::AsyncDebugStructNamed, unnamed::AsyncDebugStructUnnamed};
use super::{common::*, Result};

pub enum AsyncDebugStruct<'a> {
    Named(AsyncDebugStructNamed<'a>),
    Unit,
    Unnamed(AsyncDebugStructUnnamed<'a>),
}

impl<'a> AsyncDebugStruct<'a> {
    pub fn new(input: &'a DeriveInput, fields: &Fields) -> Result<Self> {
        Ok(match fields {
            Fields::Named(FieldsNamed { named: fields, .. }) => {
                Self::Named(AsyncDebugStructNamed::new(input, fields.iter().collect())?)
            }
            Fields::Unit => Self::Unit,
            Fields::Unnamed(FieldsUnnamed {
                unnamed: fields, ..
            }) => Self::Unnamed(AsyncDebugStructUnnamed::new(
                input,
                fields.iter().collect(),
            )?),
        })
    }

    pub fn to_token_stream(&self) -> Result<TokenStream> {
        match self {
            Self::Named(named) => named.to_token_stream(),
            Self::Unit => Err(Error::new_call_site("unit structs are not supported")),
            Self::Unnamed(unnamed) => unnamed.to_token_stream(),
        }
    }
}
