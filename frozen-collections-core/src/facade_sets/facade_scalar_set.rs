use crate::facade_maps::FacadeScalarMap;
use crate::sets::decl_macros::{
    bitand_fn, bitor_fn, bitxor_fn, debug_fn, into_iter_fn, into_iter_ref_fn, partial_eq_fn,
    set_iteration_funcs, sub_fn,
};
use crate::sets::{IntoIter, Iter};
use crate::traits::{Len, MapIteration, MapQuery, Scalar, Set, SetIteration, SetOps, SetQuery};
use core::fmt::Debug;
use core::hash::Hash;
use core::ops::{BitAnd, BitOr, BitXor, Sub};

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

impl<T> Default for FacadeScalarSet<T> {
    fn default() -> Self {
        Self {
            map: FacadeScalarMap::default(),
        }
    }
}

impl<T> Set<T, T> for FacadeScalarSet<T> where T: Scalar {}

impl<T> SetQuery<T, T> for FacadeScalarSet<T>
where
    T: Scalar,
{
    #[inline]
    fn get(&self, value: &T) -> Option<&T> {
        Some(self.map.get_key_value(value)?.0)
    }
}

impl<T> SetIteration<T> for FacadeScalarSet<T> {
    type Iterator<'a>
        = Iter<'a, T>
    where
        T: 'a;

    set_iteration_funcs!();
}

impl<T> Len for FacadeScalarSet<T> {
    fn len(&self) -> usize {
        self.map.len()
    }
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

impl<T> IntoIterator for FacadeScalarSet<T> {
    into_iter_fn!();
}

impl<'a, T> IntoIterator for &'a FacadeScalarSet<T> {
    into_iter_ref_fn!();
}

impl<T, ST> PartialEq<ST> for FacadeScalarSet<T>
where
    T: Scalar,
    ST: Set<T>,
{
    partial_eq_fn!();
}

impl<T> Eq for FacadeScalarSet<T> where T: Scalar {}

impl<T> Debug for FacadeScalarSet<T>
where
    T: Debug,
{
    debug_fn!();
}
