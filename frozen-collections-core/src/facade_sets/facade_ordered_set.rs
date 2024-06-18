use crate::facade_maps::FacadeOrderedMap;
use crate::sets::decl_macros::{
    bitand_fn, bitor_fn, bitxor_fn, contains_fn, debug_fn, get_fn, into_iter_fn, into_iter_ref_fn,
    partial_eq_fn, set_boilerplate, set_iterator_boilerplate, sub_fn,
};
use crate::sets::{IntoIter, Iter};
use crate::traits::{Len, MapIterator, Set, SetIterator};
use core::borrow::Borrow;
use core::fmt::Debug;
use core::hash::Hash;
use core::ops::{BitAnd, BitOr, BitXor, Sub};

/// A set optimized for fast read access with ordered values.
///
#[doc = include_str!("../doc_snippets/type_compat_warning.md")]
#[doc = include_str!("../doc_snippets/about.md")]
#[doc = include_str!("../doc_snippets/order_warning.md")]
///
/// # Alternate Choices
///
/// If your values are integers or enum variants, you should use the [`FacadeScalarSet`](crate::facade_sets::FacadeScalarSet) type instead.
/// If your values are strings, you should use the [`FacadeStringSet`](crate::facade_sets::FacadeStringSet) type instead. Both of these will
/// deliver better performance since they are specifically optimized for those value types.
#[derive(Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct FacadeOrderedSet<T> {
    map: FacadeOrderedMap<T, ()>,
}

impl<T> FacadeOrderedSet<T>
where
    T: Ord + Eq,
{
    /// Creates a new frozen ordered set.
    #[must_use]
    pub const fn new(map: FacadeOrderedMap<T, ()>) -> Self {
        Self { map }
    }
}

impl<T> FacadeOrderedSet<T> {
    get_fn!(Ord);
    contains_fn!(Ord);
}

impl<T> Len for FacadeOrderedSet<T> {
    fn len(&self) -> usize {
        self.map.len()
    }
}

impl<T> Debug for FacadeOrderedSet<T>
where
    T: Debug,
{
    debug_fn!();
}

impl<T> Default for FacadeOrderedSet<T> {
    fn default() -> Self {
        Self {
            map: FacadeOrderedMap::default(),
        }
    }
}

impl<T> IntoIterator for FacadeOrderedSet<T> {
    into_iter_fn!();
}

impl<'a, T> IntoIterator for &'a FacadeOrderedSet<T> {
    into_iter_ref_fn!();
}

impl<T> SetIterator<T> for FacadeOrderedSet<T> {
    type Iterator<'a>
        = Iter<'a, T>
    where
        T: 'a;

    set_iterator_boilerplate!();
}

impl<T> Set<T> for FacadeOrderedSet<T>
where
    T: Ord,
{
    set_boilerplate!();
}

impl<T, ST> BitOr<&ST> for &FacadeOrderedSet<T>
where
    T: Hash + Eq + Ord + Clone,
    ST: Set<T>,
{
    bitor_fn!(RandomState);
}

impl<T, ST> BitAnd<&ST> for &FacadeOrderedSet<T>
where
    T: Hash + Eq + Ord + Clone,
    ST: Set<T>,
{
    bitand_fn!(RandomState);
}

impl<T, ST> BitXor<&ST> for &FacadeOrderedSet<T>
where
    T: Hash + Eq + Ord + Clone,
    ST: Set<T>,
{
    bitxor_fn!(RandomState);
}

impl<T, ST> Sub<&ST> for &FacadeOrderedSet<T>
where
    T: Hash + Eq + Ord + Clone,
    ST: Set<T>,
{
    sub_fn!(RandomState);
}

impl<T, ST> PartialEq<ST> for FacadeOrderedSet<T>
where
    T: Ord,
    ST: Set<T>,
{
    partial_eq_fn!();
}

impl<T> Eq for FacadeOrderedSet<T> where T: Ord {}
