use crate::macros::parsing::payload::{parse_map_payload, Payload};
use syn::parse::{Parse, ParseStream};

pub struct ShortFormMap {
    pub payload: Payload,
}

impl Parse for ShortFormMap {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            payload: parse_map_payload(input)?,
        })
    }
}
