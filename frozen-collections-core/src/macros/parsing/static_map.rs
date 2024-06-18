use crate::macros::parsing::map_entry::MapEntry;
use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream};
use syn::token::Eq;
use syn::{braced, Path, Token, Visibility};

pub struct StaticMap {
    pub visibility: Visibility,
    pub var_name: Ident,
    pub type_name: Ident,
    pub key_type_amp: bool,
    pub key_type: Path,
    pub value_type: Path,
    pub entries: Vec<MapEntry>,
}

impl Parse for StaticMap {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![static]>()?;

        // var_name: type_name
        let var_name = input.parse::<Ident>()?;
        input.parse::<Token![:]>()?;
        let type_name = input.parse::<Ident>()?;

        // <key_type, value_type>
        input.parse::<Token![<]>()?;
        let key_type_amp = input.parse::<Token![&]>().ok();
        let key_type = input.parse::<Path>()?;
        input.parse::<Token![,]>()?;
        let value_type = input.parse::<Path>()?;
        input.parse::<Token![>]>()?;

        input.parse::<Eq>()?;

        // { key: value, key: value, ... };
        let content;
        _ = braced!(content in input);
        let entries = content.parse_terminated(MapEntry::parse, Token![,])?;

        Ok(Self {
            visibility: Visibility::Inherited, // will be overriden by the caller
            var_name,
            type_name,
            key_type_amp: key_type_amp.is_some(),
            key_type,
            value_type,
            entries: entries.into_iter().collect(),
        })
    }
}
