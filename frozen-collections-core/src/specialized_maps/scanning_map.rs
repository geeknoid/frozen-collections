use std::borrow::Borrow;
use std::fmt::{Debug, Formatter, Result};
use std::hash::Hash;
use std::intrinsics::transmute;
use std::mem::MaybeUninit;
use std::ops::{Index, IndexMut};

use crate::analyzers::check_duplicate_keys;
use crate::specialized_maps::utils::any_duplicate_keys;
use crate::specialized_maps::{
    IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, Values, ValuesMut,
};
use crate::traits::Len;

/// A general purpose map that uses linear scan of entries rather than a hash table.
///
/// # Important Note
///
/// This type is not intended to be used directly by
/// application code. Instead, applications are expected
/// to use the `FrozenMap` type or the `frozen_map!` macro.
#[derive(Clone)]
pub struct ScanningMap<K, V> {
    pub(crate) entries: Box<[(K, V)]>,
}

impl<K, V> ScanningMap<K, V> {
    #[inline]
    #[must_use]
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: ?Sized + Eq,
    {
        for entry in self.entries.iter() {
            if key.eq(entry.0.borrow()) {
                return Some(&entry.1);
            }
        }

        None
    }

    #[inline]
    #[must_use]
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: ?Sized + Eq,
    {
        for entry in self.entries.iter_mut() {
            if key.eq(entry.0.borrow()) {
                return Some(&mut entry.1);
            }
        }

        None
    }

    #[inline]
    #[must_use]
    pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q>,
        Q: ?Sized + Eq,
    {
        for entry in self.entries.iter() {
            if key.eq(entry.0.borrow()) {
                return Some((&entry.0, &entry.1));
            }
        }

        None
    }

    #[allow(mutable_transmutes)]
    pub fn get_many_mut<Q, const N: usize>(&mut self, keys: [&Q; N]) -> Option<[&mut V; N]>
    where
        K: Borrow<Q>,
        Q: ?Sized + Eq,
    {
        if any_duplicate_keys(keys) {
            return None;
        }

        unsafe {
            let mut result: MaybeUninit<[&mut V; N]> = MaybeUninit::uninit();
            let p = result.as_mut_ptr();

            for (i, key) in keys.iter().enumerate() {
                *(*p).get_unchecked_mut(i) = transmute(self.get(key)?);
            }

            Some(result.assume_init())
        }
    }

    #[inline]
    #[must_use]
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: ?Sized + Eq,
    {
        self.get(key).is_some()
    }

    #[must_use]
    pub const fn iter(&self) -> Iter<K, V> {
        Iter::new(&self.entries)
    }

    #[must_use]
    pub const fn keys(&self) -> Keys<K, V> {
        Keys::new(&self.entries)
    }

    #[must_use]
    pub const fn values(&self) -> Values<K, V> {
        Values::new(&self.entries)
    }

    #[must_use]
    pub fn into_keys(self) -> IntoKeys<K, V> {
        IntoKeys::new(self.entries)
    }

    #[must_use]
    pub fn into_values(self) -> IntoValues<K, V> {
        IntoValues::new(self.entries)
    }

    #[must_use]
    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        IterMut::new(self.entries.as_mut())
    }

    #[must_use]
    pub fn values_mut(&mut self) -> ValuesMut<'_, K, V> {
        ValuesMut::new(self.entries.as_mut())
    }
}

impl<K, V> Len for ScanningMap<K, V> {
    fn len(&self) -> usize {
        self.entries.len()
    }
}

impl<K, V> Debug for ScanningMap<K, V>
where
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let pairs = self.entries.iter().map(|x| (&x.0, &x.1));
        f.debug_map().entries(pairs).finish()
    }
}

impl<K, V> Default for ScanningMap<K, V> {
    fn default() -> Self {
        Self {
            entries: Box::default(),
        }
    }
}

impl<Q, K, V> Index<&Q> for ScanningMap<K, V>
where
    K: Borrow<Q>,
    Q: ?Sized + Eq,
{
    type Output = V;

    fn index(&self, index: &Q) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<Q, K, V> IndexMut<&Q> for ScanningMap<K, V>
where
    K: Borrow<Q>,
    Q: ?Sized + Eq,
{
    fn index_mut(&mut self, index: &Q) -> &mut V {
        self.get_mut(index).unwrap()
    }
}

impl<'a, K, V> IntoIterator for &'a ScanningMap<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, K, V> IntoIterator for &'a mut ScanningMap<K, V> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<K, V> PartialEq<Self> for ScanningMap<K, V>
where
    K: Eq,
    V: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self.iter()
            .all(|(key, value)| other.get(key).map_or(false, |v| *value == *v))
    }
}

