use crate::facade_maps::FacadeStringMap;
use crate::sets::decl_macros::{debug_fn, partial_eq_fn};
use crate::sets::{IntoIter, Iter};
use crate::traits::{Len, MapIterator, Set, SetIterator};
use ahash::RandomState;
use core::fmt::Debug;
use core::hash::BuildHasher;
use core::ops::{BitAnd, BitOr, BitXor, Sub};

/// A set optimized for fast read access with string values.
///
#[doc = include_str!("../doc_snippets/type_compat_warning.md")]
#[doc = include_str!("../doc_snippets/about.md")]
#[derive(Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct FacadeStringSet<BH = RandomState> {
    map: FacadeStringMap<(), BH>,
}

impl<BH> FacadeStringSet<BH>
where
    BH: BuildHasher,
{
    /// Creates a new frozen set which will use the given hasher to hash values.
    #[must_use]
    pub const fn new(map: FacadeStringMap<(), BH>) -> Self {
        Self { map }
    }

    #[doc = include_str!("../doc_snippets/contains_method.md")]
    #[inline(always)]
    #[must_use]
    pub fn contains(&self, value: &str) -> bool {
        self.get(value).is_some()
    }

    #[doc = include_str!("../doc_snippets/get_from_set_method.md")]
    #[inline]
    #[must_use]
    pub fn get(&self, value: &str) -> Option<&String> {
        Some(self.map.get_key_value(value)?.0)
    }
}

impl<BH> Len for FacadeStringSet<BH> {
    fn len(&self) -> usize {
        self.map.len()
    }
}

impl<BH> Debug for FacadeStringSet<BH> {
    debug_fn!();
}

impl<BH> Default for FacadeStringSet<BH>
where
    BH: Default,
{
    fn default() -> Self {
        Self {
            map: FacadeStringMap::default(),
        }
    }
}

impl<BH> IntoIterator for FacadeStringSet<BH> {
    type Item = String;
    type IntoIter = IntoIter<String>;

    fn into_iter(self) -> Self::IntoIter {
        let it = self.map.into_iter();
        IntoIter::new(it)
    }
}

impl<'a, BH> IntoIterator for &'a FacadeStringSet<BH> {
    type Item = &'a String;
    type IntoIter = Iter<'a, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<BH> SetIterator<String> for FacadeStringSet<BH> {
    type Iterator<'a>
        = Iter<'a, String>
    where
        String: 'a,
        BH: 'a;

    fn iter(&self) -> Iter<'_, String> {
        Iter::new(self.map.iter())
    }
}

impl<BH> Set<String> for FacadeStringSet<BH>
where
    BH: BuildHasher,
{
    fn contains(&self, value: &String) -> bool {
        self.contains(value)
    }
}

impl<ST, BH> BitOr<&ST> for &FacadeStringSet<BH>
where
    ST: Set<String>,
    BH: BuildHasher + Default,
{
    type Output = hashbrown::HashSet<String>;

    fn bitor(self, rhs: &ST) -> Self::Output {
        Self::Output::from_iter(self.union(rhs).cloned())
    }
}

impl<ST, BH> BitAnd<&ST> for &FacadeStringSet<BH>
where
    ST: Set<String>,
    BH: BuildHasher + Default,
{
    type Output = hashbrown::HashSet<String>;

    fn bitand(self, rhs: &ST) -> Self::Output {
        Self::Output::from_iter(self.intersection(rhs).cloned())
    }
}

impl<ST, BH> BitXor<&ST> for &FacadeStringSet<BH>
where
    ST: Set<String>,
    BH: BuildHasher + Default,
{
    type Output = hashbrown::HashSet<String>;

    fn bitxor(self, rhs: &ST) -> Self::Output {
        self.symmetric_difference(rhs).cloned().collect()
    }
}

impl<ST, BH> Sub<&ST> for &FacadeStringSet<BH>
where
    ST: Set<String>,
    BH: BuildHasher + Default,
{
    type Output = hashbrown::HashSet<String>;

    fn sub(self, rhs: &ST) -> Self::Output {
        self.difference(rhs).cloned().collect()
    }
}

impl<ST, BH> PartialEq<ST> for FacadeStringSet<BH>
where
    ST: Set<String>,
    BH: BuildHasher,
{
    partial_eq_fn!();
}

impl<BH> Eq for FacadeStringSet<BH> where BH: BuildHasher {}
