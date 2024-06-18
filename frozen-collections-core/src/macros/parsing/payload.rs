use crate::macros::parsing::entry::Entry;
use crate::macros::parsing::long_form_set::SetEntry;
use alloc::vec::Vec;
use syn::parse::Parse;
use syn::{braced, Expr, Token};

/// Data associated with a frozen collection macro.
pub enum Payload {
    /// Entries supplied inline with the macro
    InlineEntries(Vec<Entry>),

    /// An expression producing a vector of values to insert into the collection.
    Vector(Expr),
}

#[allow(clippy::module_name_repetitions)]
pub fn parse_set_payload(input: syn::parse::ParseStream) -> syn::Result<Payload> {
    let payload = if input.peek(::syn::token::Brace) {
        // { value, value, ... };
        let content;
        _ = braced!(content in input);
        Payload::InlineEntries(
            content
                .parse_terminated(SetEntry::parse, Token![,])?
                .into_iter()
                .map(|x| Entry {
                    key: x.value,
                    value: None,
                })
                .collect(),
        )
    } else {
        Payload::Vector(input.parse::<Expr>()?)
    };

    Ok(payload)
}

#[allow(clippy::module_name_repetitions)]
pub fn parse_map_payload(input: syn::parse::ParseStream) -> syn::Result<Payload> {
    let payload = if input.peek(::syn::token::Brace) {
        // { key: value, key: value, ... };
        let content;
        _ = braced!(content in input);
        Payload::InlineEntries(
            content
                .parse_terminated(Entry::parse, Token![,])?
                .into_iter()
                .collect(),
        )
    } else {
        Payload::Vector(input.parse::<Expr>()?)
    };

    Ok(payload)
}
