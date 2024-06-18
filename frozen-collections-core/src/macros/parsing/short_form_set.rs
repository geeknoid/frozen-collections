use crate::macros::parsing::payload::{parse_set_payload, Payload};
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
