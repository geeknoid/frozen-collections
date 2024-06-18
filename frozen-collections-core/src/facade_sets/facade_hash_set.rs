use crate::facade_maps::FacadeHashMap;
use crate::hashers::BridgeHasher;
use crate::sets::decl_macros::{
    bitand_fn, bitor_fn, bitxor_fn, debug_fn, into_iter_fn, into_iter_ref_fn, partial_eq_fn,
    set_boilerplate, set_iterator_boilerplate, sub_fn,
};
use crate::sets::{IntoIter, Iter};
use crate::traits::{Hasher, Len, MapIterator, Set, SetIterator};
use core::borrow::Borrow;
use core::fmt::Debug;
use core::hash::Hash;
use core::ops::{BitAnd, BitOr, BitXor, Sub};

/// A set optimized for fast read access with hashable values.
///
#[doc = include_str!("../doc_snippets/type_compat_warning.md")]
#[doc = include_str!("../doc_snippets/about.md")]
#[doc = include_str!("../doc_snippets/hash_warning.md")]
///
/// # Alternate Choices
///
/// If your values are integers or enum variants, you should use the [`FacadeScalarSet`](crate::facade_sets::FacadeScalarSet) type instead.
/// If your values are strings, you should use the [`FacadeStringSet`](crate::facade_sets::FacadeStringSet) type instead. Both of these will
/// deliver better performance since they are specifically optimized for those value types.
#[derive(Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct FacadeHashSet<T, H = BridgeHasher> {
    map: FacadeHashMap<T, (), H>,
}

impl<T, H> FacadeHashSet<T, H>
where
    T: Eq,
    H: Hasher<T>,
{
    /// Creates a new frozen set which uses the given hash builder to hash values.
    #[must_use]
    pub const fn new(map: FacadeHashMap<T, (), H>) -> Self {
        Self { map }
    }
}

impl<T, H> FacadeHashSet<T, H> {
    #[doc = include_str!("../doc_snippets/get_from_set_method.md")]
    #[inline]
    #[must_use]
    pub fn get<Q>(&self, value: &Q) -> Option<&T>
    where
        T: Borrow<Q>,
        H: Hasher<Q>,
        Q: ?Sized + Eq,
    {
        Some(self.map.get_key_value(value)?.0)
    }

    #[doc = include_str!("../doc_snippets/contains_method.md")]
    #[inline]
    #[must_use]
    pub fn contains<Q>(&self, value: &Q) -> bool
    where
        T: Borrow<Q>,
        H: Hasher<Q>,
        Q: ?Sized + Eq,
    {
        self.get(value).is_some()
    }
}

impl<T, H> Len for FacadeHashSet<T, H> {
    fn len(&self) -> usize {
        self.map.len()
    }
}

impl<T, H> Debug for FacadeHashSet<T, H>
where
    T: Debug,
{
    debug_fn!();
}

impl<T, H> Default for FacadeHashSet<T, H>
where
    H: Default,
{
    fn default() -> Self {
        Self {
            map: FacadeHashMap::default(),
        }
    }
}

impl<T, H> IntoIterator for FacadeHashSet<T, H> {
    into_iter_fn!();
}

impl<'a, T, H> IntoIterator for &'a FacadeHashSet<T, H> {
    into_iter_ref_fn!();
}

impl<T, H> SetIterator<T> for FacadeHashSet<T, H> {
    type Iterator<'a>
        = Iter<'a, T>
    where
        T: 'a,
        H: 'a;

    set_iterator_boilerplate!();
}

impl<T, H> Set<T> for FacadeHashSet<T, H>
where
    T: Eq,
    H: Hasher<T>,
{
    set_boilerplate!();
}

impl<T, ST, H> BitOr<&ST> for &FacadeHashSet<T, H>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
    H: Hasher<T> + Default,
{
    bitor_fn!(H);
}

impl<T, ST, H> BitAnd<&ST> for &FacadeHashSet<T, H>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
    H: Hasher<T> + Default,
{
    bitand_fn!(H);
}

impl<T, ST, H> BitXor<&ST> for &FacadeHashSet<T, H>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
    H: Hasher<T> + Default,
{
    bitxor_fn!(H);
}

impl<T, ST, H> Sub<&ST> for &FacadeHashSet<T, H>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
    H: Hasher<T> + Default,
{
    sub_fn!(H);
}

impl<T, ST, H> PartialEq<ST> for FacadeHashSet<T, H>
where
    T: Eq,
    ST: Set<T>,
    H: Hasher<T>,
{
    partial_eq_fn!();
}

impl<T, H> Eq for FacadeHashSet<T, H>
where
    T: Eq,
    H: Hasher<T>,
{
}
