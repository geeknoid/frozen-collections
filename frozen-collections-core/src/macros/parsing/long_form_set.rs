use crate::macros::parsing::payload::{Payload, parse_set_payload};
use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream};
use syn::{Expr, Token, Type, Visibility};

pub struct LongFormSet {
    pub var_name: Ident,
    pub type_name: Ident,
    pub value_type: Type,
    pub payload: Payload,

    pub visibility: Visibility,
    pub is_static: bool,
    pub is_mutable: bool,
}

impl Parse for LongFormSet {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // var_name: type_name
        let var_name = input.parse::<Ident>()?;
        input.parse::<Token![:]>()?;
        let type_name = input.parse::<Ident>()?;

        // <value_type>
        input.parse::<Token![<]>()?;
        let value_type = input.parse()?;
        input.parse::<Token![>]>()?;
        input.parse::<Token![,]>()?;

        Ok(Self {
            var_name,
            type_name,
            value_type,
            payload: parse_set_payload(input)?,

            // these will be overridden by the caller
            visibility: Visibility::Inherited,
            is_static: false,
            is_mutable: false,
        })
    }
}

pub struct SetEntry {
    pub value: Expr,
}

impl Parse for SetEntry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            value: input.parse::<Expr>()?,
        })
    }
}
