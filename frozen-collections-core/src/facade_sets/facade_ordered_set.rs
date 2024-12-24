use crate::facade_maps::FacadeOrderedMap;
use crate::sets::decl_macros::{
    bitand_fn, bitor_fn, bitxor_fn, debug_fn, get_fn, into_iter_fn, into_iter_ref_fn,
    partial_eq_fn, set_iteration_funcs, sub_fn,
};
use crate::sets::{IntoIter, Iter};
use crate::traits::{Len, MapIteration, MapQuery, Set, SetIteration, SetOps, SetQuery};
use core::fmt::Debug;
use core::hash::Hash;
use core::ops::{BitAnd, BitOr, BitXor, Sub};
use equivalent::Comparable;

#[cfg(feature = "serde")]
use {
    crate::sets::decl_macros::serialize_fn,
    core::fmt::Formatter,
    core::marker::PhantomData,
    serde::de::{SeqAccess, Visitor},
    serde::ser::SerializeSeq,
    serde::{Deserialize, Deserializer, Serialize, Serializer},
};

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

impl<T> Default for FacadeOrderedSet<T> {
    fn default() -> Self {
        Self {
            map: FacadeOrderedMap::default(),
        }
    }
}

impl<T, Q> Set<T, Q> for FacadeOrderedSet<T> where Q: ?Sized + Ord + Comparable<T> {}

impl<T, Q> SetQuery<T, Q> for FacadeOrderedSet<T>
where
    Q: ?Sized + Ord + Comparable<T>,
{
    get_fn!();
}

impl<T> SetIteration<T> for FacadeOrderedSet<T> {
    type Iterator<'a>
        = Iter<'a, T>
    where
        T: 'a;

    set_iteration_funcs!();
}

impl<T> Len for FacadeOrderedSet<T> {
    fn len(&self) -> usize {
        self.map.len()
    }
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

impl<T> IntoIterator for FacadeOrderedSet<T> {
    into_iter_fn!();
}

impl<'a, T> IntoIterator for &'a FacadeOrderedSet<T> {
    into_iter_ref_fn!();
}

impl<T, ST> PartialEq<ST> for FacadeOrderedSet<T>
where
    T: Ord,
    ST: Set<T>,
{
    partial_eq_fn!();
}

impl<T> Eq for FacadeOrderedSet<T> where T: Ord {}

impl<T> Debug for FacadeOrderedSet<T>
where
    T: Debug,
{
    debug_fn!();
}

#[cfg(feature = "serde")]
impl<T> Serialize for FacadeOrderedSet<T>
where
    T: Serialize,
{
    serialize_fn!();
}

#[cfg(feature = "serde")]
impl<'de, T> Deserialize<'de> for FacadeOrderedSet<T>
where
    T: Deserialize<'de> + Ord,
{
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(SetVisitor {
            marker: PhantomData,
        })
    }
}

#[cfg(feature = "serde")]
struct SetVisitor<T> {
    marker: PhantomData<T>,
}

#[cfg(feature = "serde")]
impl<'de, T> Visitor<'de> for SetVisitor<T>
where
    T: Deserialize<'de> + Ord,
{
    type Value = FacadeOrderedSet<T>;

    fn expecting(&self, formatter: &mut Formatter) -> core::fmt::Result {
        formatter.write_str("A set with ordered values")
    }

    fn visit_seq<M>(self, mut access: M) -> core::result::Result<Self::Value, M::Error>
    where
        M: SeqAccess<'de>,
    {
        let mut v = Vec::with_capacity(access.size_hint().unwrap_or(0));
        while let Some(x) = access.next_element()? {
            v.push((x, ()));
        }

        Ok(FacadeOrderedSet::new(FacadeOrderedMap::new(v)))
    }
}
