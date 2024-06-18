use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{Expr, Token};

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct MapEntry {
    pub key: Expr,
    pub value: Expr,
}

impl Parse for MapEntry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key = input.parse::<Expr>()?;

        if input.parse::<Token![=>]>().is_err() {
            _ = input.parse::<Token![:]>()?;
        }

        let value = input.parse::<Expr>()?;

        Ok(Self { key, value })
    }
}

impl ToTokens for MapEntry {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let key = self.key.clone();
        let value = self.value.clone();

        tokens.extend(quote!(#key, #value));
    }
}
