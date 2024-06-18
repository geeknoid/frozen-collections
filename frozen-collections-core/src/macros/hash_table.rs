use crate::maps::HashTableSlot;
use proc_macro2::Literal;
use quote::{quote, ToTokens};

impl ToTokens for HashTableSlot<usize> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let min_index = Literal::usize_unsuffixed(self.min_index);
        let max_index = Literal::usize_unsuffixed(self.max_index);

        tokens.extend(
            quote!(::frozen_collections::inline_maps::HashTableSlot::new(#min_index, #max_index)),
        );
    }
}
