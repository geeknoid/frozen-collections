#![allow(clippy::redundant_pub_crate)]

use core::fmt::{Debug, Formatter};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_quote, Expr};

#[cfg(feature = "macros")]
pub(crate) struct NonLiteralKey;

pub struct CollectionEntry<K> {
    pub(crate) key: K,
    key_expr: Expr,
    pub(crate) value_expr: Expr,
}

impl<K> CollectionEntry<K> {
    pub const fn map_entry(key: K, key_expr: Expr, value_expr: Expr) -> Self {
        Self {
            key,
            key_expr,
            value_expr,
        }
    }

    pub fn set_entry(value: K, value_expr: Expr) -> Self {
        Self {
            key: value,
            key_expr: value_expr,
            value_expr: parse_quote!(()),
        }
    }
}

impl<K> ToTokens for CollectionEntry<K> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let key = &self.key_expr;
        let value = &self.value_expr;
        tokens.extend(quote!((#key, #value)));
    }
}

impl<K> Clone for CollectionEntry<K>
where
    K: Clone,
{
    fn clone(&self) -> Self {
        Self {
            key: self.key.clone(),
            key_expr: self.key_expr.clone(),
            value_expr: self.value_expr.clone(),
        }
    }
}

impl<K> Debug for CollectionEntry<K>
where
    K: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "CollectionEntry {{ key: {:?}, key_expr '{}', value_expr: '{}'}}",
            self.key,
            self.key_expr.to_token_stream(),
            self.value_expr.to_token_stream()
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_map_entry() {
        let key = "key";
        let key_expr: Expr = parse_quote!(key);
        let value_expr: Expr = parse_quote!(value);
        let entry = CollectionEntry::map_entry(key, key_expr.clone(), value_expr.clone());

        assert_eq!(entry.key, key);
        assert_eq!(entry.key_expr, key_expr);
        assert_eq!(entry.value_expr, value_expr);
    }

    #[test]
    fn test_set_entry() {
        let value = "value";
        let value_expr: Expr = parse_quote!(value);
        let entry = CollectionEntry::set_entry(value, value_expr.clone());

        assert_eq!(entry.key, value);
        assert_eq!(entry.key_expr, value_expr);
        assert_eq!(entry.value_expr, parse_quote!(()));
    }

    #[test]
    fn test_to_tokens() {
        let key_expr: Expr = parse_quote!(key);
        let value_expr: Expr = parse_quote!(value);
        let entry = CollectionEntry::map_entry("key", key_expr, value_expr);
        let mut tokens = TokenStream::new();
        entry.to_tokens(&mut tokens);

        assert_eq!(tokens.to_string(), "(key , value)");
    }

    #[test]
    fn test_clone() {
        let key = "key";
        let key_expr: Expr = parse_quote!(key);
        let value_expr: Expr = parse_quote!(value);
        let entry = CollectionEntry::map_entry(key, key_expr.clone(), value_expr.clone());
        let cloned_entry = entry.clone();

        assert_eq!(cloned_entry.key, key);
        assert_eq!(cloned_entry.key_expr, key_expr);
        assert_eq!(cloned_entry.value_expr, value_expr);
    }

    #[test]
    fn test_debug() {
        let key = "key";
        let key_expr: Expr = parse_quote!(key);
        let value_expr: Expr = parse_quote!(value);
        let entry = CollectionEntry::map_entry(key, key_expr, value_expr);
        let debug_str = format!("{entry:?}");

        assert_eq!(
            debug_str,
            "CollectionEntry { key: \"key\", key_expr 'key', value_expr: 'value'}"
        );
    }
}
