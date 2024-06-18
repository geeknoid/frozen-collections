use alloc::boxed::Box;
use core::fmt::{Debug, Formatter};
use core::iter::FusedIterator;

/// An iterator over the entries of a map.
pub struct Iter<'a, K, V> {
    inner: core::slice::Iter<'a, (K, V)>,
}

impl<'a, K, V> Iter<'a, K, V> {
    pub(crate) fn new(entries: &'a [(K, V)]) -> Self {
        Self {
            inner: entries.iter(),
        }
    }
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|entry| (&entry.0, &entry.1))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.inner.count()
    }

    fn fold<B, F>(self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        self.inner.fold(init, |acc, (k, v)| f(acc, (k, v)))
    }
}

impl<K, V> ExactSizeIterator for Iter<'_, K, V> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<K, V> Clone for Iter<'_, K, V> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<K, V> Debug for Iter<'_, K, V>
where
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

/// An iterator over the entries of a map providing mutable values.
pub struct IterMut<'a, K, V> {
    inner: core::slice::IterMut<'a, (K, V)>,
}

impl<'a, K, V> IterMut<'a, K, V> {
    pub(crate) fn new(entries: &'a mut [(K, V)]) -> Self {
        Self {
            inner: entries.iter_mut(),
        }
    }
}

impl<'a, K, V> Iterator for IterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|entry| (&entry.0, &mut entry.1))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.inner.count()
    }

    fn fold<B, F>(self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        self.inner.fold(init, |acc, (k, v)| f(acc, (k, v)))
    }
}

impl<K, V> ExactSizeIterator for IterMut<'_, K, V> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<K, V> FusedIterator for IterMut<'_, K, V> {}

impl<K, V> Debug for IterMut<'_, K, V>
where
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.inner.fmt(f)
    }
}

/// An iterator over the keys of a map.
pub struct Keys<'a, K, V> {
    inner: Iter<'a, K, V>,
}

impl<'a, K, V> Keys<'a, K, V> {
    #[must_use]
    pub fn new(entries: &'a [(K, V)]) -> Self {
        Self {
            inner: Iter::new(entries),
        }
    }
}

impl<'a, K, V> Iterator for Keys<'a, K, V> {
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|x| x.0)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }

    fn count(self) -> usize {
        self.inner.count()
    }

    fn fold<B, F>(self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        self.inner.fold(init, |acc, (k, _)| f(acc, k))
    }
}

impl<K, V> ExactSizeIterator for Keys<'_, K, V> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<K, V> FusedIterator for Keys<'_, K, V> {}

impl<K, V> Clone for Keys<'_, K, V> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<K, V> Debug for Keys<'_, K, V>
where
    K: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

/// An iterator over the values of a map.
pub struct Values<'a, K, V> {
    inner: Iter<'a, K, V>,
}

impl<'a, K, V> Values<'a, K, V> {
    #[must_use]
    pub fn new(entries: &'a [(K, V)]) -> Self {
        Self {
            inner: Iter::new(entries),
        }
    }
}

impl<'a, K, V> Iterator for Values<'a, K, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|x| x.1)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }

    fn count(self) -> usize {
        self.inner.count()
    }

    fn fold<B, F>(self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        self.inner.fold(init, |acc, (_, v)| f(acc, v))
    }
}

impl<K, V> ExactSizeIterator for Values<'_, K, V> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<K, V> FusedIterator for Values<'_, K, V> {}

impl<K, V> Clone for Values<'_, K, V> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<K, V> Debug for Values<'_, K, V>
where
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

/// An iterator over the mutable values of a map.
pub struct ValuesMut<'a, K, V> {
    inner: IterMut<'a, K, V>,
}

impl<'a, K, V> ValuesMut<'a, K, V> {
    pub(crate) fn new(entries: &'a mut [(K, V)]) -> Self {
        Self {
            inner: IterMut::new(entries),
        }
    }
}

impl<'a, K, V> Iterator for ValuesMut<'a, K, V> {
    type Item = &'a mut V;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|entry| entry.1)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.inner.count()
    }

    fn fold<B, F>(self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        self.inner.fold(init, |acc, (_, v)| f(acc, v))
    }
}

impl<K, V> ExactSizeIterator for ValuesMut<'_, K, V> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<K, V> FusedIterator for ValuesMut<'_, K, V> {}

impl<K, V> Debug for ValuesMut<'_, K, V>
where
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.inner.fmt(f)
    }
}

/// A consuming iterator over the entries in a map.
pub struct IntoIter<K, V> {
    inner: alloc::vec::IntoIter<(K, V)>,
}

impl<K, V> IntoIter<K, V> {
    pub(crate) fn new(entries: Box<[(K, V)]>) -> Self {
        Self {
            inner: entries.into_vec().into_iter(),
        }
    }
}

