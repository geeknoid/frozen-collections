use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{Expr, Token};

#[derive(Clone)]
pub struct Entry {
    pub key: Expr,
    pub value: Option<Expr>,
}

impl Parse for Entry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key = input.parse::<Expr>()?;

        if input.parse::<Token![=>]>().is_err() {
            _ = input.parse::<Token![:]>()?;
        }

        let value = Some(input.parse::<Expr>()?);

        Ok(Self { key, value })
    }
}

impl ToTokens for Entry {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let key = &self.key;
        if let Some(value) = &self.value {
            tokens.extend(quote!((#key, #value)));
        } else {
            tokens.extend(quote!((#key, ())));
        }
    }
}
