use crate::facade_maps::FacadeStringMap;
use crate::hashers::{LeftRangeHasher, RightRangeHasher};
use crate::sets::decl_macros::{
    bitand_fn, bitor_fn, bitxor_fn, debug_fn, into_iter_fn, into_iter_ref_fn, partial_eq_fn,
    set_boilerplate, set_iterator_boilerplate, sub_fn,
};
use crate::sets::{IntoIter, Iter};
use crate::traits::{Hasher, Len, MapIterator, Set, SetIterator};
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

impl<T, BH> FacadeStringSet<T, BH> {
    #[doc = include_str!("../doc_snippets/get_from_set_method.md")]
    #[inline]
    #[must_use]
    pub fn get<Q>(&self, value: &Q) -> Option<&T>
    where
        Q: ?Sized + Hash + Eq + Len + Equivalent<T>,
        BH: BuildHasher,
        LeftRangeHasher<BH>: Hasher<Q>,
        RightRangeHasher<BH>: Hasher<Q>,
    {
        Some(self.map.get_key_value(value)?.0)
    }

    #[doc = include_str!("../doc_snippets/contains_method.md")]
    #[inline]
    #[must_use]
    pub fn contains<Q>(&self, value: &Q) -> bool
    where
        Q: ?Sized + Hash + Eq + Len + Equivalent<T>,
        BH: BuildHasher,
        LeftRangeHasher<BH>: Hasher<Q>,
        RightRangeHasher<BH>: Hasher<Q>,
    {
        self.get(value).is_some()
    }
}

impl<T, BH> Len for FacadeStringSet<T, BH> {
    fn len(&self) -> usize {
        self.map.len()
    }
}

impl<T, BH> Debug for FacadeStringSet<T, BH>
where
    T: Debug,
{
    debug_fn!();
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

impl<T, BH> IntoIterator for FacadeStringSet<T, BH> {
    into_iter_fn!();
}

impl<'a, T, BH> IntoIterator for &'a FacadeStringSet<T, BH> {
    into_iter_ref_fn!();
}

impl<T, BH> SetIterator<T> for FacadeStringSet<T, BH> {
    type Iterator<'a>
        = Iter<'a, T>
    where
        T: 'a,
        BH: 'a;

    set_iterator_boilerplate!();
}

impl<T, BH> Set<T> for FacadeStringSet<T, BH>
where
    T: Hash + Eq + Len,
    BH: BuildHasher,
    LeftRangeHasher<BH>: Hasher<T>,
    RightRangeHasher<BH>: Hasher<T>,
{
    set_boilerplate!();
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