impl<K, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.inner.count()
    }

    fn fold<B, F>(self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        self.inner.fold(init, |acc, (k, v)| f(acc, (k, v)))
    }
}

impl<K, V> ExactSizeIterator for IntoIter<K, V> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<K, V> FusedIterator for IntoIter<K, V> {}

impl<K, V> Debug for IntoIter<K, V>
where
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.inner.fmt(f)
    }
}

/// A consuming iterator over the keys in a map.
pub struct IntoKeys<K, V> {
    inner: IntoIter<K, V>,
}

impl<K, V> IntoKeys<K, V> {
    pub(crate) fn new(entries: Box<[(K, V)]>) -> Self {
        Self {
            inner: IntoIter::new(entries),
        }
    }
}

impl<K, V> Iterator for IntoKeys<K, V> {
    type Item = K;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|x| x.0)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }

    fn count(self) -> usize {
        self.inner.count()
    }

    fn fold<B, F>(self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        self.inner.fold(init, |acc, (k, _)| f(acc, k))
    }
}

impl<K, V> ExactSizeIterator for IntoKeys<K, V> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<K, V> FusedIterator for IntoKeys<K, V> {}

/// A consuming iterator over the values in a map.
pub struct IntoValues<K, V> {
    inner: IntoIter<K, V>,
}

impl<K, V> IntoValues<K, V> {
    pub(crate) fn new(entries: Box<[(K, V)]>) -> Self {
        Self {
            inner: IntoIter::new(entries),
        }
    }
}

impl<K, V> Iterator for IntoValues<K, V> {
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|x| x.1)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }

    fn count(self) -> usize {
        self.inner.count()
    }

    fn fold<B, F>(self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        self.inner.fold(init, |acc, (_, v)| f(acc, v))
    }
}

