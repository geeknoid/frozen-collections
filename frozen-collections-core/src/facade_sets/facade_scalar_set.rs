use crate::facade_maps::FacadeScalarMap;
use crate::sets::decl_macros::{
    bitand_fn, bitor_fn, bitxor_fn, debug_fn, into_iter_fn, into_iter_ref_fn, partial_eq_fn,
    set_boilerplate, set_iterator_boilerplate, sub_fn,
};
use crate::sets::{IntoIter, Iter};
use crate::traits::{Len, MapIterator, Scalar, Set, SetIterator};
use core::fmt::Debug;
use core::hash::Hash;
use core::ops::{BitAnd, BitOr, BitXor, Sub};
use equivalent::Equivalent;

/// A set optimized for fast read access with integer or enum values.
///
#[doc = include_str!("../doc_snippets/type_compat_warning.md")]
#[doc = include_str!("../doc_snippets/about.md")]
#[derive(Clone)]
pub struct FacadeScalarSet<T> {
    map: FacadeScalarMap<T, ()>,
}

impl<T> FacadeScalarSet<T>
where
    T: Scalar,
{
    /// Creates a new frozen set.
    #[must_use]
    pub const fn new(map: FacadeScalarMap<T, ()>) -> Self {
        Self { map }
    }
}

impl<T> FacadeScalarSet<T> {
    #[doc = "Returns a reference to the value in the set, if any, that is equal to the given value.\n"]
    #[inline]
    #[must_use]
    pub fn get<Q>(&self, value: &Q) -> Option<&T>
    where
        Q: Scalar + Eq + Equivalent<T>,
    {
        Some(self.map.get_key_value(value)?.0)
    }

    #[doc = "Returns `true` if the set contains a value.\n"]
    #[inline]
    #[must_use]
    pub fn contains<Q>(&self, value: &Q) -> bool
    where
        Q: Scalar + Eq + Equivalent<T>,
    {
        self.get(value).is_some()
    }
}

impl<T> Len for FacadeScalarSet<T> {
    fn len(&self) -> usize {
        self.map.len()
    }
}

impl<T> Debug for FacadeScalarSet<T>
where
    T: Debug,
{
    debug_fn!();
}

impl<T> Default for FacadeScalarSet<T> {
    fn default() -> Self {
        Self {
            map: FacadeScalarMap::default(),
        }
    }
}

impl<T> IntoIterator for FacadeScalarSet<T> {
    into_iter_fn!();
}

impl<'a, T> IntoIterator for &'a FacadeScalarSet<T> {
    into_iter_ref_fn!();
}

impl<T> SetIterator<T> for FacadeScalarSet<T> {
    type Iterator<'a>
        = Iter<'a, T>
    where
        T: 'a;

    set_iterator_boilerplate!();
}

impl<T> Set<T> for FacadeScalarSet<T>
where
    T: Scalar,
{
    set_boilerplate!();
}

impl<T, ST> BitOr<&ST> for &FacadeScalarSet<T>
where
    T: Hash + Eq + Scalar + Clone,
    ST: Set<T>,
{
    bitor_fn!(RandomState);
}

impl<T, ST> BitAnd<&ST> for &FacadeScalarSet<T>
where
    T: Hash + Eq + Scalar + Clone,
    ST: Set<T>,
{
    bitand_fn!(RandomState);
}

impl<T, ST> BitXor<&ST> for &FacadeScalarSet<T>
where
    T: Hash + Eq + Scalar + Clone,
    ST: Set<T>,
{
    bitxor_fn!(RandomState);
}

impl<T, ST> Sub<&ST> for &FacadeScalarSet<T>
where
    T: Hash + Eq + Scalar + Clone,
    ST: Set<T>,
{
    sub_fn!(RandomState);
}

impl<T, ST> PartialEq<ST> for FacadeScalarSet<T>
where
    T: Scalar,
    ST: Set<T>,
{
    partial_eq_fn!();
}

impl<T> Eq for FacadeScalarSet<T> where T: Scalar {}
