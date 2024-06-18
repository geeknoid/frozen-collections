use crate::macros::parsing::map_entry::MapEntry;
use crate::macros::parsing::set_entry::SetEntry;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Expr;

#[derive(Clone)]
pub struct CommonEntry {
    pub key: Expr,
    pub value: Option<Expr>,
}

impl From<MapEntry> for CommonEntry {
    fn from(entry: MapEntry) -> Self {
        Self {
            key: entry.key,
            value: Some(entry.value),
        }
    }
}

impl From<SetEntry> for CommonEntry {
    fn from(entry: SetEntry) -> Self {
        Self {
            key: entry.value,
            value: None,
        }
    }
}

impl ToTokens for CommonEntry {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let key = &self.key;
        if let Some(value) = &self.value {
            tokens.extend(quote!((#key, #value)));
        } else {
            tokens.extend(quote!((#key, ())));
        }
    }
}
