/// An individual slot in a hash table.
///
#[doc = include_str!("../doc_snippets/type_compat_warning.md")]
///
/// A slot contains the range of indices in the table's entry vector
/// that contain entries that hash to this slot.
#[derive(Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct HashTableSlot<CM> {
    pub(crate) min_index: CM,
    pub(crate) max_index: CM,
}

impl<CM> HashTableSlot<CM> {
    pub const fn new(min_index: CM, max_index: CM) -> Self {
        Self {
            min_index,
            max_index,
        }
    }
}
