use crate::maps::decl_macros::{
    contains_key_fn, debug_fn, get_many_mut_body, get_many_mut_fn, index_fn,
    into_iter_fn_for_slice, into_iter_mut_ref_fn, into_iter_ref_fn, map_boilerplate_for_slice,
    map_iterator_boilerplate_for_slice, partial_eq_fn, sparse_scalar_lookup_core,
};
use crate::maps::{IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, Values, ValuesMut};
use crate::traits::{CollectionMagnitude, Len, Map, MapIterator, Scalar, SmallCollection};
use crate::utils::dedup_by_keep_last;
use alloc::vec::Vec;
use core::borrow::Borrow;
use core::fmt::{Debug, Formatter, Result};
use core::ops::Index;

/// A map whose keys are a sparse range of integers.
///
#[doc = include_str!("../doc_snippets/type_compat_warning.md")]
#[doc = include_str!("../doc_snippets/about.md")]
///
/// # Type Parameters
///
/// - `K`: The key type.
/// - `V`: The value type.
/// - `CM`: The magnitude of the map, one of [`SmallCollection`](crate::traits::SmallCollection), [`MediumCollection`](crate::traits::MediumCollection), or [`LargeCollection`](crate::traits::LargeCollection).
/// - `SZ`: The number of entries in the map.
/// - `LTSZ`: The number of entries in the lookup table.
#[derive(Clone)]
pub struct InlineSparseScalarLookupMap<
    K,
    V,
    const SZ: usize,
    const LTSZ: usize,
    CM = SmallCollection,
> {
    min: usize,
    max: usize,
    lookup: [CM; LTSZ],
    entries: [(K, V); SZ],
}

impl<K, V, const SZ: usize, const LTSZ: usize, CM> InlineSparseScalarLookupMap<K, V, SZ, LTSZ, CM>
where
    K: Scalar,
    CM: CollectionMagnitude,
{
    /// Creates a new `IntegerSparseLookupMap` from a list of entries.
    ///
    /// Note that this supports 1 less entry relative to the maximum capacity of the collection scale
    /// since 0 is used as a sentinel value within the lookup table.
    ///
    /// # Errors
    ///
    /// Fails if the length of the vector, after removing duplicates, isn't equal to the generic parameter `SZ`,
    /// or if there are too many entries for the specified collection magnitude as indicated by the generic
    /// parameter `CM`.
    pub fn new(mut entries: Vec<(K, V)>) -> core::result::Result<Self, String> {
        entries.sort_by_key(|x| x.0);
        dedup_by_keep_last(&mut entries, |x, y| x.0.eq(&y.0));

        if SZ != entries.len() || SZ == 0 {
            let len = entries.len();
            return Ok(Self::new_raw(
                entries
                    .try_into()
                    .map_err(|_| format!("incorrect # of entries: got {len} but SZ={SZ}"))?,
                [CM::ZERO; LTSZ],
                1,
                0,
            ));
        }

        let min = entries[0].0.index();
        let max = entries[entries.len() - 1].0.index();

        let count = max - min + 1;
        if count >= CM::MAX_CAPACITY {
            return Err(
                "the range of keys is too large for the selected collection magnitude".to_string(),
            );
        }

        let mut lookup = Vec::<CM>::with_capacity(count);
        lookup.resize(lookup.capacity(), CM::ZERO);

        for (i, entry) in entries.iter().enumerate() {
            let index_in_lookup = entry.0.index() - min;
            let index_in_entries = CM::try_from(i + 1).map_err(|_| "Unreachable")?;
            lookup[index_in_lookup] = index_in_entries;
        }

        let len = entries.len();
        Ok(Self::new_raw(
            entries
                .try_into()
                .map_err(|_| format!("incorrect # of entries: got {len} but SZ={SZ}"))?,
            lookup.try_into().map_err(|_| {
                format!("incorrect # of lookup table slots: needs {count} but LTSZ={LTSZ}")
            })?,
            min,
            max,
        ))
    }

    /// Creates a frozen map.
    #[must_use]
    pub const fn new_raw(
        processed_entries: [(K, V); SZ],
        lookup: [CM; LTSZ],
        min: usize,
        max: usize,
    ) -> Self {
        Self {
            min,
            max,
            lookup,
            entries: processed_entries,
        }
    }
}

