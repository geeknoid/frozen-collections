use syn::parse::{Parse, ParseStream};
use syn::{Expr, Token};

use crate::macros::parsing::map_entry::MapEntry;

pub struct Map {
    pub entries: Vec<MapEntry>,
}

impl Parse for Map {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut entries = vec![];

        while !input.is_empty() {
            let key = input.parse::<Expr>()?;

            if input.parse::<Token![=>]>().is_err() {
                _ = input.parse::<Token![:]>()?;
            }

            let value = input.parse::<Expr>()?;
            entries.push(MapEntry { key, value });

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(Self { entries })
    }
}
