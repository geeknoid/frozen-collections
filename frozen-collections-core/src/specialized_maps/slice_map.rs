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
use crate::traits::RangeHash;
use crate::traits::{Len, Map};

macro_rules! slice_map {
    ($map_name:ident, $left:literal, $no_collisions:literal) => {
        /// A map that hashes aligned slices of its keys.
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
        pub struct $map_name<K, V, S = u8, BH = RandomState> {
            pub(crate) table: HashTable<K, V, S>,
            bh: BH,
            range: Range<usize>,
        }

        impl<K, V, S, BH> $map_name<K, V, S, BH>
        where
            K: RangeHash + Len + Hash + Eq,
            S: PrimInt + Unsigned,
            BH: BuildHasher + Default,
        {
            #[allow(clippy::missing_errors_doc)]
            pub fn new(
                payload: Vec<(K, V)>,
                range: Range<usize>,
            ) -> std::result::Result<Self, &'static str> {
                Self::with_hasher(payload, range, BH::default())
            }
        }

        impl<K, V, S, BH> $map_name<K, V, S, BH>
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
                        key.hash_range(&bh, Self::get_hash_range(&range, key.len()))
                    } else {
                        0
                    }
                });
                let code_analysis = analyze_hash_codes(codes);

                Self::with_hasher_and_analysis(payload, range, bh, code_analysis)
            }

            /// PRIVATE: used by macros, subject to change
            #[allow(clippy::missing_errors_doc)]
            #[doc(hidden)]
            pub fn with_hasher_and_analysis(
                payload: Vec<(K, V)>,
                range: Range<usize>,
                bh: BH,
                code_analysis: HashCodeAnalysisResult,
            ) -> std::result::Result<Self, &'static str> {
                Ok(Self {
                    table: HashTable::new(payload, code_analysis.num_hash_slots, |key| {
                        key.hash_range(&bh, Self::get_hash_range(&range, key.len()))
                    })?,
                    bh,
                    range,
                })
            }
        }

        impl<K, V, S, BH> $map_name<K, V, S, BH>
        where
            S: PrimInt + Unsigned,
            BH: BuildHasher,
        {
            #[inline]
            const fn get_hash_range(range: &Range<usize>, len: usize) -> Range<usize> {
                if $left {
                    range.start..range.end
                } else {
                    len - range.end..len - range.start
                }
            }

            #[inline]
            #[must_use]
            fn get_hash_info<Q>(&self, key: &Q) -> Range<usize>
            where
                Q: ?Sized + RangeHash + Len,
            {
                let hash_code = if key.len() >= self.range.end {
                    unsafe {
                        key.hash_range_unchecked(
                            &self.bh,
                            Self::get_hash_range(&self.range, key.len()),
                        )
                    }
                } else {
                    0
                };

                self.table.get_hash_info(hash_code)
            }

            #[inline]
            #[must_use]
            #[allow(clippy::redundant_else)]
            pub fn get<Q>(&self, key: &Q) -> Option<&V>
            where
                K: Borrow<Q>,
                Q: ?Sized + RangeHash + Len + Eq,
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
                Q: ?Sized + RangeHash + Len + Eq,
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
                Q: ?Sized + RangeHash + Len + Eq,
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
                Q: ?Sized + RangeHash + Len + Eq,
            {
                get_many_mut!(self, keys);
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
                    range: Range::default(),
                }
            }
        }

        impl<Q, K, V, S, BH> Index<&Q> for $map_name<K, V, S, BH>
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

        impl<Q, K, V, S, BH> IndexMut<&Q> for $map_name<K, V, S, BH>
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

        impl<K, V, S, BH> IntoIterator for $map_name<K, V, S, BH> {
            type Item = (K, V);
            type IntoIter = IntoIter<K, V>;

            fn into_iter(self) -> Self::IntoIter {
                IntoIter::new(self.table.entries)
            }
        }

        impl<'a, K, V, S, BH> IntoIterator for &'a $map_name<K, V, S, BH>
        where
            K: RangeHash + Len + Eq,
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
            K: RangeHash + Len + Eq,
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
            K: RangeHash + Len + Eq,
            V: PartialEq,
            S: PrimInt + Unsigned,
            MT: Map<K, V>,
            BH: BuildHasher,
        {
            partial_eq!();
        }

        impl<K, V, S, BH> Eq for $map_name<K, V, S, BH>
        where
            K: RangeHash + Len + Eq,
            V: Eq,
            S: PrimInt + Unsigned,
            BH: BuildHasher,
        {
        }

        impl<K, V, S, BH> Map<K, V> for $map_name<K, V, S, BH>
        where
            K: RangeHash + Len + Eq,
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
    };
}

slice_map!(LeftSliceMap, true, false);
slice_map!(RightSliceMap, false, false);

slice_map!(LeftSliceMapNoCollisions, true, true);
slice_map!(RightSliceMapNoCollisions, false, true);
