use crate::macros::parsing::set_entry::SetEntry;
use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream};
use syn::token::Eq;
use syn::{braced, Path, Token, Visibility};

pub struct StaticSet {
    pub visibility: Visibility,
    pub var_name: Ident,
    pub type_name: Ident,
    pub value_type_amp: bool,
    pub value_type: Path,
    pub entries: Vec<SetEntry>,
}

impl Parse for StaticSet {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![static]>()?;

        // var_name: type_name
        let var_name = input.parse::<Ident>()?;
        input.parse::<Token![:]>()?;
        let type_name = input.parse::<Ident>()?;

        // <value_type>
        input.parse::<Token![<]>()?;
        let value_type_amp = input.parse::<Token![&]>().ok();
        let value_type = input.parse::<Path>()?;
        input.parse::<Token![>]>()?;

        input.parse::<Eq>()?;

        // { value, value, ... };
        let content;
        _ = braced!(content in input);
        let entries = content.parse_terminated(SetEntry::parse, Token![,])?;

        Ok(Self {
            visibility: Visibility::Inherited, // will be overriden by the caller
            var_name,
            type_name,
            value_type_amp: value_type_amp.is_some(),
            value_type,
            entries: entries.into_iter().collect(),
        })
    }
}
