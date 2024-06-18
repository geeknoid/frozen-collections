use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::Expr;

#[derive(Clone)]
pub struct SetEntry {
    pub value: Expr,
}

impl Parse for SetEntry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let value = input.parse::<Expr>()?;

        Ok(Self { value })
    }
}

impl ToTokens for SetEntry {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let value = self.value.clone();

        tokens.extend(quote!(#value));
    }
}