impl<K, V> ExactSizeIterator for IntoValues<K, V> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<K, V> FusedIterator for IntoValues<K, V> {}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec::Vec;
    use alloc::{format, vec};

    #[test]
    fn test_iter() {
        let entries = vec![("Alice", 1), ("Bob", 2), ("Sandy", 3), ("Tom", 4)];
        let iter = Iter::new(&entries);
        assert_eq!(entries.len(), iter.len());

        let collected: Vec<_> = iter.collect();
        assert_eq!(
            collected,
            vec![(&"Alice", &1), (&"Bob", &2), (&"Sandy", &3), (&"Tom", &4)]
        );
    }

    #[test]
    fn test_iter_count() {
        let entries = vec![("Alice", 1), ("Bob", 2), ("Sandy", 3), ("Tom", 4)];
        let iter = Iter::new(&entries);
        assert_eq!(entries.len(), iter.len());
        assert_eq!(entries.len(), iter.count());
    }

    #[test]
    fn test_iter_empty() {
        let entries: Vec<(&str, i32)> = vec![];
        let mut iter = Iter::new(&entries);
        assert_eq!(0, iter.len());
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_iter_debug() {
        let entries = vec![("Alice", 1), ("Bob", 2)];
        let iter = Iter::new(&entries);

        let debug_str = format!("{iter:?}");
        assert!(debug_str.contains("Alice"));
        assert!(debug_str.contains("Bob"));
    }

    #[test]
    fn test_iter_mut() {
        let mut entries = vec![("Alice", 1), ("Bob", 2), ("Sandy", 3), ("Tom", 4)];
        let iter_mut = IterMut::new(&mut entries);

        for (_, v) in iter_mut {
            *v += 1;
        }

        let expected = vec![("Alice", 2), ("Bob", 3), ("Sandy", 4), ("Tom", 5)];
        assert_eq!(entries, expected);
    }

    #[test]
    fn test_iter_mut_count() {
        let mut entries = vec![("Alice", 1), ("Bob", 2), ("Sandy", 3), ("Tom", 4)];
        let iter_mut = IterMut::new(&mut entries);
        assert_eq!(4, iter_mut.count());
    }

    #[test]
    fn test_iter_mut_empty() {
        let mut entries: Vec<(&str, i32)> = vec![];
        let mut iter_mut = IterMut::new(&mut entries);
        assert_eq!(0, iter_mut.len());
        assert!(iter_mut.next().is_none());
    }

    #[test]
    fn test_iter_size_hint() {
        let entries = vec![("Alice", 1), ("Bob", 2)];
        let iter = Iter::new(&entries);

        assert_eq!(iter.size_hint(), (2, Some(2)));
    }

    #[test]
    fn test_iter_mut_size_hint() {
        let mut entries = vec![("Alice", 1), ("Bob", 2)];
        let iter_mut = IterMut::new(&mut entries);

        assert_eq!(iter_mut.size_hint(), (2, Some(2)));
    }

    #[test]
    fn test_iter_clone() {
        let entries = vec![("Alice", 1), ("Bob", 2)];
        let iter = Iter::new(&entries);
        let iter_clone = iter.clone();

        let collected: Vec<_> = iter_clone.collect();
        assert_eq!(collected, vec![(&"Alice", &1), (&"Bob", &2)]);
    }

    #[test]
    fn test_iter_mut_debug() {
        let mut entries = vec![("Alice", 1), ("Bob", 2)];
        let iter_mut = IterMut::new(&mut entries);

        let debug_str = format!("{iter_mut:?}");
        assert!(debug_str.contains("Alice"));
        assert!(debug_str.contains("Bob"));
    }

    #[test]
    fn test_keys() {
        let entries = vec![("Alice", 1), ("Bob", 2)];
        let keys = Keys::new(&entries);
        assert_eq!(entries.len(), keys.len());

        let collected: Vec<_> = keys.collect();
        assert_eq!(collected, vec![&"Alice", &"Bob"]);
    }

    #[test]
    fn test_keys_count() {
        let entries = vec![("Alice", 1), ("Bob", 2)];
        let keys = Keys::new(&entries);
        assert_eq!(2, keys.count());
    }

    #[test]
    fn test_keys_empty() {
        let entries: Vec<(&str, i32)> = vec![];
        let mut keys = Keys::new(&entries);
        assert_eq!(0, keys.len());
        assert!(keys.next().is_none());
    }

    #[test]
    fn test_keys_size_hint() {
        let entries = vec![("Alice", 1), ("Bob", 2)];
        let keys = Keys::new(&entries);

        assert_eq!(keys.size_hint(), (2, Some(2)));
    }

    #[test]
    fn test_keys_clone() {
        let entries = vec![("Alice", 1), ("Bob", 2)];
        let keys = Keys::new(&entries);
        let keys_clone = keys.clone();

        let collected: Vec<_> = keys_clone.collect();
        assert_eq!(collected, vec![&"Alice", &"Bob"]);
    }

    #[test]
    fn test_keys_debug() {
        let entries = vec![("Alice", 1), ("Bob", 2)];
        let keys = Keys::new(&entries);

        let debug_str = format!("{keys:?}");
        assert!(debug_str.contains("Alice"));
        assert!(debug_str.contains("Bob"));
    }

    #[test]
    fn test_values() {
        let entries = vec![("Alice", 1), ("Bob", 2)];
        let values = Values::new(&entries);
        assert_eq!(entries.len(), values.len());

        let collected: Vec<_> = values.collect();
        assert_eq!(collected, vec![&1, &2]);
    }

    #[test]
    fn test_values_count() {
        let entries = vec![("Alice", 1), ("Bob", 2)];
        let values = Values::new(&entries);
        assert_eq!(2, values.count());
    }

    #[test]
    fn test_values_empty() {
        let entries: Vec<(&str, i32)> = vec![];
        let mut values = Values::new(&entries);
        assert_eq!(0, values.len());
        assert!(values.next().is_none());
    }

    #[test]
    fn test_values_size_hint() {
        let entries = vec![("Alice", 1), ("Bob", 2)];
        let values = Values::new(&entries);

        assert_eq!(values.size_hint(), (2, Some(2)));
    }

    #[test]
    fn test_values_clone() {
        let entries = vec![("Alice", 1), ("Bob", 2)];
        let values = Values::new(&entries);
        let values_clone = values.clone();

        let collected: Vec<_> = values_clone.collect();
        assert_eq!(collected, vec![&1, &2]);
    }

    #[test]
    fn test_values_debug() {
        let entries = vec![("Alice", 1), ("Bob", 2)];
        let values = Values::new(&entries);

        let debug_str = format!("{values:?}");
        assert!(debug_str.contains('1'));
        assert!(debug_str.contains('2'));
    }

    #[test]
    fn test_into_keys() {
        let entries = vec![("Alice", 1), ("Bob", 2)];
        let into_keys = IntoKeys::new(entries.into_boxed_slice());
        assert_eq!(2, into_keys.len());

        let collected: Vec<_> = into_keys.collect();
        assert_eq!(collected, vec!["Alice", "Bob"]);
    }

    #[test]
    fn test_into_keys_count() {
        let entries = vec![("Alice", 1), ("Bob", 2)];
        let into_keys = IntoKeys::new(entries.into_boxed_slice());
        assert_eq!(2, into_keys.count());
    }

    #[test]
    fn test_into_keys_empty() {
        let entries: Vec<(&str, i32)> = vec![];
        let mut into_keys = IntoKeys::new(entries.into_boxed_slice());
        assert_eq!(0, into_keys.len());
        assert!(into_keys.next().is_none());
    }

    #[test]
    fn test_into_keys_size_hint() {
        let entries = vec![("Alice", 1), ("Bob", 2)];
        let into_keys = IntoKeys::new(entries.into_boxed_slice());

        assert_eq!(into_keys.size_hint(), (2, Some(2)));
    }

    #[test]
    fn test_into_values() {
        let entries = vec![("Alice", 1), ("Bob", 2)];
        let into_values = IntoValues::new(entries.into_boxed_slice());
        assert_eq!(2, into_values.len());

        let collected: Vec<_> = into_values.collect();
        assert_eq!(collected, vec![1, 2]);
    }

    #[test]
    fn test_into_values_count() {
        let entries = vec![("Alice", 1), ("Bob", 2)];
        let into_values = IntoValues::new(entries.into_boxed_slice());
        assert_eq!(2, into_values.count());
    }

    #[test]
    fn test_into_values_empty() {
        let entries: Vec<(&str, i32)> = vec![];
        let mut into_values = IntoValues::new(entries.into_boxed_slice());
        assert_eq!(0, into_values.len());
        assert!(into_values.next().is_none());
    }

    #[test]
    fn test_into_values_size_hint() {
        let entries = vec![("Alice", 1), ("Bob", 2)];
        let into_values = IntoValues::new(entries.into_boxed_slice());
        assert_eq!(into_values.size_hint(), (2, Some(2)));
    }

    #[test]
    fn test_values_mut() {
        let mut entries = vec![("Alice", 1), ("Bob", 2)];
        let values_mut = ValuesMut::new(&mut entries);
        assert_eq!(2, values_mut.len());

        for v in values_mut {
            *v += 1;
        }

        let expected = vec![("Alice", 2), ("Bob", 3)];
        assert_eq!(entries, expected);
    }

    #[test]
    fn test_values_mut_count() {
        let mut entries = vec![("Alice", 1), ("Bob", 2)];
        let values_mut = ValuesMut::new(&mut entries);
        assert_eq!(2, values_mut.count());
    }

    #[test]
    fn test_values_mut_empty() {
        let mut entries: Vec<(&str, i32)> = vec![];
        let mut values_mut = ValuesMut::new(&mut entries);
        assert_eq!(0, values_mut.len());
        assert!(values_mut.next().is_none());
    }

    #[test]
    fn test_values_mut_size_hint() {
        let mut entries = vec![("Alice", 1), ("Bob", 2)];
        let values_mut = ValuesMut::new(&mut entries);
        assert_eq!(values_mut.size_hint(), (2, Some(2)));
    }

    #[test]
    fn test_values_mut_debug() {
        let mut entries = vec![("Alice", 1), ("Bob", 2)];
        let values_mut = ValuesMut::new(&mut entries);

        let debug_str = format!("{values_mut:?}");
        assert!(debug_str.contains("Alice"));
        assert!(debug_str.contains("Bob"));
    }

    #[test]
    fn test_into_iter() {
        let entries = vec![("Alice", 1), ("Bob", 2), ("Sandy", 3), ("Tom", 4)];
        let into_iter = IntoIter::new(entries.into_boxed_slice());
        assert_eq!(4, into_iter.len());

        let collected: Vec<_> = into_iter.collect();
        assert_eq!(
            collected,
            vec![("Alice", 1), ("Bob", 2), ("Sandy", 3), ("Tom", 4)]
        );
    }

    #[test]
    fn test_into_iter_count() {
        let entries = vec![("Alice", 1), ("Bob", 2), ("Sandy", 3), ("Tom", 4)];
        let into_iter = IntoIter::new(entries.into_boxed_slice());
        assert_eq!(4, into_iter.count());
    }

    #[test]
    fn test_into_iter_empty() {
        let entries: Vec<(&str, i32)> = vec![];
        let mut into_iter = IntoIter::new(entries.into_boxed_slice());
        assert_eq!(0, into_iter.len());
        assert!(into_iter.next().is_none());
    }

    #[test]
    fn test_into_iter_size_hint() {
        let entries = vec![("Alice", 1), ("Bob", 2)];
        let into_iter = IntoIter::new(entries.into_boxed_slice());

        assert_eq!(into_iter.size_hint(), (2, Some(2)));
    }

    #[test]
    fn test_into_iter_debug() {
        let entries = vec![("Alice", 1), ("Bob", 2)];
        let into_iter = IntoIter::new(entries.into_boxed_slice());

        let debug_str = format!("{into_iter:?}");
        assert!(debug_str.contains("Alice"));
        assert!(debug_str.contains("Bob"));
    }
}
