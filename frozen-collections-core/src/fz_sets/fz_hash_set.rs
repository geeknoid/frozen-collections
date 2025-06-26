use crate::DefaultBuildHasher;
use crate::fz_maps::FzHashMap;
use crate::maps::decl_macros::len_trait_funcs;
use crate::sets::decl_macros::{
    bitand_trait_funcs, bitor_trait_funcs, bitxor_trait_funcs, debug_trait_funcs, into_iterator_ref_trait_funcs, into_iterator_trait_funcs,
    partial_eq_trait_funcs, set_extras_trait_funcs, set_iteration_trait_funcs, set_query_trait_funcs, sub_trait_funcs,
};
use crate::sets::{IntoIter, Iter};
use crate::traits::{Len, Set, SetExtras, SetIteration, SetOps, SetQuery};
use core::fmt::Debug;
use core::hash::BuildHasher;
use core::hash::Hash;
use core::ops::{BitAnd, BitOr, BitXor, Sub};
use equivalent::Equivalent;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

#[cfg(feature = "serde")]
use {
    crate::sets::decl_macros::serialize_trait_funcs,
    core::fmt::Formatter,
    core::marker::PhantomData,
    serde::de::{SeqAccess, Visitor},
    serde::ser::SerializeSeq,
    serde::{Deserialize, Deserializer, Serialize, Serializer},
};

/// A set optimized for fast read access with hashable values.
///
#[doc = include_str!("../doc_snippets/about.md")]
#[doc = include_str!("../doc_snippets/hash_warning.md")]
///
/// # Alternate Choices
///
/// If your values are integers or enum variants, you should use the [`FzScalarSet`](crate::fz_sets::FzScalarSet) type instead.
/// If your values are strings, you should use the [`FzStringSet`](crate::fz_sets::FzStringSet) type instead. Both of these will
/// deliver better performance since they are specifically optimized for those value types.
///
/// If your values are known at compile time, consider using the various `fz_*_set` macros instead of
/// this type as they generally perform better.
#[derive(Clone)]
pub struct FzHashSet<T, BH = DefaultBuildHasher> {
    map: FzHashMap<T, (), BH>,
}

impl<T> FzHashSet<T, DefaultBuildHasher> {
    /// Creates a new frozen set.
    #[must_use]
    pub fn new(entries: Vec<T>) -> Self
    where
        T: Hash + Eq,
    {
        Self::with_hasher(entries, DefaultBuildHasher::default())
    }
}

impl<T, BH> FzHashSet<T, BH>
where
    BH: BuildHasher,
{
    /// Creates a new frozen set which uses the given hash builder to hash values.
    #[must_use]
    pub fn with_hasher(entries: Vec<T>, bh: BH) -> Self
    where
        T: Hash + Eq,
    {
        Self {
            map: FzHashMap::with_hasher(entries.into_iter().map(|x| (x, ())).collect(), bh),
        }
    }

    #[doc = include_str!("../doc_snippets/get_from_set.md")]
    #[inline]
    fn get<Q>(&self, value: &Q) -> Option<&T>
    where
        Q: ?Sized + Hash + Equivalent<T>,
    {
        Some(self.map.get_key_value(value)?.0)
    }

    #[doc = include_str!("../doc_snippets/contains.md")]
    #[inline]
    #[must_use]
    pub fn contains<Q>(&self, value: &Q) -> bool
    where
        Q: ?Sized + Hash + Equivalent<T>,
    {
        self.map.contains_key(value)
    }

    #[doc = include_str!("../doc_snippets/len.md")]
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.map.len()
    }

    #[doc = include_str!("../doc_snippets/is_empty.md")]
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    #[doc = include_str!("../doc_snippets/iter.md")]
    #[must_use]
    pub fn iter(&self) -> Iter<'_, T> {
        Iter::new(self.map.iter())
    }

    #[must_use]
    fn into_iter(self) -> IntoIter<T> {
        IntoIter::new(self.map.into_iter())
    }
}

impl<T, BH> Default for FzHashSet<T, BH>
where
    BH: Default,
{
    fn default() -> Self {
        Self { map: FzHashMap::default() }
    }
}

impl<T, BH> From<FzHashMap<T, (), BH>> for FzHashSet<T, BH>
where
    T: Hash + Eq,
    BH: BuildHasher + Default,
{
    fn from(map: FzHashMap<T, (), BH>) -> Self {
        Self { map }
    }
}

