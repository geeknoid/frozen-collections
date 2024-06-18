use syn::parse::{Parse, ParseStream};
use syn::{Expr, Token};

use crate::macros::parsing::set_entry::SetEntry;

pub struct Set {
    pub values: Vec<SetEntry>,
}

impl Parse for Set {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut values = vec![];

        while !input.is_empty() {
            let value = input.parse::<Expr>()?;

            values.push(SetEntry { value });

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(Self { values })
    }
}
