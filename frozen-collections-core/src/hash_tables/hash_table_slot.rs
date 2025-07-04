use crate::traits::CollectionMagnitude;

/// An individual slot in a hash table.
///
#[doc = include_str!("../doc_snippets/private_api_warning.md")]
///
/// A slot contains the range of indices in the table's entry vector
/// that contain entries that hash to this slot.
#[derive(Clone, Debug)]
pub struct HashTableSlot<CM> {
    pub(crate) min_index: CM,
    pub(crate) max_index: CM,
}

impl<CM> HashTableSlot<CM>
where
    CM: CollectionMagnitude,
{
    /// Creates a new hash table slot with the specified minimum and maximum indices.
    pub const fn new(min_index: CM, max_index: CM) -> Self {
        Self { min_index, max_index }
    }

    /// Returns whether the hash slot is completely empty.
    #[cfg(any(feature = "emit", feature = "macros"))]
    pub(crate) fn is_empty(&self) -> bool {
        self.max_index.into() == 0_usize
    }
}

#[cfg(any(feature = "macros", feature = "emit"))]
impl quote::ToTokens for HashTableSlot<usize> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let min_index = proc_macro2::Literal::usize_unsuffixed(self.min_index);
        let max_index = proc_macro2::Literal::usize_unsuffixed(self.max_index);

        tokens.extend(quote::quote!(::frozen_collections::hash_tables::HashTableSlot::new(#min_index, #max_index)));
    }
}
