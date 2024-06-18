use std::borrow::Borrow;
use std::collections::HashSet;
use std::fmt::{Debug, Formatter, Result};
use std::hash::{BuildHasher, Hash};
use std::ops::{BitAnd, BitOr, BitXor, Sub};

use ahash::RandomState;
use num_traits::{PrimInt, Unsigned};

use crate::analyzers::HashCodeAnalysisResult;
use crate::specialized_maps::{CommonMap, CommonMapNoCollisions};
use crate::specialized_sets::utils::partial_eq;
use crate::specialized_sets::{IntoIter, Iter};
use crate::traits::Len;
use crate::traits::Set;

macro_rules! common_set {
    ($set_name:ident, $map_name:ident) => {
        /// A general-purpose set.
        ///
        /// # Capacity Constraints
        ///
        /// The `S` generic argument controls the maximum capacity
        /// of the set. A `u8` will allow up to 255 elements, `u16`
        /// will allow up to 65,535 elements, and `usize` will allow
        /// up to [`usize::MAX`] elements.
        ///
        /// # Important Note
        ///
        /// This type is not intended to be used directly by
        /// application code. Instead, applications are expected
        /// to use the `FrozenSet` type or the `frozen_set!` macro.
        #[derive(Clone)]
        pub struct $set_name<T, S = u8, BH = RandomState> {
            map: $map_name<T, (), S, BH>,
        }

        impl<T, S> $set_name<T, S, RandomState>
        where
            T: Hash + Eq,
            S: PrimInt + Unsigned,
        {
            #[allow(clippy::missing_errors_doc)]
            pub fn new(payload: Vec<T>) -> std::result::Result<Self, &'static str> {
                Self::with_hasher(payload, RandomState::new())
            }
        }

        impl<T, S, BH> $set_name<T, S, BH>
        where
            T: Hash + Eq,
            S: PrimInt + Unsigned,
            BH: BuildHasher,
        {
            #[allow(clippy::missing_errors_doc)]
            pub fn with_hasher(payload: Vec<T>, bh: BH) -> std::result::Result<Self, &'static str> {
                Ok(Self {
                    map: $map_name::with_hasher(
                        payload.into_iter().map(|x| (x, ())).collect(),
                        bh,
                    )?,
                })
            }

            /// PRIVATE: Used by macros, subject to change.
            #[allow(clippy::missing_errors_doc)]
            #[doc(hidden)]
            pub fn with_hasher_and_analysis(
                payload: Vec<T>,
                bh: BH,
                code_analysis: HashCodeAnalysisResult,
            ) -> std::result::Result<Self, &'static str> {
                Ok(Self {
                    map: $map_name::with_hasher_and_analysis(
                        payload.into_iter().map(|x| (x, ())).collect(),
                        bh,
                        code_analysis,
                    )?,
                })
            }
        }

        impl<T, S, BH> $set_name<T, S, BH>
        where
            S: PrimInt + Unsigned,
            BH: BuildHasher,
        {
            #[inline]
            #[must_use]
            pub fn get<Q>(&self, value: &Q) -> Option<&T>
            where
                T: Borrow<Q>,
                Q: ?Sized + Hash + Eq,
            {
                Some(self.map.get_key_value(value)?.0)
            }

            #[inline]
            #[must_use]
            pub fn contains<Q>(&self, value: &Q) -> bool
            where
                T: Borrow<Q>,
                Q: ?Sized + Hash + Eq,
            {
                self.get(value).is_some()
            }
        }

        impl<T, S, BH> $set_name<T, S, BH> {
            /// Returns the hasher for this set.
            #[must_use]
            pub const fn hasher(&self) -> &BH {
                self.map.hasher()
            }
        }

        impl<T, S, BH> Len for $set_name<T, S, BH> {
            fn len(&self) -> usize {
                self.map.len()
            }
        }

        impl<T, S, BH> Debug for $set_name<T, S, BH>
        where
            T: Hash + Eq + Debug,
            S: PrimInt + Unsigned,
            BH: BuildHasher,
        {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                f.debug_set().entries(self.iter()).finish()
            }
        }

        impl<T, S, BH> Default for $set_name<T, S, BH>
        where
            S: PrimInt + Unsigned,
            BH: BuildHasher + Default,
        {
            fn default() -> Self {
                Self {
                    map: $map_name::default(),
                }
            }
        }

        impl<T, S, BH> IntoIterator for $set_name<T, S, BH> {
            type Item = T;
            type IntoIter = IntoIter<T>;

            fn into_iter(self) -> Self::IntoIter {
                IntoIter::new(self.map.table.entries)
            }
        }

        impl<'a, T, S, BH> IntoIterator for &'a $set_name<T, S, BH>
        where
            T: Hash + Eq,
            S: PrimInt + Unsigned,
            BH: BuildHasher,
        {
            type Item = &'a T;
            type IntoIter = Iter<'a, T>;

            fn into_iter(self) -> Self::IntoIter {
                self.iter()
            }
        }

        impl<T, S, BH> TryFrom<Vec<T>> for $set_name<T, S, BH>
        where
            T: Hash + Eq,
            S: PrimInt + Unsigned,
            BH: BuildHasher + Default,
        {
            type Error = &'static str;

            fn try_from(payload: Vec<T>) -> std::result::Result<Self, Self::Error> {
                Self::with_hasher(payload, BH::default())
            }
        }

        impl<T, S, const N: usize, BH> TryFrom<[T; N]> for $set_name<T, S, BH>
        where
            T: Hash + Eq,
            S: PrimInt + Unsigned,
            BH: BuildHasher + Default,
        {
            type Error = &'static str;

            fn try_from(payload: [T; N]) -> std::result::Result<Self, Self::Error> {
                Ok(Self {
                    map: $map_name::try_from(Vec::from_iter(payload.into_iter().map(|x| (x, ()))))?,
                })
            }
        }

        impl<T, S, BH> FromIterator<T> for $set_name<T, S, BH>
        where
            T: Hash + Eq,
            S: PrimInt + Unsigned,
            BH: BuildHasher + Default,
        {
            fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
                Self {
                    map: $map_name::try_from(Vec::from_iter(iter.into_iter().map(|x| (x, ()))))
                        .unwrap(),
                }
            }
        }

        impl<T, S, BH> Set<T> for $set_name<T, S, BH>
        where
            T: Hash + Eq,
            S: PrimInt + Unsigned,
            BH: BuildHasher,
        {
            type Iterator<'a> = Iter<'a, T>
                                                                                    where
                                                                                        T: 'a,
                                                                                        S: 'a,
                                                                                        BH: 'a;

            fn iter(&self) -> Iter<'_, T> {
                Iter::new(&self.map.table.entries)
            }

            fn contains(&self, value: &T) -> bool {
                self.contains(value)
            }
        }

        impl<T, S, ST, BH> BitOr<&ST> for &$set_name<T, S, BH>
        where
            T: Hash + Eq + Clone,
            S: PrimInt + Unsigned,
            ST: Set<T>,
            BH: BuildHasher + Default,
        {
            type Output = HashSet<T, BH>;

            fn bitor(self, rhs: &ST) -> Self::Output {
                self.union(rhs).cloned().collect()
            }
        }

        impl<T, S, ST, BH> BitAnd<&ST> for &$set_name<T, S, BH>
        where
            T: Hash + Eq + Clone,
            S: PrimInt + Unsigned,
            ST: Set<T>,
            BH: BuildHasher + Default,
        {
            type Output = HashSet<T, BH>;

            fn bitand(self, rhs: &ST) -> Self::Output {
                self.intersection(rhs).cloned().collect()
            }
        }

        impl<T, S, ST, BH> BitXor<&ST> for &$set_name<T, S, BH>
        where
            T: Hash + Eq + Clone,
            S: PrimInt + Unsigned,
            ST: Set<T>,
            BH: BuildHasher + Default,
        {
            type Output = HashSet<T, BH>;

            fn bitxor(self, rhs: &ST) -> Self::Output {
                self.symmetric_difference(rhs).cloned().collect()
            }
        }

        impl<T, S, ST, BH> Sub<&ST> for &$set_name<T, S, BH>
        where
            T: Hash + Eq + Clone,
            S: PrimInt + Unsigned,
            ST: Set<T>,
            BH: BuildHasher + Default,
        {
            type Output = HashSet<T, BH>;

            fn sub(self, rhs: &ST) -> Self::Output {
                self.difference(rhs).cloned().collect()
            }
        }

        impl<T, S, ST, BH> PartialEq<ST> for $set_name<T, S, BH>
        where
            T: Hash + Eq,
            S: PrimInt + Unsigned,
            ST: Set<T>,
            BH: BuildHasher + Default,
        {
            partial_eq!();
        }

        impl<T, S, BH> Eq for $set_name<T, S, BH>
        where
            T: Hash + Eq,
            S: PrimInt + Unsigned,
            BH: BuildHasher + Default,
        {
        }
    };
}

common_set!(CommonSet, CommonMap);
common_set!(CommonSetNoCollisions, CommonMapNoCollisions);
