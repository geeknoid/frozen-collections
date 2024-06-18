use std::borrow::Borrow;
use std::fmt::{Debug, Formatter, Result};
use std::hash::Hash;
use std::ops::{Index, IndexMut};

use num_traits::PrimInt;

use crate::analyzers::check_duplicate_keys;
use crate::specialized_maps::utils::{any_duplicate_keys, get_many_mut, partial_eq};
use crate::specialized_maps::{
    IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, Values, ValuesMut,
};
use crate::traits::{Len, Map};

/// A map whose keys are a continuous range of integers.
///
/// # Important Note
///
/// This type is not intended to be used directly by
/// application code. Instead, applications are expected
/// to use the `FrozenIntMap` type or the `frozen_map!` macro.
#[derive(Clone)]
pub struct IntegerRangeMap<K, V> {
    min: K,
    max: K,
    pub(crate) entries: Box<[(K, V)]>,
}

impl<K, V> IntegerRangeMap<K, V>
where
    K: PrimInt + Hash + Eq,
{
    #[allow(clippy::missing_errors_doc)]
    #[allow(clippy::missing_panics_doc)]
    pub fn new(mut payload: Vec<(K, V)>) -> std::result::Result<Self, &'static str> {
        if payload.is_empty() {
            return Ok(Self::default());
        }

        check_duplicate_keys(payload.iter().map(|entry| &entry.0))?;

        payload.sort_by_key(|x| x.0);

        let min = payload[0].0;
        let max = payload[payload.len() - 1].0;

        if max.sub(min).to_usize().unwrap() == payload.len() - 1 {
            Ok(Self {
                min,
                max,
                entries: payload.into_boxed_slice(),
            })
        } else {
            Err("IntegerRangeMap and IntegerRangeSet require that the map keys be in a continuous range")
        }
    }
}

impl<K, V> IntegerRangeMap<K, V> {
    #[inline]
    #[must_use]
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: PrimInt,
    {
        if *key >= *self.min.borrow() && *key <= *self.max.borrow() {
            let index = (*key - *self.min.borrow()).to_usize()?;
            Some(&self.entries[index].1)
        } else {
            None
        }
    }

    #[inline]
    #[must_use]
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: PrimInt,
    {
        if *key >= *self.min.borrow() && *key <= *self.max.borrow() {
            let index = (*key - *self.min.borrow()).to_usize()?;
            Some(&mut self.entries[index].1)
        } else {
            None
        }
    }

    #[must_use]
    pub fn get_many_mut<Q, const N: usize>(&mut self, keys: [&Q; N]) -> Option<[&mut V; N]>
    where
        K: Borrow<Q>,
        Q: PrimInt,
    {
        get_many_mut!(self, keys);
    }

    #[inline]
    #[must_use]
    pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q>,
        Q: PrimInt,
    {
        if *key >= *self.min.borrow() && *key <= *self.max.borrow() {
            let index = (*key - *self.min.borrow()).to_usize()?;
            Some((&self.entries[index].0, &self.entries[index].1))
        } else {
            None
        }
    }

    #[inline]
    #[must_use]
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: PrimInt,
    {
        self.get(key).is_some()
    }
}

impl<K, V> Len for IntegerRangeMap<K, V> {
    fn len(&self) -> usize {
        self.entries.len()
    }
}

impl<K, V> Debug for IntegerRangeMap<K, V>
where
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let pairs = self.entries.iter().map(|x| (&x.0, &x.1));
        f.debug_map().entries(pairs).finish()
    }
}

impl<K, V> Default for IntegerRangeMap<K, V>
where
    K: PrimInt,
{
    fn default() -> Self {
        Self {
            min: K::zero(),
            max: K::zero(),
            entries: Box::new([]),
        }
    }
}

impl<Q, K, V> Index<&Q> for IntegerRangeMap<K, V>
where
    K: Borrow<Q>,
    Q: PrimInt,
{
    type Output = V;

    fn index(&self, index: &Q) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<Q, K, V> IndexMut<&Q> for IntegerRangeMap<K, V>
where
    K: Borrow<Q>,
    Q: PrimInt,
{
    fn index_mut(&mut self, index: &Q) -> &mut V {
        self.get_mut(index).unwrap()
    }
}

impl<K, V> IntoIterator for IntegerRangeMap<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self.entries)
    }
}

