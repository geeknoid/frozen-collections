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
        _ = input.parse::<Token![:]>()?;
        let value = Some(input.parse::<Expr>()?);

        Ok(Self { key, value })
    }
}
