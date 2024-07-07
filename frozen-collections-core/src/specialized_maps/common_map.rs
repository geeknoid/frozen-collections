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

/// A general purpose map.
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
pub struct CommonMap<K, V, S = u8, BH = RandomState> {
    pub(crate) table: HashTable<K, V, S>,
    bh: BH,
}

impl<K, V, S, BH> CommonMap<K, V, S, BH>
where
    K: Hash + Eq,
    S: PrimInt + Unsigned,
    BH: BuildHasher,
{
    #[allow(clippy::missing_errors_doc)]
    pub fn with_hasher(payload: Vec<(K, V)>, bh: BH) -> std::result::Result<Self, &'static str> {
        check_duplicate_keys(payload.iter().map(|entry| &entry.0))?;

        let code_analysis = analyze_hash_codes(payload.iter().map(|entry| bh.hash_one(&entry.0)));

        Ok(Self {
            table: HashTable::new(payload, code_analysis.num_hash_slots, |k| bh.hash_one(k))?,
            bh,
        })
    }
}

impl<K, V, S, BH> CommonMap<K, V, S, BH>
where
    S: PrimInt + Unsigned,
    BH: BuildHasher,
{
    #[inline]
    #[must_use]
    fn get_hash_info<Q>(&self, key: &Q) -> Range<usize>
    where
        Q: ?Sized + Hash + Eq,
    {
        let hash_code = self.bh.hash_one(key.borrow());
        self.table.get_hash_info(hash_code)
    }

    #[inline]
    #[must_use]
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
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
        Q: ?Sized + Hash + Eq,
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
        Q: ?Sized + Hash + Eq,
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
        Q: ?Sized + Hash + Eq,
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
        Q: ?Sized + Hash + Eq,
    {
        self.get(key).is_some()
    }
}

impl<K, V, S, BH> CommonMap<K, V, S, BH> {
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

impl<K, V, S, BH> Len for CommonMap<K, V, S, BH> {
    fn len(&self) -> usize {
        self.table.len()
    }
}

impl<K, V, S, BH> Debug for CommonMap<K, V, S, BH>
where
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.table.fmt(f)
    }
}

impl<K, V, S, BH> Default for CommonMap<K, V, S, BH>
where
    S: PrimInt + Unsigned,
    BH: BuildHasher + Default,
{
    fn default() -> Self {
        Self {
            table: HashTable::default(),
            bh: BH::default(),
        }
    }
}

impl<Q, K, V, S, BH> Index<&Q> for CommonMap<K, V, S, BH>
where
    K: Borrow<Q>,
    Q: ?Sized + Hash + Eq,
    S: PrimInt + Unsigned,
    BH: BuildHasher,
{
    type Output = V;

    fn index(&self, index: &Q) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<Q, K, V, S, BH> IndexMut<&Q> for CommonMap<K, V, S, BH>
where
    K: Borrow<Q>,
    Q: ?Sized + Hash + Eq,
    S: PrimInt + Unsigned,
    BH: BuildHasher,
{
    fn index_mut(&mut self, index: &Q) -> &mut V {
        self.get_mut(index).unwrap()
    }
}

impl<'a, K, V, S, BH> IntoIterator for &'a CommonMap<K, V, S, BH> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, K, V, S, BH> IntoIterator for &'a mut CommonMap<K, V, S, BH> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<K, V, S, BH> PartialEq<Self> for CommonMap<K, V, S, BH>
where
    K: Hash + Eq,
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

impl<K, V, S, BH> Eq for CommonMap<K, V, S, BH>
where
    K: Hash + Eq,
    V: Eq,
    S: PrimInt + Unsigned,
    BH: BuildHasher,
{
}

impl<K, V, S> TryFrom<Vec<(K, V)>> for CommonMap<K, V, S, RandomState>
where
    K: Hash + Eq,
    S: PrimInt + Unsigned,
{
    type Error = &'static str;

    fn try_from(payload: Vec<(K, V)>) -> std::result::Result<Self, Self::Error> {
        Self::with_hasher(payload, RandomState::new())
    }
}

impl<K, V, S, const N: usize> TryFrom<[(K, V); N]> for CommonMap<K, V, S, RandomState>
where
    K: Hash + Eq,
    S: PrimInt + Unsigned,
{
    type Error = &'static str;

    fn try_from(payload: [(K, V); N]) -> std::result::Result<Self, Self::Error> {
        Self::with_hasher(Vec::from_iter(payload), RandomState::new())
    }
}

impl<K, V, S> FromIterator<(K, V)> for CommonMap<K, V, S, RandomState>
where
    K: Hash + Eq,
    S: PrimInt + Unsigned,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        Self::with_hasher(Vec::from_iter(iter), RandomState::new()).unwrap()
    }
}

impl<K, V, S, BH> IntoIterator for CommonMap<K, V, S, BH> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self.table.entries)
    }
}

#[cfg(test)]
mod tests {
    use std::hash::RandomState;

    use super::*;

    #[test]
    fn test_from_iter_empty() {
        let pairs: Vec<(i32, i32)> = vec![];
        let map: CommonMap<i32, i32, u32, RandomState> = pairs.into_iter().collect();
        assert!(map.is_empty());
    }

    #[test]
    fn test_from_iter_single() {
        let pairs = vec![(1, 2)];
        let map: CommonMap<i32, i32, u32, RandomState> = pairs.into_iter().collect();
        assert_eq!(map.get(&1), Some(&2));
    }

    #[test]
    fn test_from_iter_multiple() {
        let pairs = vec![(1, 2), (3, 4), (5, 6)];
        let map: CommonMap<i32, i32, u32, RandomState> = pairs.into_iter().collect();
        assert_eq!(map.get(&1), Some(&2));
        assert_eq!(map.get(&3), Some(&4));
        assert_eq!(map.get(&5), Some(&6));
    }
}
