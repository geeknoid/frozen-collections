use crate::macros::parsing::entry::Entry;
use crate::macros::parsing::long_form_set::SetEntry;
use syn::parse::Parse;
use syn::{Token, braced};

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

/// Data associated with a frozen collection macro.
pub struct Payload {
    pub entries: Vec<Entry>,
}

pub fn parse_set_payload(input: syn::parse::ParseStream) -> syn::Result<Payload> {
    // { value, value, ... };
    let content;
    _ = braced!(content in input);

    Ok(Payload {
        entries: content
            .parse_terminated(SetEntry::parse, Token![,])?
            .into_iter()
            .map(|x| Entry { key: x.value, value: None })
            .collect(),
    })
}

pub fn parse_map_payload(input: syn::parse::ParseStream) -> syn::Result<Payload> {
    // { key: value, key: value, ... };
    let content;
    _ = braced!(content in input);

    Ok(Payload {
        entries: content.parse_terminated(Entry::parse, Token![,])?.into_iter().collect(),
    })
}
