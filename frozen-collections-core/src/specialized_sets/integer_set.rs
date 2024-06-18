use std::borrow::Borrow;
use std::collections::HashSet;
use std::fmt::{Debug, Formatter, Result};
use std::hash::Hash;
use std::ops::{BitAnd, BitOr, BitXor, Sub};

use num_traits::{AsPrimitive, PrimInt, Unsigned};

use crate::analyzers::HashCodeAnalysisResult;
use crate::specialized_maps::{IntegerMap, IntegerMapNoCollisions};
use crate::specialized_sets::utils::partial_eq;
use crate::specialized_sets::{IntoIter, Iter};
use crate::traits::Len;
use crate::traits::Set;

macro_rules! integer_set {
    ($set_name:ident, $map_name:ident) => {
        /// A set whose values are integers.
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
        /// to use the `FrozenIntSet` type or the `frozen_set!` macro.
        #[derive(Clone)]
        pub struct $set_name<T, S = u8> {
            map: $map_name<T, (), S>,
        }

        impl<T, S> $set_name<T, S>
        where
            T: PrimInt + AsPrimitive<u64> + Hash + Eq,
            S: PrimInt + Unsigned,
        {
            #[allow(clippy::from_iter_instead_of_collect)]
            #[allow(clippy::missing_errors_doc)]
            pub fn new(payload: Vec<T>) -> std::result::Result<Self, &'static str> {
                Ok(Self {
                    map: $map_name::new(Vec::from_iter(payload.into_iter().map(|x| (x, ()))))?,
                })
            }

            /// PRIVATE: used by macros, subject to change.
            #[allow(clippy::from_iter_instead_of_collect)]
            #[allow(clippy::missing_errors_doc)]
            #[doc(hidden)]
            pub fn with_analysis(
                payload: Vec<T>,
                code_analysis: HashCodeAnalysisResult,
            ) -> std::result::Result<Self, &'static str> {
                Ok(Self {
                    map: $map_name::with_analysis(
                        Vec::from_iter(payload.into_iter().map(|x| (x, ()))),
                        code_analysis,
                    )?,
                })
            }
        }

        impl<T, S> $set_name<T, S>
        where
            S: PrimInt + Unsigned,
        {
            #[inline]
            #[must_use]
            pub fn get<Q>(&self, value: &Q) -> Option<&T>
            where
                T: Borrow<Q>,
                Q: PrimInt + AsPrimitive<u64>,
            {
                Some(self.map.get_key_value(value)?.0)
            }

            #[inline]
            #[must_use]
            pub fn contains<Q>(&self, value: &Q) -> bool
            where
                T: Borrow<Q>,
                Q: PrimInt + AsPrimitive<u64>,
            {
                self.get(value).is_some()
            }
        }

        impl<T, S> Len for $set_name<T, S> {
            fn len(&self) -> usize {
                self.map.len()
            }
        }

        impl<T, S> Debug for $set_name<T, S>
        where
            T: PrimInt + AsPrimitive<u64> + Debug,
            S: PrimInt + Unsigned,
        {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                f.debug_set().entries(self.iter()).finish()
            }
        }

        impl<T, S> Default for $set_name<T, S>
        where
            S: PrimInt + Unsigned,
        {
            fn default() -> Self {
                Self {
                    map: $map_name::default(),
                }
            }
        }

        impl<T, S> IntoIterator for $set_name<T, S> {
            type Item = T;
            type IntoIter = IntoIter<T>;

            fn into_iter(self) -> Self::IntoIter {
                IntoIter::new(self.map.table.entries)
            }
        }

        impl<'a, T, S> IntoIterator for &'a $set_name<T, S>
        where
            T: PrimInt + AsPrimitive<u64>,
            S: PrimInt + Unsigned,
        {
            type Item = &'a T;
            type IntoIter = Iter<'a, T>;

            fn into_iter(self) -> Self::IntoIter {
                self.iter()
            }
        }

        impl<T, S> TryFrom<Vec<T>> for $set_name<T, S>
        where
            T: PrimInt + AsPrimitive<u64> + Hash + Eq,
            S: PrimInt + Unsigned,
        {
            type Error = &'static str;

            #[allow(clippy::from_iter_instead_of_collect)]
            fn try_from(payload: Vec<T>) -> std::result::Result<Self, Self::Error> {
                Self::new(payload)
            }
        }

        impl<T, S, const N: usize> TryFrom<[T; N]> for $set_name<T, S>
        where
            T: PrimInt + AsPrimitive<u64> + Hash + Eq,
            S: PrimInt + Unsigned,
        {
            type Error = &'static str;

            #[allow(clippy::from_iter_instead_of_collect)]
            fn try_from(payload: [T; N]) -> std::result::Result<Self, Self::Error> {
                Ok(Self {
                    map: $map_name::try_from(Vec::from_iter(payload.into_iter().map(|x| (x, ()))))?,
                })
            }
        }

        impl<T, S> FromIterator<T> for $set_name<T, S>
        where
            T: PrimInt + AsPrimitive<u64> + Hash + Eq,
            S: PrimInt + Unsigned,
        {
            fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
                Self {
                    map: iter.into_iter().map(|x| (x, ())).collect(),
                }
            }
        }

        impl<T, S> Set<T> for $set_name<T, S>
        where
            T: PrimInt + AsPrimitive<u64>,
            S: PrimInt + Unsigned,
        {
            type Iterator<'a> = Iter<'a, T>
                                                                                    where
                                                                                        T: 'a,
                                                                                        S: 'a;

            fn iter(&self) -> Iter<'_, T> {
                Iter::new(&self.map.table.entries)
            }

            fn contains(&self, value: &T) -> bool {
                self.contains(value)
            }
        }

        impl<T, S, ST> BitOr<&ST> for &$set_name<T, S>
        where
            T: PrimInt + AsPrimitive<u64> + Clone + Hash,
            S: PrimInt + Unsigned,
            ST: Set<T>,
        {
            type Output = HashSet<T>;

            fn bitor(self, rhs: &ST) -> Self::Output {
                self.union(rhs).copied().collect()
            }
        }

        impl<T, S, ST> BitAnd<&ST> for &$set_name<T, S>
        where
            T: PrimInt + AsPrimitive<u64> + Clone + Hash,
            S: PrimInt + Unsigned,
            ST: Set<T>,
        {
            type Output = HashSet<T>;

            fn bitand(self, rhs: &ST) -> Self::Output {
                self.intersection(rhs).copied().collect()
            }
        }

        impl<T, S, ST> BitXor<&ST> for &$set_name<T, S>
        where
            T: PrimInt + AsPrimitive<u64> + Clone + Hash,
            S: PrimInt + Unsigned,
            ST: Set<T>,
        {
            type Output = HashSet<T>;

            fn bitxor(self, rhs: &ST) -> Self::Output {
                self.symmetric_difference(rhs).copied().collect()
            }
        }

        impl<T, S, ST> Sub<&ST> for &$set_name<T, S>
        where
            T: PrimInt + AsPrimitive<u64> + Clone + Hash,
            S: PrimInt + Unsigned,
            ST: Set<T>,
        {
            type Output = HashSet<T>;

            fn sub(self, rhs: &ST) -> Self::Output {
                self.difference(rhs).copied().collect()
            }
        }

        impl<T, S, ST> PartialEq<ST> for $set_name<T, S>
        where
            T: PrimInt + AsPrimitive<u64> + Hash,
            S: PrimInt + Unsigned,
            ST: Set<T>,
        {
            partial_eq!();
        }

        impl<T, S> Eq for $set_name<T, S>
        where
            T: PrimInt + AsPrimitive<u64> + Hash,
            S: PrimInt + Unsigned,
        {
        }
    };
}

integer_set!(IntegerSet, IntegerMap);
integer_set!(IntegerSetNoCollisions, IntegerMapNoCollisions);
