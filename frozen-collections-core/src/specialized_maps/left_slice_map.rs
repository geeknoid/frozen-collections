use std::borrow::Borrow;
use std::fmt::{Debug, Formatter, Result};
use std::hash::RandomState;
use std::hash::{BuildHasher, Hash};
use std::intrinsics::transmute;
use std::mem::MaybeUninit;
use std::ops::Range;
use std::ops::{Index, IndexMut};

use num_traits::{PrimInt, Unsigned};

use crate::analyzers::{analyze_hash_codes, check_duplicate_keys};
use crate::specialized_maps::hash_table::HashTable;
use crate::specialized_maps::utils::any_duplicate_keys;
use crate::specialized_maps::{
    IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, Values, ValuesMut,
};
use crate::traits::Len;
use crate::traits::RangeHash;

/// A map that hashes left-aligned slices of its keys.
///
/// # Capacity Constraints
///
/// The `S` generic argument controls the maximum capacity
/// of the map. A `u8` will allow up to 255 entries, `u16`
/// will allow up to 65,535 entries, and `usize` will allow
/// up to `usize::MAX` entries.
///
/// # Important Note
///
/// This type is not intended to be used directly by
/// application code. Instead, applications are expected
/// to use the `FrozenMap` type or the `frozen_map!` macro.
#[derive(Clone)]
pub struct LeftSliceMap<K, V, S = u8, BH = RandomState> {
    pub(crate) table: HashTable<K, V, S>,
    bh: BH,
    range: Range<usize>,
}

impl<K, V, S> LeftSliceMap<K, V, S, RandomState>
where
    K: RangeHash + Len + Hash + Eq,
    S: PrimInt + Unsigned,
{
    #[allow(clippy::missing_errors_doc)]
    pub fn try_from(
        payload: Vec<(K, V)>,
        range: Range<usize>,
    ) -> std::result::Result<Self, &'static str> {
        Self::with_hasher(payload, range, RandomState::new())
    }
}

impl<K, V, S, BH> LeftSliceMap<K, V, S, BH>
where
    K: RangeHash + Len + Hash + Eq,
    S: PrimInt + Unsigned,
    BH: BuildHasher,
{
    #[allow(clippy::missing_errors_doc)]
    pub fn with_hasher(
        payload: Vec<(K, V)>,
        range: Range<usize>,
        bh: BH,
    ) -> std::result::Result<Self, &'static str> {
        check_duplicate_keys(payload.iter().map(|entry| &entry.0))?;

        let codes = payload.iter().map(|entry| {
            let key = &entry.0;
            if key.len() >= range.end {
                key.hash_range(&bh, range.clone())
            } else {
                0
            }
        });
        let code_analysis = analyze_hash_codes(codes);

        Ok(Self {
            table: HashTable::new(payload, code_analysis.num_hash_slots, |k| {
                k.hash_range(&bh, range.clone())
            })?,
            bh,
            range,
        })
    }
}

impl<K, V, S, BH> LeftSliceMap<K, V, S, BH>
where
    S: PrimInt + Unsigned,
    BH: BuildHasher,
{
    #[inline]
    #[must_use]
    fn get_hash_info<Q>(&self, key: &Q) -> Range<usize>
    where
        Q: ?Sized + RangeHash + Len,
    {
        let hash_code = if key.len() >= self.range.end {
            key.hash_range(&self.bh, self.range.clone())
        } else {
            0
        };

        self.table.get_hash_info(hash_code)
    }

    #[inline]
    #[must_use]
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: ?Sized + RangeHash + Len + Eq,
    {
        let range = self.get_hash_info(key);
        let entries = unsafe { self.table.entries.get_unchecked(range) };
        for entry in entries {
            if key.eq(entry.0.borrow()) {
                return Some(&entry.1);
            }
        }

        None
    }

    #[inline]
    #[must_use]
    pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q>,
        Q: ?Sized + RangeHash + Len + Eq,
    {
        let range = self.get_hash_info(key);
        let entries = unsafe { self.table.entries.get_unchecked(range) };
        for entry in entries {
            if key.eq(entry.0.borrow()) {
                return Some((&entry.0, &entry.1));
            }
        }

        None
    }

    #[inline]
    #[must_use]
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: ?Sized + RangeHash + Len + Eq,
    {
        let range = self.get_hash_info(key);
        let entries = unsafe { self.table.entries.get_unchecked_mut(range) };
        for entry in entries {
            if key.eq(entry.0.borrow()) {
                return Some(&mut entry.1);
            }
        }

        None
    }

    #[allow(mutable_transmutes)]
    pub fn get_many_mut<Q, const N: usize>(&mut self, keys: [&Q; N]) -> Option<[&mut V; N]>
    where
        K: Borrow<Q>,
        Q: ?Sized + RangeHash + Len + Eq,
    {
        if any_duplicate_keys(keys) {
            return None;
        }

        unsafe {
            let mut result: MaybeUninit<[&mut V; N]> = MaybeUninit::uninit();
            let p = result.as_mut_ptr();

            for (i, key) in keys.iter().enumerate() {
                *(*p).get_unchecked_mut(i) = transmute(self.get(key)?);
            }

            Some(result.assume_init())
        }
    }

    #[inline]
    #[must_use]
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: ?Sized + RangeHash + Len + Eq,
    {
        self.get(key).is_some()
    }
}