impl<K, V, const SZ: usize, const LTSZ: usize, CM> InlineSparseScalarLookupMap<K, V, SZ, LTSZ, CM>
where
    CM: CollectionMagnitude,
{
    sparse_scalar_lookup_core!();
}

impl<K, V, const SZ: usize, const LTSZ: usize, CM> Len
    for InlineSparseScalarLookupMap<K, V, SZ, LTSZ, CM>
{
    fn len(&self) -> usize {
        SZ
    }
}

impl<K, V, const SZ: usize, const LTSZ: usize, CM> Debug
    for InlineSparseScalarLookupMap<K, V, SZ, LTSZ, CM>
where
    K: Debug,
    V: Debug,
{
    debug_fn!();
}

impl<Q, K, V, const SZ: usize, const LTSZ: usize, CM> Index<&Q>
    for InlineSparseScalarLookupMap<K, V, SZ, LTSZ, CM>
where
    K: Borrow<Q>,
    Q: Scalar,
    CM: CollectionMagnitude,
{
    index_fn!();
}

impl<K, V, const SZ: usize, const LTSZ: usize, CM> IntoIterator
    for InlineSparseScalarLookupMap<K, V, SZ, LTSZ, CM>
{
    into_iter_fn_for_slice!(entries);
}

impl<'a, K, V, const SZ: usize, const LTSZ: usize, CM> IntoIterator
    for &'a InlineSparseScalarLookupMap<K, V, SZ, LTSZ, CM>
where
    CM: CollectionMagnitude,
{
    into_iter_ref_fn!();
}

impl<'a, K, V, const SZ: usize, const LTSZ: usize, CM> IntoIterator
    for &'a mut InlineSparseScalarLookupMap<K, V, SZ, LTSZ, CM>
where
    CM: CollectionMagnitude,
{
    into_iter_mut_ref_fn!();
}

impl<K, V, MT, const SZ: usize, const LTSZ: usize, CM> PartialEq<MT>
    for InlineSparseScalarLookupMap<K, V, SZ, LTSZ, CM>
where
    K: Scalar,
    V: PartialEq,
    MT: Map<K, V>,
    CM: CollectionMagnitude,
{
    partial_eq_fn!();
}

impl<K, V, const SZ: usize, const LTSZ: usize, CM> Eq
    for InlineSparseScalarLookupMap<K, V, SZ, LTSZ, CM>
where
    K: Scalar,
    V: Eq,
    CM: CollectionMagnitude,
{
}

impl<K, V, const SZ: usize, const LTSZ: usize, CM> MapIterator<K, V>
    for InlineSparseScalarLookupMap<K, V, SZ, LTSZ, CM>
{
    type Iterator<'a>
        = Iter<'a, K, V>
    where
        K: 'a,
        V: 'a,
        CM: 'a;

    type KeyIterator<'a>
        = Keys<'a, K, V>
    where
        K: 'a,
        V: 'a,
        CM: 'a;

    type ValueIterator<'a>
        = Values<'a, K, V>
    where
        K: 'a,
        V: 'a,
        CM: 'a;

    type MutIterator<'a>
        = IterMut<'a, K, V>
    where
        K: 'a,
        V: 'a,
        CM: 'a;

    type ValueMutIterator<'a>
        = ValuesMut<'a, K, V>
    where
        K: 'a,
        V: 'a,
        CM: 'a;

    map_iterator_boilerplate_for_slice!(entries);
}

impl<K, V, const SZ: usize, const LTSZ: usize, CM> Map<K, V>
    for InlineSparseScalarLookupMap<K, V, SZ, LTSZ, CM>
where
    K: Scalar,
    CM: CollectionMagnitude,
{
    map_boilerplate_for_slice!(entries);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_fails_when_too_many_entries() {
        let mut entries = Vec::new();
        for i in 0..3 {
            entries.push((i, 42));
        }

        let map = InlineSparseScalarLookupMap::<_, _, 2, 3>::new(entries);
        assert_eq!(
            map,
            Err("incorrect # of entries: got 3 but SZ=2".to_string())
        );
    }

    #[test]
    fn new_fails_when_lookup_too_small() {
        let mut entries = Vec::new();
        for i in 0..3 {
            entries.push((i, 42));
        }

        let map = InlineSparseScalarLookupMap::<_, _, 3, 2>::new(entries);
        assert_eq!(
            map,
            Err("incorrect # of lookup table slots: needs 3 but LTSZ=2".to_string())
        );
    }
}
