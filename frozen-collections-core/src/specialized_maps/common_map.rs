use std::borrow::Borrow;
use std::fmt::{Debug, Formatter, Result};
use std::hash::{BuildHasher, Hash};
use std::ops::Range;
use std::ops::{Index, IndexMut};

use ahash::RandomState;
use num_traits::{PrimInt, Unsigned};

use crate::analyzers::{analyze_hash_codes, check_duplicate_keys, HashCodeAnalysisResult};
use crate::specialized_maps::hash_table::HashTable;
use crate::specialized_maps::utils::{
    any_duplicate_keys, get, get_key_value, get_key_value_no_collisions, get_many_mut, get_mut,
    get_mut_no_collisions, get_no_collisions, partial_eq,
};
use crate::specialized_maps::{
    IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, Values, ValuesMut,
};
use crate::traits::{Len, Map};

macro_rules! common_map {
    ($map_name:ident, $no_collisions:literal, $test_mod:ident) => {
        /// A general purpose map.
        ///
        /// # Capacity Constraints
        ///
        /// The `S` generic argument controls the maximum capacity
        /// of the map. A `u8` will allow up to 255 entries, `u16`
        /// will allow up to 65,535 entries, and `usize` will allow
        /// up to [`usize::MAX`] entries.
        ///
        /// # Important Note
        ///
        /// This type is not intended to be used directly by
        /// application code. Instead, applications are expected
        /// to use the `FrozenMap` type or the `frozen_map!` macro.
        #[derive(Clone)]
        pub struct $map_name<K, V, S = u8, BH = RandomState> {
            pub(crate) table: HashTable<K, V, S>,
            bh: BH,
        }

        impl<K, V, S> $map_name<K, V, S, RandomState>
        where
            K: Hash + Eq,
            S: PrimInt + Unsigned,
        {
            #[allow(clippy::missing_errors_doc)]
            pub fn new(payload: Vec<(K, V)>) -> std::result::Result<Self, &'static str> {
                Self::with_hasher(payload, RandomState::new())
            }
        }

        impl<K, V, S, BH> $map_name<K, V, S, BH>
        where
            K: Hash + Eq,
            S: PrimInt + Unsigned,
            BH: BuildHasher,
        {
            #[allow(clippy::missing_errors_doc)]
            pub fn with_hasher(
                payload: Vec<(K, V)>,
                bh: BH,
            ) -> std::result::Result<Self, &'static str> {
                check_duplicate_keys(payload.iter().map(|entry| &entry.0))?;

                let code_analysis =
                    analyze_hash_codes(payload.iter().map(|entry| bh.hash_one(&entry.0)));

                Self::with_hasher_and_analysis(payload, bh, code_analysis)
            }

            /// PRIVATE: used by macros, subject to change
            #[allow(clippy::missing_errors_doc)]
            #[doc(hidden)]
            pub fn with_hasher_and_analysis(
                payload: Vec<(K, V)>,
                bh: BH,
                code_analysis: HashCodeAnalysisResult,
            ) -> std::result::Result<Self, &'static str> {
                Ok(Self {
                    table: HashTable::new(payload, code_analysis.num_hash_slots, |k| {
                        bh.hash_one(k)
                    })?,
                    bh,
                })
            }
        }

        impl<K, V, S, BH> $map_name<K, V, S, BH>
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
            #[allow(clippy::redundant_else)]
            pub fn get<Q>(&self, key: &Q) -> Option<&V>
            where
                K: Borrow<Q>,
                Q: ?Sized + Hash + Eq,
            {
                if $no_collisions {
                    get_no_collisions!(self, key);
                } else {
                    get!(self, key);
                }
            }

            #[inline]
            #[must_use]
            #[allow(clippy::redundant_else)]
            pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
            where
                K: Borrow<Q>,
                Q: ?Sized + Hash + Eq,
            {
                if $no_collisions {
                    get_key_value_no_collisions!(self, key);
                } else {
                    get_key_value!(self, key);
                }
            }

            #[inline]
            #[must_use]
            #[allow(clippy::redundant_else)]
            pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
            where
                K: Borrow<Q>,
                Q: ?Sized + Hash + Eq,
            {
                if $no_collisions {
                    get_mut_no_collisions!(self, key);
                } else {
                    get_mut!(self, key);
                }
            }

            #[must_use]
            pub fn get_many_mut<Q, const N: usize>(&mut self, keys: [&Q; N]) -> Option<[&mut V; N]>
            where
                K: Borrow<Q>,
                Q: ?Sized + Hash + Eq,
            {
                get_many_mut!(self, keys);
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

        impl<K, V, S, BH> $map_name<K, V, S, BH> {
            #[must_use]
            pub const fn hasher(&self) -> &BH {
                &self.bh
            }
        }

        impl<K, V, S, BH> Len for $map_name<K, V, S, BH> {
            fn len(&self) -> usize {
                self.table.len()
            }
        }

        impl<K, V, S, BH> Debug for $map_name<K, V, S, BH>
        where
            K: Debug,
            V: Debug,
        {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                self.table.fmt(f)
            }
        }

        impl<K, V, S, BH> Default for $map_name<K, V, S, BH>
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

        impl<Q, K, V, S, BH> Index<&Q> for $map_name<K, V, S, BH>
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

        impl<Q, K, V, S, BH> IndexMut<&Q> for $map_name<K, V, S, BH>
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

        impl<K, V, S, BH> IntoIterator for $map_name<K, V, S, BH> {
            type Item = (K, V);
            type IntoIter = IntoIter<K, V>;

            fn into_iter(self) -> Self::IntoIter {
                IntoIter::new(self.table.entries)
            }
        }

        impl<'a, K, V, S, BH> IntoIterator for &'a $map_name<K, V, S, BH>
        where
            K: Hash + Eq,
            S: PrimInt + Unsigned,
            BH: BuildHasher,
        {
            type Item = (&'a K, &'a V);
            type IntoIter = Iter<'a, K, V>;

            fn into_iter(self) -> Self::IntoIter {
                self.iter()
            }
        }

        impl<'a, K, V, S, BH> IntoIterator for &'a mut $map_name<K, V, S, BH>
        where
            K: Hash + Eq,
            S: PrimInt + Unsigned,
            BH: BuildHasher,
        {
            type Item = (&'a K, &'a mut V);
            type IntoIter = IterMut<'a, K, V>;

            fn into_iter(self) -> Self::IntoIter {
                self.iter_mut()
            }
        }

        impl<K, V, S, MT, BH> PartialEq<MT> for $map_name<K, V, S, BH>
        where
            K: Hash + Eq,
            V: PartialEq,
            S: PrimInt + Unsigned,
            MT: Map<K, V>,
            BH: BuildHasher,
        {
            partial_eq!();
        }

        impl<K, V, S, BH> Eq for $map_name<K, V, S, BH>
        where
            K: Hash + Eq,
            V: Eq,
            S: PrimInt + Unsigned,
            BH: BuildHasher,
        {
        }

        impl<K, V, S, BH> TryFrom<Vec<(K, V)>> for $map_name<K, V, S, BH>
        where
            K: Hash + Eq,
            S: PrimInt + Unsigned,
            BH: BuildHasher + Default,
        {
            type Error = &'static str;

            fn try_from(payload: Vec<(K, V)>) -> std::result::Result<Self, Self::Error> {
                Self::with_hasher(payload, BH::default())
            }
        }

        impl<K, V, S, const N: usize, BH> TryFrom<[(K, V); N]> for $map_name<K, V, S, BH>
        where
            K: Hash + Eq,
            S: PrimInt + Unsigned,
            BH: BuildHasher + Default,
        {
            type Error = &'static str;

            fn try_from(payload: [(K, V); N]) -> std::result::Result<Self, Self::Error> {
                Self::with_hasher(Vec::from_iter(payload), BH::default())
            }
        }

        impl<K, V, S, BH> FromIterator<(K, V)> for $map_name<K, V, S, BH>
        where
            K: Hash + Eq,
            S: PrimInt + Unsigned,
            BH: BuildHasher + Default,
        {
            fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
                Self::with_hasher(Vec::from_iter(iter), BH::default()).unwrap()
            }
        }

        impl<K, V, S, BH> Map<K, V> for $map_name<K, V, S, BH>
        where
            K: Hash + Eq,
            S: PrimInt + Unsigned,
            BH: BuildHasher,
        {
            type Iterator<'a> = Iter<'a, K, V>
                                                                                    where
                                                                                        K: 'a,
                                                                                        V: 'a,
                                                                                        S: 'a,
                                                                                        BH: 'a;

            type KeyIterator<'a> = Keys<'a, K, V>
                                                                                    where
                                                                                        K: 'a,
                                                                                        V: 'a,
                                                                                        S: 'a,
                                                                                        BH: 'a;

            type ValueIterator<'a> = Values<'a, K, V>
                                                                                    where
                                                                                        K: 'a,
                                                                                        V: 'a,
                                                                                        S: 'a,
                                                                                        BH: 'a;

            type IntoKeyIterator = IntoKeys<K, V>;
            type IntoValueIterator = IntoValues<K, V>;

            type MutIterator<'a> = IterMut<'a, K, V>
                                                                                    where
                                                                                        K: 'a,
                                                                                        V: 'a,
                                                                                        S: 'a,
                                                                                        BH: 'a;

            type ValueMutIterator<'a> = ValuesMut<'a, K, V>
                                                                                    where
                                                                                        K: 'a,
                                                                                        V: 'a,
                                                                                        S: 'a,
                                                                                        BH: 'a;

            #[inline]
            fn iter(&self) -> Self::Iterator<'_> {
                Iter::new(&self.table.entries)
            }

            #[inline]
            fn keys(&self) -> Self::KeyIterator<'_> {
                Keys::new(&self.table.entries)
            }

            #[inline]
            fn values(&self) -> Self::ValueIterator<'_> {
                Values::new(&self.table.entries)
            }

            #[inline]
            fn into_keys(self) -> Self::IntoKeyIterator {
                IntoKeys::new(self.table.entries)
            }

            #[inline]
            fn into_values(self) -> Self::IntoValueIterator {
                IntoValues::new(self.table.entries)
            }

            #[inline]
            fn iter_mut(&mut self) -> Self::MutIterator<'_> {
                IterMut::new(self.table.entries.as_mut())
            }

            #[inline]
            fn values_mut(&mut self) -> Self::ValueMutIterator<'_> {
                ValuesMut::new(self.table.entries.as_mut())
            }

            #[inline]
            fn contains_key(&self, key: &K) -> bool {
                self.contains_key(key)
            }

            #[inline]
            fn get(&self, key: &K) -> Option<&V> {
                Self::get(self, key)
            }
        }

        #[cfg(test)]
        mod $test_mod {
            use ahash::RandomState;

            use super::*;

            #[test]
            fn test_from_iter_empty() {
                let pairs: Vec<(i32, i32)> = vec![];
                let map: $map_name<i32, i32, u32, RandomState> = pairs.into_iter().collect();
                assert!(map.is_empty());
            }

            #[test]
            fn test_from_iter_single() {
                let pairs = vec![(1, 2)];
                let map: $map_name<i32, i32, u32, RandomState> = pairs.into_iter().collect();
                assert_eq!(map.get(&1), Some(&2));
            }

            #[test]
            fn test_from_iter_multiple() {
                let pairs = vec![(1, 2), (3, 4), (5, 6)];
                let map: $map_name<i32, i32, u32, RandomState> = pairs.into_iter().collect();
                assert_eq!(map.get(&1), Some(&2));
                assert_eq!(map.get(&3), Some(&4));
                assert_eq!(map.get(&5), Some(&6));
            }
        }
    };
}

common_map!(CommonMap, false, tests);
common_map!(CommonMapNoCollisions, true, tests_no_collisions);