impl<K, V, S, BH> LeftSliceMap<K, V, S, BH> {
    #[must_use]
    pub const fn iter(&self) -> Iter<K, V> {
        Iter::new(&self.table.entries)
    }

    #[must_use]
    pub const fn keys(&self) -> Keys<K, V> {
        Keys::new(&self.table.entries)
    }

    #[must_use]
    pub const fn values(&self) -> Values<K, V> {
        Values::new(&self.table.entries)
    }

    #[must_use]
    pub fn into_keys(self) -> IntoKeys<K, V> {
        IntoKeys::new(self.table.entries)
    }

    #[must_use]
    pub fn into_values(self) -> IntoValues<K, V> {
        IntoValues::new(self.table.entries)
    }

    #[must_use]
    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        IterMut::new(self.table.entries.as_mut())
    }

    #[must_use]
    pub fn values_mut(&mut self) -> ValuesMut<'_, K, V> {
        ValuesMut::new(self.table.entries.as_mut())
    }

    #[must_use]
    pub const fn hasher(&self) -> &BH {
        &self.bh
    }
}

impl<K, V, S, BH> Len for LeftSliceMap<K, V, S, BH> {
    fn len(&self) -> usize {
        self.table.len()
    }
}

impl<K, V, S, BH> Debug for LeftSliceMap<K, V, S, BH>
where
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.table.fmt(f)
    }
}

impl<K, V, S, BH> Default for LeftSliceMap<K, V, S, BH>
where
    S: PrimInt + Unsigned,
    BH: BuildHasher + Default,
{
    fn default() -> Self {
        Self {
            table: HashTable::default(),
            bh: BH::default(),
            range: Range::default(),
        }
    }
}

impl<Q, K, V, S, BH> Index<&Q> for LeftSliceMap<K, V, S, BH>
where
    K: Borrow<Q>,
    Q: ?Sized + RangeHash + Len + Eq,
    S: PrimInt + Unsigned,
    BH: BuildHasher,
{
    type Output = V;

    fn index(&self, index: &Q) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<Q, K, V, S, BH> IndexMut<&Q> for LeftSliceMap<K, V, S, BH>
where
    K: Borrow<Q>,
    Q: ?Sized + RangeHash + Len + Eq,
    S: PrimInt + Unsigned,
    BH: BuildHasher,
{
    fn index_mut(&mut self, index: &Q) -> &mut V {
        self.get_mut(index).unwrap()
    }
}

impl<'a, K, V, S, BH> IntoIterator for &'a LeftSliceMap<K, V, S, BH> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, K, V, S, BH> IntoIterator for &'a mut LeftSliceMap<K, V, S, BH> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<K, V, S, BH> PartialEq<Self> for LeftSliceMap<K, V, S, BH>
where
    K: RangeHash + Len + Eq,
    V: PartialEq,
    S: PrimInt + Unsigned,
    BH: BuildHasher,
{
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self.iter()
            .all(|(key, value)| other.get(key).map_or(false, |v| *value == *v))
    }
}

impl<K, V, S, BH> Eq for LeftSliceMap<K, V, S, BH>
where
    K: RangeHash + Len + Eq,
    V: Eq,
    S: PrimInt + Unsigned,
    BH: BuildHasher,
{
}

impl<K, V, S, BH> IntoIterator for LeftSliceMap<K, V, S, BH> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self.table.entries)
    }
}
