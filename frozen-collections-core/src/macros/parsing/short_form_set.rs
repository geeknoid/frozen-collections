use crate::macros::parsing::payload::{Payload, parse_set_payload};
use syn::parse::{Parse, ParseStream};

pub struct ShortFormSet {
    pub payload: Payload,
}

impl Parse for ShortFormSet {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            payload: parse_set_payload(input)?,
        })
    }
}
