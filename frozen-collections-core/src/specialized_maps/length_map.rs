use std::borrow::Borrow;
use std::fmt::{Debug, Formatter, Result};
use std::hash::Hash;
use std::ops::Range;
use std::ops::{Index, IndexMut};

use bitvec::macros::internal::funty::Fundamental;
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

macro_rules! length_map {
    ($map_name:ident, $no_collisions:literal, $test_mod:ident) => {
        /// A map that uses key lengths as hash codes.
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
        /// to use the `FrozenStringMap` type or the `frozen_map!` macro.
        #[derive(Clone)]
        pub struct $map_name<K, V, S = u8> {
            pub(crate) table: HashTable<K, V, S>,
        }

        impl<K, V, S> $map_name<K, V, S>
        where
            K: Len + Hash + Eq,
            S: PrimInt + Unsigned,
        {
            #[allow(clippy::missing_errors_doc)]
            pub fn new(payload: Vec<(K, V)>) -> std::result::Result<Self, &'static str> {
                check_duplicate_keys(payload.iter().map(|entry| &entry.0))?;

                let code_analysis =
                    analyze_hash_codes(payload.iter().map(|entry| entry.0.len().as_u64()));

                Self::with_analysis(payload, code_analysis)
            }

            /// PRIVATE: used by macros, subject to change
            #[allow(clippy::missing_errors_doc)]
            #[doc(hidden)]
            pub fn with_analysis(
                payload: Vec<(K, V)>,
                code_analysis: HashCodeAnalysisResult,
            ) -> std::result::Result<Self, &'static str> {
                Ok(Self {
                    table: HashTable::new(payload, code_analysis.num_hash_slots, |k| {
                        k.len() as u64
                    })?,
                })
            }
        }

        impl<K, V, S> $map_name<K, V, S>
        where
            S: PrimInt + Unsigned,
        {
            #[inline]
            #[must_use]
            fn get_hash_info<Q>(&self, key: &Q) -> Range<usize>
            where
                Q: ?Sized + Len,
            {
                let hash_code = key.len().as_u64();
                self.table.get_hash_info(hash_code)
            }

            #[inline]
            #[must_use]
            #[allow(clippy::redundant_else)]
            pub fn get<Q>(&self, key: &Q) -> Option<&V>
            where
                K: Borrow<Q>,
                Q: ?Sized + Len + Eq,
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
                Q: ?Sized + Len + Eq,
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
                Q: ?Sized + Len + Eq,
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
                Q: ?Sized + Len + Eq,
            {
                get_many_mut!(self, keys);
            }

            #[inline]
            #[must_use]
            pub fn contains_key<Q>(&self, key: &Q) -> bool
            where
                K: Borrow<Q>,
                Q: ?Sized + Len + Eq,
            {
                self.get(key).is_some()
            }
        }

        impl<K, V, S> Len for $map_name<K, V, S> {
            fn len(&self) -> usize {
                self.table.len()
            }
        }

        impl<K, V, S> Debug for $map_name<K, V, S>
        where
            K: Debug,
            V: Debug,
        {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                self.table.fmt(f)
            }
        }

        impl<K, V, S> Default for $map_name<K, V, S>
        where
            S: PrimInt + Unsigned,
        {
            fn default() -> Self {
                Self {
                    table: HashTable::default(),
                }
            }
        }

        impl<Q, K, V, S> Index<&Q> for $map_name<K, V, S>
        where
            K: Borrow<Q>,
            Q: ?Sized + Len + Eq,
            S: PrimInt + Unsigned,
        {
            type Output = V;

            fn index(&self, index: &Q) -> &Self::Output {
                self.get(index).unwrap()
            }
        }

        impl<Q, K, V, S> IndexMut<&Q> for $map_name<K, V, S>
        where
            K: Borrow<Q>,
            Q: ?Sized + Len + Eq,
            S: PrimInt + Unsigned,
        {
            fn index_mut(&mut self, index: &Q) -> &mut V {
                self.get_mut(index).unwrap()
            }
        }

        impl<K, V, S> IntoIterator for $map_name<K, V, S> {
            type Item = (K, V);
            type IntoIter = IntoIter<K, V>;

            fn into_iter(self) -> Self::IntoIter {
                IntoIter::new(self.table.entries)
            }
        }

        impl<'a, K, V, S> IntoIterator for &'a $map_name<K, V, S>
        where
            K: Len + Hash + Eq,
            S: PrimInt + Unsigned,
        {
            type Item = (&'a K, &'a V);
            type IntoIter = Iter<'a, K, V>;

            fn into_iter(self) -> Self::IntoIter {
                self.iter()
            }
        }

        impl<'a, K, V, S> IntoIterator for &'a mut $map_name<K, V, S>
        where
            K: Len + Hash + Eq,
            S: PrimInt + Unsigned,
        {
            type Item = (&'a K, &'a mut V);
            type IntoIter = IterMut<'a, K, V>;

            fn into_iter(self) -> Self::IntoIter {
                self.iter_mut()
            }
        }

        impl<K, V, S, MT> PartialEq<MT> for $map_name<K, V, S>
        where
            K: Len + Hash + Eq,
            V: PartialEq,
            S: PrimInt + Unsigned,
            MT: Map<K, V>,
        {
            partial_eq!();
        }

        impl<K, V, S> Eq for $map_name<K, V, S>
        where
            K: Len + Eq + Hash,
            V: Eq,
            S: PrimInt + Unsigned,
        {
        }

        impl<K, V, S> TryFrom<Vec<(K, V)>> for $map_name<K, V, S>
        where
            K: Len + Hash + Eq,
            S: PrimInt + Unsigned,
        {
            type Error = &'static str;

            fn try_from(payload: Vec<(K, V)>) -> std::result::Result<Self, Self::Error> {
                Self::new(payload)
            }
        }

        impl<K, V, const N: usize, S> TryFrom<[(K, V); N]> for $map_name<K, V, S>
        where
            K: Len + Hash + Eq,
            S: PrimInt + Unsigned,
        {
            type Error = &'static str;

            fn try_from(payload: [(K, V); N]) -> std::result::Result<Self, Self::Error> {
                Self::try_from(Vec::from_iter(payload))
            }
        }

        impl<K, V, S> FromIterator<(K, V)> for $map_name<K, V, S>
        where
            K: Len + Hash + Eq,
            S: PrimInt + Unsigned,
        {
            fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
                Self::try_from(Vec::from_iter(iter)).unwrap()
            }
        }

        impl<K, V, S> Map<K, V> for $map_name<K, V, S>
        where
            K: Len + Hash + Eq,
            S: PrimInt + Unsigned,
        {
            type Iterator<'a> = Iter<'a, K, V>
                                                                                    where
                                                                                        K: 'a,
                                                                                        V: 'a,
                                                                                        S: 'a;

            type KeyIterator<'a> = Keys<'a, K, V>
                                                                                    where
                                                                                        K: 'a,
                                                                                        V: 'a,
                                                                                        S: 'a;

            type ValueIterator<'a> = Values<'a, K, V>
                                                                                    where
                                                                                        K: 'a,
                                                                                        V: 'a,
                                                                                        S: 'a;

            type IntoKeyIterator = IntoKeys<K, V>;
            type IntoValueIterator = IntoValues<K, V>;

            type MutIterator<'a> = IterMut<'a, K, V>
                                                                                    where
                                                                                        K: 'a,
                                                                                        V: 'a,
                                                                                        S: 'a;

            type ValueMutIterator<'a> = ValuesMut<'a, K, V>
                                                                                    where
                                                                                        K: 'a,
                                                                                        V: 'a,
                                                                                        S: 'a;

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
            use super::*;

            #[test]
            fn test_from_empty() {
                let pairs: [(String, i32); 0] = [];
                let map = $map_name::<String, i32>::try_from(pairs).unwrap();
                assert!(map.is_empty());
            }

            #[test]
            fn test_from_single() {
                let pairs = [("key1".to_string(), 1)];
                let map = $map_name::<String, i32>::try_from(pairs).unwrap();
                assert_eq!(map.get(&"key1".to_string()), Some(&1));
            }

            #[test]
            fn test_from_multiple() {
                let pairs = [
                    ("key1".to_string(), 1),
                    ("key22".to_string(), 2),
                    ("key333".to_string(), 3),
                ];
                let map = $map_name::<String, i32>::try_from(pairs).unwrap();
                assert_eq!(map.get(&"key1".to_string()), Some(&1));
                assert_eq!(map.get(&"key22".to_string()), Some(&2));
                assert_eq!(map.get(&"key333".to_string()), Some(&3));
            }

            #[test]
            fn test_from_iter_empty() {
                let pairs: Vec<(String, i32)> = vec![];
                let map: $map_name<String, i32> = pairs.into_iter().collect();
                assert!(map.is_empty());
            }

            #[test]
            fn test_from_iter_single() {
                let pairs = vec![("key1".to_string(), 1)];
                let map: $map_name<String, i32> = pairs.into_iter().collect();
                assert_eq!(map.get(&"key1".to_string()), Some(&1));
            }

            #[test]
            fn test_from_iter_multiple() {
                let pairs = vec![
                    ("key1".to_string(), 1),
                    ("key22".to_string(), 2),
                    ("key333".to_string(), 3),
                ];
                let map: $map_name<String, i32> = pairs.into_iter().collect();
                assert_eq!(map.get(&"key1".to_string()), Some(&1));
                assert_eq!(map.get(&"key22".to_string()), Some(&2));
                assert_eq!(map.get(&"key333".to_string()), Some(&3));
            }
        }
    };
}

length_map!(LengthMap, false, tests);
length_map!(LengthMapNoCollisions, true, tests_no_collisions);