impl<T, const N: usize, BH> From<[T; N]> for FzHashSet<T, BH>
where
    T: Hash + Eq,
    BH: BuildHasher + Default,
{
    fn from(entries: [T; N]) -> Self {
        Self::from(FzHashMap::from_iter(entries.into_iter().map(|x| (x, ()))))
    }
}

impl<T, BH> FromIterator<T> for FzHashSet<T, BH>
where
    T: Hash + Eq,
    BH: BuildHasher + Default,
{
    fn from_iter<IT: IntoIterator<Item = T>>(iter: IT) -> Self {
        Self::from(FzHashMap::from_iter(iter.into_iter().map(|x| (x, ()))))
    }
}

impl<T, Q, BH> Set<T, Q> for FzHashSet<T, BH>
where
    Q: ?Sized + Hash + Equivalent<T>,
    BH: BuildHasher,
{
}

impl<T, Q, BH> SetExtras<T, Q> for FzHashSet<T, BH>
where
    Q: ?Sized + Hash + Equivalent<T>,
    BH: BuildHasher,
{
    set_extras_trait_funcs!();
}

impl<T, Q, BH> SetQuery<Q> for FzHashSet<T, BH>
where
    Q: ?Sized + Hash + Equivalent<T>,
    BH: BuildHasher,
{
    set_query_trait_funcs!();
}

impl<T, BH> SetIteration<T> for FzHashSet<T, BH>
where
    BH: BuildHasher,
{
    type Iterator<'a>
        = Iter<'a, T>
    where
        T: 'a,
        BH: 'a;

    set_iteration_trait_funcs!();
}

impl<T, BH> Len for FzHashSet<T, BH>
where
    BH: BuildHasher,
{
    len_trait_funcs!();
}

impl<T, ST, BH> BitOr<&ST> for &FzHashSet<T, BH>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
    BH: BuildHasher,
{
    bitor_trait_funcs!();
}

impl<T, ST, BH> BitAnd<&ST> for &FzHashSet<T, BH>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
    BH: BuildHasher,
{
    bitand_trait_funcs!();
}

impl<T, ST, BH> BitXor<&ST> for &FzHashSet<T, BH>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
    BH: BuildHasher,
{
    bitxor_trait_funcs!();
}

impl<T, ST, BH> Sub<&ST> for &FzHashSet<T, BH>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
    BH: BuildHasher,
{
    sub_trait_funcs!();
}

impl<T, BH> IntoIterator for FzHashSet<T, BH>
where
    BH: BuildHasher,
{
    into_iterator_trait_funcs!();
}

impl<'a, T, BH> IntoIterator for &'a FzHashSet<T, BH>
where
    BH: BuildHasher,
{
    into_iterator_ref_trait_funcs!();
}

impl<T, ST, BH> PartialEq<ST> for FzHashSet<T, BH>
where
    T: PartialEq + Hash,
    ST: SetQuery<T>,
    BH: BuildHasher,
{
    partial_eq_trait_funcs!();
}

impl<T, BH> Eq for FzHashSet<T, BH>
where
    T: Hash + Eq,
    BH: BuildHasher,
{
}

impl<T, BH> Debug for FzHashSet<T, BH>
where
    T: Debug,
    BH: BuildHasher,
{
    debug_trait_funcs!();
}

#[cfg(feature = "serde")]
impl<T, BH> Serialize for FzHashSet<T, BH>
where
    T: Serialize,
    BH: BuildHasher,
{
    serialize_trait_funcs!();
}

#[cfg(feature = "serde")]
impl<'de, T, BH> Deserialize<'de> for FzHashSet<T, BH>
where
    T: Deserialize<'de> + Hash + Eq,
    BH: BuildHasher + Default,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(SetVisitor { marker: PhantomData })
    }
}

#[cfg(feature = "serde")]
struct SetVisitor<T, BH> {
    marker: PhantomData<(T, BH)>,
}

#[cfg(feature = "serde")]
impl<'de, T, BH> Visitor<'de> for SetVisitor<T, BH>
where
    T: Deserialize<'de> + Hash + Eq,
    BH: BuildHasher + Default,
{
    type Value = FzHashSet<T, BH>;

    fn expecting(&self, formatter: &mut Formatter) -> core::fmt::Result {
        formatter.write_str("a set with hashable values")
    }

    fn visit_seq<M>(self, mut seq: M) -> Result<Self::Value, M::Error>
    where
        M: SeqAccess<'de>,
    {
        let mut v = Vec::with_capacity(seq.size_hint().unwrap_or(0));
        while let Some(x) = seq.next_element()? {
            v.push((x, ()));
        }

        Ok(FzHashSet::from(FzHashMap::with_hasher(v, BH::default())))
    }
}