impl<K, V> Eq for ScanningMap<K, V>
where
    K: Eq,
    V: Eq,
{
}

impl<K, V> TryFrom<Vec<(K, V)>> for ScanningMap<K, V>
where
    K: Hash + Eq,
{
    type Error = &'static str;

    fn try_from(payload: Vec<(K, V)>) -> std::result::Result<Self, Self::Error> {
        check_duplicate_keys(payload.iter().map(|entry| &entry.0))?;

        Ok(Self {
            entries: payload.into_boxed_slice(),
        })
    }
}

impl<K, V, const N: usize> TryFrom<[(K, V); N]> for ScanningMap<K, V>
where
    K: Hash + Eq,
{
    type Error = &'static str;

    fn try_from(payload: [(K, V); N]) -> std::result::Result<Self, Self::Error> {
        Self::try_from(Vec::from_iter(payload))
    }
}

impl<K, V> FromIterator<(K, V)> for ScanningMap<K, V>
where
    K: Hash + Eq,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        Self::try_from(Vec::from_iter(iter)).unwrap()
    }
}

impl<K, V> IntoIterator for ScanningMap<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self.entries)
    }
}

#[cfg(test)]
mod tests {
    use crate::traits::Len;

    use super::ScanningMap;

    #[test]
    fn new_creates_scanning_map_with_given_payload() {
        let payload = vec![(10, 20), (30, 40), (50, 60)];
        let map = ScanningMap::<i32, i32>::try_from(payload.clone()).unwrap();
        assert_eq!(payload.len(), map.len());
    }

    #[test]
    fn get_returns_some_for_existing_keys() {
        let payload = vec![(10, 20), (30, 40), (50, 60)];
        let map = ScanningMap::<i32, i32>::try_from(payload).unwrap();
        assert_eq!(&20, map.get(&10).unwrap());
        assert_eq!(&40, map.get(&30).unwrap());
        assert_eq!(&60, map.get(&50).unwrap());
    }

    #[test]
    fn get_returns_none_for_non_existing_keys() {
        let payload = vec![(10, 20), (30, 40), (50, 60)];
        let map = ScanningMap::<i32, i32>::try_from(payload).unwrap();
        assert_eq!(None, map.get(&0));
    }

    #[test]
    fn get_mut_returns_some_for_existing_keys() {
        let payload = vec![(10, 20), (30, 40), (50, 60)];
        let mut map = ScanningMap::<i32, i32>::try_from(payload).unwrap();
        assert_eq!(&20, map.get_mut(&10).unwrap());
        assert_eq!(&40, map.get_mut(&30).unwrap());
        assert_eq!(&60, map.get_mut(&50).unwrap());
    }

    #[test]
    fn get_mut_returns_none_for_non_existing_keys() {
        let payload = vec![(10, 20), (30, 40), (50, 60)];
        let mut map = ScanningMap::<i32, i32>::try_from(payload).unwrap();
        assert_eq!(None, map.get_mut(&0));
    }

    #[test]
    fn get_key_value_returns_some_for_existing_keys() {
        let payload = vec![(10, 20), (30, 40), (50, 60)];
        let map = ScanningMap::<i32, i32>::try_from(payload).unwrap();
        assert_eq!((&10, &20), map.get_key_value(&10).unwrap());
        assert_eq!((&30, &40), map.get_key_value(&30).unwrap());
        assert_eq!((&50, &60), map.get_key_value(&50).unwrap());
    }

    #[test]
    fn get_key_value_returns_none_for_non_existing_keys() {
        let payload = vec![(10, 20), (30, 40), (50, 60)];
        let map = ScanningMap::<i32, i32>::try_from(payload).unwrap();
        assert_eq!(None, map.get_key_value(&0));
    }

    #[test]
    fn debug_format_is_correct() {
        let payload = vec![(10, 20)];
        let map = ScanningMap::<i32, i32>::try_from(payload).unwrap();
        assert_eq!("{10: 20}", format!("{map:?}"));
    }
}
