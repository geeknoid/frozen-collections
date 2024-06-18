use crate::macros::parsing::payload::{parse_set_payload, Payload};
use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream};
use syn::{Expr, Path, Token, Visibility};

pub struct LongFormSet {
    pub var_name: Ident,
    pub type_name: Ident,
    pub value_type_amp: bool,
    pub value_type: Path,
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
        let value_type_amp = input.parse::<Token![&]>().ok();
        let value_type = input.parse::<Path>()?;
        input.parse::<Token![>]>()?;
        input.parse::<Token![,]>()?;

        Ok(Self {
            var_name,
            type_name,
            value_type_amp: value_type_amp.is_some(),
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