impl<'a, K, V> IntoIterator for &'a IntegerRangeMap<K, V>
where
    K: PrimInt + Hash + Eq,
{
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, K, V> IntoIterator for &'a mut IntegerRangeMap<K, V>
where
    K: PrimInt + Hash + Eq,
{
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<K, V, MT> PartialEq<MT> for IntegerRangeMap<K, V>
where
    K: PrimInt + Hash + Eq,
    V: PartialEq,
    MT: Map<K, V>,
{
    partial_eq!();
}

impl<K, V> Eq for IntegerRangeMap<K, V>
where
    K: PrimInt + Hash + Eq,
    V: Eq,
{
}

impl<K, V> TryFrom<Vec<(K, V)>> for IntegerRangeMap<K, V>
where
    K: PrimInt + Hash + Eq,
{
    type Error = &'static str;

    fn try_from(payload: Vec<(K, V)>) -> std::result::Result<Self, Self::Error> {
        Self::new(payload)
    }
}

impl<K, V, const N: usize> TryFrom<[(K, V); N]> for IntegerRangeMap<K, V>
where
    K: PrimInt + Hash + Eq,
{
    type Error = &'static str;

    fn try_from(payload: [(K, V); N]) -> std::result::Result<Self, Self::Error> {
        Self::new(Vec::from_iter(payload))
    }
}

impl<K, V> FromIterator<(K, V)> for IntegerRangeMap<K, V>
where
    K: PrimInt + Hash + Eq,
{
    /// # Panics
    ///
    /// This panics if the keys don't represent a contiguous range of integer values.
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        Self::new(Vec::from_iter(iter)).unwrap()
    }
}

impl<K, V> Map<K, V> for IntegerRangeMap<K, V>
where
    K: PrimInt + Hash + Eq,
{
    type Iterator<'a> = Iter<'a, K, V>
    where
        K: 'a,
        V: 'a;

    type KeyIterator<'a> = Keys<'a, K, V>
    where
        K: 'a,
        V: 'a;

    type ValueIterator<'a> = Values<'a, K, V>
    where
        K: 'a,
        V: 'a;

    type IntoKeyIterator = IntoKeys<K, V>;
    type IntoValueIterator = IntoValues<K, V>;

    type MutIterator<'a> = IterMut<'a, K, V>
    where
        K: 'a,
        V: 'a;

    type ValueMutIterator<'a> = ValuesMut<'a, K, V>
    where
        K: 'a,
        V: 'a;

    #[inline]
    fn iter(&self) -> Self::Iterator<'_> {
        Iter::new(&self.entries)
    }

    #[inline]
    fn keys(&self) -> Self::KeyIterator<'_> {
        Keys::new(&self.entries)
    }

    #[inline]
    fn values(&self) -> Self::ValueIterator<'_> {
        Values::new(&self.entries)
    }

    #[inline]
    fn into_keys(self) -> Self::IntoKeyIterator {
        IntoKeys::new(self.entries)
    }

    #[inline]
    fn into_values(self) -> Self::IntoValueIterator {
        IntoValues::new(self.entries)
    }

    #[inline]
    fn iter_mut(&mut self) -> Self::MutIterator<'_> {
        IterMut::new(self.entries.as_mut())
    }

    #[inline]
    fn values_mut(&mut self) -> Self::ValueMutIterator<'_> {
        ValuesMut::new(self.entries.as_mut())
    }

    #[inline]
    fn contains_key(&self, key: &K) -> bool {
        self.contains_key(key)
    }

    #[inline]
    fn get(&self, key: &K) -> Option<&V> {
        Self::get(self, key)
    }
}

#[cfg(test)]
mod test {
    use crate::traits::Len;

    use super::IntegerRangeMap;

    #[test]
    fn range_map_test() {
        const MIN: [i32; 5] = [-11, -10, -9, 0, 1];

        for min in MIN {
            let mut v = Vec::new();
            for i in 0..10 {
                v.push((min + i, i));
            }

            let mut m = IntegerRangeMap::<i32, i32>::try_from(v).unwrap();

            assert_eq!(10, m.len());
            for i in 0..9 {
                let index = min + i;
                assert_eq!(i, *m.get(&index).unwrap());
                assert_eq!(i, *m.get_mut(&index).unwrap());

                let (k, v) = m.get_key_value(&index).unwrap();
                assert_eq!((index, i), (*k, *v));
            }

            let below = min - 1;
            assert_eq!(None, m.get(&below));
            assert_eq!(None, m.get_mut(&below));
            assert_eq!(None, m.get_key_value(&below));

            let above = min + 10;
            assert_eq!(None, m.get(&above));
            assert_eq!(None, m.get_mut(&above));
            assert_eq!(None, m.get_key_value(&above));

            if min == -11 {
                assert_eq!(
                    "{-11: 0, -10: 1, -9: 2, -8: 3, -7: 4, -6: 5, -5: 6, -4: 7, -3: 8, -2: 9}",
                    format!("{m:?}")
                );
            }
        }
    }
}
