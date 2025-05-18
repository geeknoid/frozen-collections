use crate::macros::parsing::payload::{Payload, parse_map_payload};
use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream};
use syn::{Token, Type, Visibility};

pub struct LongFormMap {
    pub var_name: Ident,
    pub type_name: Ident,

    pub key_type: Type,
    pub value_type: Type,
    pub payload: Payload,

    pub visibility: Visibility,
    pub is_static: bool,
    pub is_mutable: bool,
}

impl Parse for LongFormMap {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // var_name: type_name
        let var_name = input.parse::<Ident>()?;
        _ = input.parse::<Token![:]>()?;
        let type_name = input.parse::<Ident>()?;

        // <key_type, value_type>
        _ = input.parse::<Token![<]>()?;
        let key_type = input.parse()?;
        _ = input.parse::<Token![,]>()?;
        let value_type = input.parse()?;
        _ = input.parse::<Token![>]>()?;
        _ = input.parse::<Token![,]>()?;

        Ok(Self {
            var_name,
            type_name,
            key_type,
            value_type,
            payload: parse_map_payload(input)?,

            // these get overridden by the caller
            visibility: Visibility::Inherited,
            is_static: false,
            is_mutable: false,
        })
    }
}
