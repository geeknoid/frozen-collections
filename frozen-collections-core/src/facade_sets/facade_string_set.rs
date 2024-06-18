use crate::facade_maps::FacadeStringMap;
use crate::hashers::{LeftRangeHasher, RightRangeHasher};
use crate::sets::decl_macros::{
    bitand_fn, bitor_fn, bitxor_fn, debug_fn, into_iter_fn, into_iter_ref_fn, partial_eq_fn,
    set_iteration_funcs, sub_fn,
};
use crate::sets::{IntoIter, Iter};
use crate::traits::{Hasher, Len, MapIteration, MapQuery, Set, SetIteration, SetOps, SetQuery};
use ahash::RandomState;
use core::fmt::Debug;
use core::hash::BuildHasher;
use core::hash::Hash;
use core::ops::{BitAnd, BitOr, BitXor, Sub};
use equivalent::Equivalent;

/// A set optimized for fast read access with string values.
///
#[doc = include_str!("../doc_snippets/type_compat_warning.md")]
#[doc = include_str!("../doc_snippets/about.md")]
#[doc = include_str!("../doc_snippets/hash_warning.md")]
#[derive(Clone)]
pub struct FacadeStringSet<T, BH = RandomState> {
    map: FacadeStringMap<T, (), BH>,
}

impl<'a, BH> FacadeStringSet<&'a str, BH> {
    /// Creates a new frozen set which uses the given hash builder to hash values.
    #[must_use]
    pub const fn new(map: FacadeStringMap<&'a str, (), BH>) -> Self {
        Self { map }
    }
}

impl<T, BH> Default for FacadeStringSet<T, BH>
where
    BH: Default,
{
    fn default() -> Self {
        Self {
            map: FacadeStringMap::default(),
        }
    }
}

impl<T, Q, BH> Set<T, Q> for FacadeStringSet<T, BH>
where
    Q: Hash + Eq + Len + Equivalent<T>,
    BH: BuildHasher,
    LeftRangeHasher<BH>: Hasher<Q>,
    RightRangeHasher<BH>: Hasher<Q>,
{
}

impl<T, Q, BH> SetQuery<T, Q> for FacadeStringSet<T, BH>
where
    Q: Hash + Eq + Len + Equivalent<T>,
    BH: BuildHasher,
    LeftRangeHasher<BH>: Hasher<Q>,
    RightRangeHasher<BH>: Hasher<Q>,
{
    #[inline]
    fn get(&self, value: &Q) -> Option<&T> {
        Some(self.map.get_key_value(value)?.0)
    }
}

impl<T, BH> SetIteration<T> for FacadeStringSet<T, BH> {
    type Iterator<'a>
        = Iter<'a, T>
    where
        T: 'a,
        BH: 'a;

    set_iteration_funcs!();
}

impl<T, BH> Len for FacadeStringSet<T, BH> {
    fn len(&self) -> usize {
        self.map.len()
    }
}

impl<T, ST, BH> BitOr<&ST> for &FacadeStringSet<T, BH>
where
    T: Hash + Eq + Len + Clone,
    ST: Set<T>,
    BH: BuildHasher + Default,
    LeftRangeHasher<BH>: Hasher<T>,
    RightRangeHasher<BH>: Hasher<T>,
{
    bitor_fn!(H);
}

impl<T, ST, BH> BitAnd<&ST> for &FacadeStringSet<T, BH>
where
    T: Hash + Eq + Len + Clone,
    ST: Set<T>,
    BH: BuildHasher + Default,
    LeftRangeHasher<BH>: Hasher<T>,
    RightRangeHasher<BH>: Hasher<T>,
{
    bitand_fn!(H);
}

impl<T, ST, BH> BitXor<&ST> for &FacadeStringSet<T, BH>
where
    T: Hash + Eq + Len + Clone,
    ST: Set<T>,
    BH: BuildHasher + Default,
    LeftRangeHasher<BH>: Hasher<T>,
    RightRangeHasher<BH>: Hasher<T>,
{
    bitxor_fn!(H);
}

impl<T, ST, BH> Sub<&ST> for &FacadeStringSet<T, BH>
where
    T: Hash + Eq + Len + Clone,
    ST: Set<T>,
    BH: BuildHasher + Default,
    LeftRangeHasher<BH>: Hasher<T>,
    RightRangeHasher<BH>: Hasher<T>,
{
    sub_fn!(H);
}

impl<T, BH> IntoIterator for FacadeStringSet<T, BH> {
    into_iter_fn!();
}

impl<'a, T, BH> IntoIterator for &'a FacadeStringSet<T, BH> {
    into_iter_ref_fn!();
}

impl<T, ST, BH> PartialEq<ST> for FacadeStringSet<T, BH>
where
    T: Hash + Eq + Len,
    ST: Set<T>,
    BH: BuildHasher + Default,
    LeftRangeHasher<BH>: Hasher<T>,
    RightRangeHasher<BH>: Hasher<T>,
{
    partial_eq_fn!();
}

impl<T, BH> Eq for FacadeStringSet<T, BH>
where
    T: Hash + Eq + Len,
    BH: BuildHasher + Default,
    LeftRangeHasher<BH>: Hasher<T>,
    RightRangeHasher<BH>: Hasher<T>,
{
}

impl<T, BH> Debug for FacadeStringSet<T, BH>
where
    T: Debug,
{
    debug_fn!();
}
