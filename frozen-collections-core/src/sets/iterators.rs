use crate::traits::{Set, SetIteration, SetOps};
use core::cmp::{max, min};
use core::fmt::{Debug, Formatter};
use core::iter::{Chain, FusedIterator};

/// An iterator over the values of a set.
pub struct Iter<'a, T> {
    inner: crate::maps::Iter<'a, T, ()>,
}

impl<'a, T> Iter<'a, T> {
    pub(crate) const fn new(inner: crate::maps::Iter<'a, T, ()>) -> Self {
        Self { inner }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|entry| entry.0)
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
        self.inner.fold(init, |acc, (k, ())| f(acc, k))
    }
}

impl<T> ExactSizeIterator for Iter<'_, T> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<T> FusedIterator for Iter<'_, T> {}

impl<T> Clone for Iter<'_, T> {
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }
}

impl<T> Debug for Iter<'_, T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_list().entries((*self).clone()).finish()
    }
}

/// A consuming iterator over the values of a set.
#[derive(Debug)]
pub struct IntoIter<T> {
    inner: crate::maps::IntoIter<T, ()>,
}

impl<T> IntoIter<T> {
    pub(crate) const fn new(inner: crate::maps::IntoIter<T, ()>) -> Self {
        Self { inner }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|entry| entry.0)
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
        self.inner.fold(init, |acc, (k, ())| f(acc, k))
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<T> FusedIterator for IntoIter<T> {}

/// An iterator that returns the union between two sets.
pub struct Union<'a, S1, S2, T>
where
    S1: Set<T>,
    S2: Set<T>,
    T: 'a,
{
    s1: &'a S1,
    s1_iter: <S1 as SetIteration<T>>::Iterator<'a>,
    s2: &'a S2,
    s2_iter: <S2 as SetIteration<T>>::Iterator<'a>,
}

impl<'a, S1, S2, T> Union<'a, S1, S2, T>
where
    S1: Set<T>,
    S2: Set<T>,
{
    pub(crate) fn new(s1: &'a S1, s2: &'a S2) -> Self {
        Self {
            s1_iter: s1.iter(),
            s1,
            s2_iter: s2.iter(),
            s2,
        }
    }
}

impl<'a, S1, S2, T> Iterator for Union<'a, S1, S2, T>
where
    S1: Set<T>,
    S2: Set<T>,
{
    type Item = &'a T;

    #[allow(clippy::needless_borrow)]
    #[mutants::skip]
    fn next(&mut self) -> Option<Self::Item> {
        if self.s1.len() > self.s2.len() {
            let item = self.s1_iter.next();
            if item.is_some() {
                return item;
            }

            loop {
                let item = self.s2_iter.next()?;
                if !self.s1.contains(&item) {
                    return Some(item);
                }
            }
        } else {
            let item = self.s2_iter.next();
            if item.is_some() {
                return item;
            }

            loop {
                let item = self.s1_iter.next()?;
                if !self.s2.contains(&item) {
                    return Some(item);
                }
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let h1 = self.s1_iter.size_hint();
        let h2 = self.s2_iter.size_hint();

        let mut max_bound = None;
        if let Some(h1x) = h1.1 {
            if let Some(h2x) = h2.1 {
                max_bound = h1x.checked_add(h2x);
            }
        }

        (max(h1.0, h2.0), max_bound)
    }
}

impl<'a, S1, S2, T> Clone for Union<'a, S1, S2, T>
where
    S1: Set<T>,
    S2: Set<T>,
    <S1 as SetIteration<T>>::Iterator<'a>: Clone,
    <S2 as SetIteration<T>>::Iterator<'a>: Clone,
{
    fn clone(&self) -> Self {
        Self {
            s1: self.s1,
            s1_iter: self.s1_iter.clone(),
            s2: self.s2,
            s2_iter: self.s2_iter.clone(),
        }
    }
}

impl<S1, S2, T> FusedIterator for Union<'_, S1, S2, T>
where
    S1: Set<T>,
    S2: Set<T>,
{
}

impl<'a, S1, S2, T> Debug for Union<'a, S1, S2, T>
where
    S1: Set<T>,
    S2: Set<T>,
    <S1 as SetIteration<T>>::Iterator<'a>: Clone,
    <S2 as SetIteration<T>>::Iterator<'a>: Clone,
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_list().entries((*self).clone()).finish()
    }
}

/// An iterator that returns the symmetric difference between two sets.
pub struct SymmetricDifference<'a, S1, S2, T>
where
    S1: Set<T>,
    S2: Set<T>,
    T: 'a,
{
    iter: Chain<Difference<'a, S1, S2, T>, Difference<'a, S2, S1, T>>,
}

impl<'a, S1, S2, T> SymmetricDifference<'a, S1, S2, T>
where
    S1: Set<T>,
    S2: Set<T>,
{
    pub(crate) fn new(s1: &'a S1, s2: &'a S2) -> Self {
        Self {
            iter: s1.difference(s2).chain(s2.difference(s1)),
        }
    }
}

impl<'a, S1, S2, T> Iterator for SymmetricDifference<'a, S1, S2, T>
where
    S1: Set<T>,
    S2: Set<T>,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.iter.count()
    }
}

impl<'a, S1, S2, T> Clone for SymmetricDifference<'a, S1, S2, T>
where
    S1: Set<T>,
    S2: Set<T>,
    <S1 as SetIteration<T>>::Iterator<'a>: Clone,
    <S2 as SetIteration<T>>::Iterator<'a>: Clone,
{
    fn clone(&self) -> Self {
        Self { iter: self.iter.clone() }
    }
}

impl<S1, S2, T> FusedIterator for SymmetricDifference<'_, S1, S2, T>
where
    S1: Set<T>,
    S2: Set<T>,
{
}

impl<'a, S1, S2, T> Debug for SymmetricDifference<'a, S1, S2, T>
where
    S1: Set<T>,
    S2: Set<T>,
    <S1 as SetIteration<T>>::Iterator<'a>: Clone,
    <S2 as SetIteration<T>>::Iterator<'a>: Clone,
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.iter.fmt(f)
    }
}

/// An iterator that returns the difference between two sets.
pub struct Difference<'a, S1, S2, T>
where
    S1: Set<T>,
    S2: Set<T>,
    T: 'a,
{
    s1: &'a S1,
    s1_iter: <S1 as SetIteration<T>>::Iterator<'a>,
    s2: &'a S2,
}

impl<'a, S1, S2, T> Difference<'a, S1, S2, T>
where
    S1: Set<T>,
    S2: Set<T>,
{
    pub(crate) fn new(s1: &'a S1, s2: &'a S2) -> Self {
        Self {
            s1_iter: s1.iter(),
            s1,
            s2,
        }
    }
}

impl<'a, S1, S2, T> Iterator for Difference<'a, S1, S2, T>
where
    S1: Set<T>,
    S2: Set<T>,
{
    type Item = &'a T;

    #[allow(clippy::needless_borrow)]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let item = self.s1_iter.next()?;
            if !self.s2.contains(&item) {
                return Some(item);
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_, upper) = self.s1_iter.size_hint();
        (0, upper)
    }
}

impl<'a, S1, S2, T> Clone for Difference<'a, S1, S2, T>
where
    S1: Set<T>,
    S2: Set<T>,
    <S1 as SetIteration<T>>::Iterator<'a>: Clone,
    <S2 as SetIteration<T>>::Iterator<'a>: Clone,
{
    fn clone(&self) -> Self {
        Self {
            s1: self.s1,
            s1_iter: self.s1_iter.clone(),
            s2: self.s2,
        }
    }
}

impl<S1, S2, T> FusedIterator for Difference<'_, S1, S2, T>
where
    S1: Set<T>,
    S2: Set<T>,
{
}

impl<'a, S1, S2, T> Debug for Difference<'a, S1, S2, T>
where
    S1: Set<T>,
    S2: Set<T>,
    <S1 as SetIteration<T>>::Iterator<'a>: Clone,
    <S2 as SetIteration<T>>::Iterator<'a>: Clone,
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_list().entries((*self).clone()).finish()
    }
}

/// An iterator that returns the intersection between two sets.
pub struct Intersection<'a, S1, S2, T>
where
    S1: Set<T>,
    S2: Set<T>,
    T: 'a,
{
    s1: &'a S1,
    s1_iter: <S1 as SetIteration<T>>::Iterator<'a>,
    s2: &'a S2,
    s2_iter: <S2 as SetIteration<T>>::Iterator<'a>,
}

impl<'a, S1, S2, T> Intersection<'a, S1, S2, T>
where
    S1: Set<T>,
    S2: Set<T>,
{
    pub(crate) fn new(s1: &'a S1, s2: &'a S2) -> Self {
        Self {
            s1_iter: s1.iter(),
            s1,
            s2_iter: s2.iter(),
            s2,
        }
    }
}

impl<'a, S1, S2, T> Iterator for Intersection<'a, S1, S2, T>
where
    S1: Set<T>,
    S2: Set<T>,
{
    type Item = &'a T;

    #[allow(clippy::needless_borrow)]
    #[mutants::skip]
    fn next(&mut self) -> Option<Self::Item> {
        if self.s1.len() < self.s2.len() {
            loop {
                let item = self.s1_iter.next()?;
                if self.s2.contains(&item) {
                    return Some(item);
                }
            }
        } else {
            loop {
                let item = self.s2_iter.next()?;
                if self.s1.contains(&item) {
                    return Some(item);
                }
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(min(self.s1.len(), self.s2.len())))
    }
}

impl<'a, S1, S2, T> Clone for Intersection<'a, S1, S2, T>
where
    S1: Set<T>,
    S2: Set<T>,
    <S1 as SetIteration<T>>::Iterator<'a>: Clone,
    <S2 as SetIteration<T>>::Iterator<'a>: Clone,
{
    fn clone(&self) -> Self {
        Self {
            s1: self.s1,
            s1_iter: self.s1_iter.clone(),
            s2: self.s2,
            s2_iter: self.s2_iter.clone(),
        }
    }
}

impl<S1, S2, T> FusedIterator for Intersection<'_, S1, S2, T>
where
    S1: Set<T>,
    S2: Set<T>,
{
}

impl<'a, S1, S2, T> Debug for Intersection<'a, S1, S2, T>
where
    S1: Set<T>,
    S2: Set<T>,
    <S1 as SetIteration<T>>::Iterator<'a>: Clone,
    <S2 as SetIteration<T>>::Iterator<'a>: Clone,
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_list().entries((*self).clone()).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::maps::{IntoIter as MapIntoIter, Iter as MapIter};
    use alloc::string::String;
    use alloc::vec::Vec;
    use alloc::{format, vec};
    use hashbrown::HashSet as HashbrownSet;

    #[test]
    fn test_iter() {
        let entries = vec![("Alice", ()), ("Bob", ())];
        let map_iter = MapIter::new(&entries);
        let iter = Iter::new(map_iter);

        let collected: Vec<_> = iter.collect();
        assert_eq!(collected, vec![&"Alice", &"Bob"]);
    }

    #[test]
    fn test_iter_empty() {
        let entries: Vec<(&str, ())> = vec![];
        let map_iter = MapIter::new(&entries);
        let mut iter = Iter::new(map_iter);

        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_size_hint() {
        let entries = vec![("Alice", ()), ("Bob", ())];
        let map_iter = MapIter::new(&entries);
        let iter = Iter::new(map_iter);

        assert_eq!(iter.size_hint(), (2, Some(2)));
    }

    #[test]
    fn test_iter_clone() {
        let entries = vec![("Alice", ()), ("Bob", ())];
        let map_iter = MapIter::new(&entries);
        let iter = Iter::new(map_iter);
        let iter_clone = iter.clone();

        let collected: Vec<_> = iter_clone.collect();
        assert_eq!(collected, vec![&"Alice", &"Bob"]);
    }

    #[test]
    fn test_iter_debug() {
        let entries = vec![("Alice", ()), ("Bob", ())];
        let map_iter = MapIter::new(&entries);
        let iter = Iter::new(map_iter);

        let debug_str = format!("{iter:?}");
        assert!(debug_str.contains("Alice"));
        assert!(debug_str.contains("Bob"));
    }

    #[test]
    fn test_into_iter() {
        let entries = vec![("Alice", ()), ("Bob", ())];
        let map_into_iter = MapIntoIter::new(entries.into_boxed_slice());
        let into_iter = IntoIter::new(map_into_iter);

        let collected: Vec<_> = into_iter.collect();
        assert_eq!(collected, vec!["Alice", "Bob"]);
    }

    #[test]
    fn test_into_iter_empty() {
        let entries: Vec<(&str, ())> = vec![];
        let map_into_iter = MapIntoIter::new(entries.into_boxed_slice());
        let mut into_iter = IntoIter::new(map_into_iter);

        assert_eq!(into_iter.next(), None);
    }

    #[test]
    fn test_into_iter_size_hint() {
        let entries = vec![("Alice", ()), ("Bob", ())];
        let map_into_iter = MapIntoIter::new(entries.into_boxed_slice());
        let into_iter = IntoIter::new(map_into_iter);

        assert_eq!(into_iter.size_hint(), (2, Some(2)));
    }

    #[test]
    fn test_union() {
        let set1 = vec!["Alice", "Bob"].into_iter().collect::<HashbrownSet<_>>();
        let set2 = vec!["Bob", "Charlie"].into_iter().collect::<HashbrownSet<_>>();
        let union = Union::new(&set1, &set2);

        assert_eq!((2, Some(4)), union.size_hint());
        assert_eq!(3, union.clone().count());

        let mut collected: Vec<_> = union.collect();
        collected.sort();
        assert_eq!(collected, vec![&"Alice", &"Bob", &"Charlie"]);
    }

    #[test]
    fn test_union_empty() {
        let set1: HashbrownSet<&str> = HashbrownSet::new();
        let set2: HashbrownSet<&str> = HashbrownSet::new();
        let union = Union::new(&set1, &set2);

        assert_eq!(union.count(), 0);
    }

    #[test]
    fn test_symmetric_difference() {
        let set1 = vec!["Alice", "Bob"].into_iter().collect::<HashbrownSet<_>>();
        let set2 = vec!["Bob", "Charlie"].into_iter().collect::<HashbrownSet<_>>();
        let symmetric_difference = SymmetricDifference::new(&set1, &set2);

        assert_eq!((0, Some(4)), symmetric_difference.size_hint());
        assert_eq!(2, symmetric_difference.clone().count());

        let collected: Vec<_> = symmetric_difference.collect();
        assert_eq!(collected, vec![&"Alice", &"Charlie"]);
    }

    #[test]
    fn test_symmetric_difference_empty() {
        let set1: HashbrownSet<&str> = HashbrownSet::new();
        let set2: HashbrownSet<&str> = HashbrownSet::new();
        let symmetric_difference = SymmetricDifference::new(&set1, &set2);

        assert_eq!(symmetric_difference.count(), 0);
    }

    #[test]
    fn test_difference() {
        let set1 = vec!["Alice", "Bob"].into_iter().collect::<HashbrownSet<_>>();
        let set2 = vec!["Bob", "Charlie"].into_iter().collect::<HashbrownSet<_>>();
        let difference = Difference::new(&set1, &set2);

        assert_eq!((0, Some(2)), difference.size_hint());
        assert_eq!(1, difference.clone().count());

        let collected: Vec<_> = difference.collect();
        assert_eq!(collected, vec![&"Alice"]);
    }

    #[test]
    fn test_difference_empty() {
        let set1: HashbrownSet<&str> = HashbrownSet::new();
        let set2: HashbrownSet<&str> = HashbrownSet::new();
        let difference = Difference::new(&set1, &set2);

        assert_eq!(difference.count(), 0);
    }

    #[test]
    fn test_intersection() {
        let set1 = vec!["Alice", "Bob"].into_iter().collect::<HashbrownSet<_>>();
        let set2 = vec!["Bob", "Charlie"].into_iter().collect::<HashbrownSet<_>>();
        let intersection = Intersection::new(&set1, &set2);

        assert_eq!((0, Some(2)), intersection.size_hint());
        assert_eq!(1, intersection.clone().count());

        let collected: Vec<_> = intersection.collect();
        assert_eq!(collected, vec![&"Bob"]);
    }

    #[test]
    fn test_intersection_empty() {
        let set1: HashbrownSet<&str> = HashbrownSet::new();
        let set2: HashbrownSet<&str> = HashbrownSet::new();
        let intersection = Intersection::new(&set1, &set2);

        assert_eq!(intersection.count(), 0);
    }

    #[test]
    fn test_difference_clone() {
        let set1 = vec!["Alice", "Bob"].into_iter().collect::<HashbrownSet<_>>();
        let set2 = vec!["Bob", "Charlie"].into_iter().collect::<HashbrownSet<_>>();
        let difference = Difference::new(&set1, &set2);
        let difference_clone = difference.clone();

        let collected: Vec<_> = difference_clone.collect();
        assert_eq!(collected, vec![&"Alice"]);
    }

    #[test]
    fn test_intersection_clone() {
        let set1 = vec!["Alice", "Bob"].into_iter().collect::<HashbrownSet<_>>();
        let set2 = vec!["Bob", "Charlie"].into_iter().collect::<HashbrownSet<_>>();
        let intersection = Intersection::new(&set1, &set2);
        let intersection_clone = intersection.clone();

        let collected: Vec<_> = intersection_clone.collect();
        assert_eq!(collected, vec![&"Bob"]);
    }

    #[test]
    fn test_symmetric_difference_clone() {
        let set1 = vec!["Alice", "Bob"].into_iter().collect::<HashbrownSet<_>>();
        let set2 = vec!["Bob", "Charlie"].into_iter().collect::<HashbrownSet<_>>();
        let symmetric_difference = SymmetricDifference::new(&set1, &set2);
        let symmetric_difference_clone = symmetric_difference.clone();

        let collected: Vec<_> = symmetric_difference_clone.collect();
        assert_eq!(collected, vec![&"Alice", &"Charlie"]);
    }

    #[test]
    fn test_union_clone() {
        let set1 = vec!["Alice", "Bob"].into_iter().collect::<HashbrownSet<_>>();
        let set2 = vec!["Bob", "Charlie"].into_iter().collect::<HashbrownSet<_>>();
        let union = Union::new(&set1, &set2);
        let union_clone = union.clone();

        let mut collected: Vec<_> = union_clone.collect();
        collected.sort();
        assert_eq!(collected, vec![&"Alice", &"Bob", &"Charlie"]);
    }

    #[test]
    fn test_union_fmt() {
        let set1 = vec!["Alice", "Bob"].into_iter().collect::<HashbrownSet<_>>();
        let set2 = vec!["Bob", "Charlie"].into_iter().collect::<HashbrownSet<_>>();
        let union = Union::new(&set1, &set2);

        let debug_str = format!("{union:?}");
        assert!(debug_str.contains("Alice"));
        assert!(debug_str.contains("Bob"));
        assert!(debug_str.contains("Charlie"));
    }

    #[test]
    fn test_symmetric_difference_fmt() {
        let set1 = vec!["Alice", "Bob"].into_iter().collect::<HashbrownSet<_>>();
        let set2 = vec!["Bob", "Charlie"].into_iter().collect::<HashbrownSet<_>>();
        let symmetric_difference = SymmetricDifference::new(&set1, &set2);

        let debug_str = format!("{symmetric_difference:?}");
        assert!(debug_str.contains("Alice"));
        assert!(debug_str.contains("Charlie"));
    }

    #[test]
    fn test_difference_fmt() {
        let set1 = vec!["Alice", "Bob"].into_iter().collect::<HashbrownSet<_>>();
        let set2 = vec!["Bob", "Charlie"].into_iter().collect::<HashbrownSet<_>>();
        let difference = Difference::new(&set1, &set2);

        let debug_str = format!("{difference:?}");
        assert!(debug_str.contains("Alice"));
    }

    #[test]
    fn test_intersection_fmt() {
        let set1 = vec!["Alice", "Bob"].into_iter().collect::<HashbrownSet<_>>();
        let set2 = vec!["Bob", "Charlie"].into_iter().collect::<HashbrownSet<_>>();
        let intersection = Intersection::new(&set1, &set2);

        let debug_str = format!("{intersection:?}");
        assert!(debug_str.contains("Bob"));
    }

    #[test]
    fn test_iter_fold() {
        let entries = vec![("Alice", ()), ("Bob", ())];
        let map_iter = MapIter::new(&entries);
        let iter = Iter::new(map_iter);

        let result = iter.fold(String::new(), |mut acc, &name| {
            acc.push_str(name);
            acc
        });
        assert!(result.eq("AliceBob") || result.eq("BobAlice"));
    }

    #[test]
    fn test_iter_len() {
        let entries = vec![("Alice", ()), ("Bob", ())];
        let map_iter = MapIter::new(&entries);
        let iter = Iter::new(map_iter);

        assert_eq!(iter.len(), 2);
    }

    #[test]
    fn test_into_iter_fold() {
        let entries = vec![("Alice", ()), ("Bob", ())];
        let map_into_iter = MapIntoIter::new(entries.into_boxed_slice());
        let into_iter = IntoIter::new(map_into_iter);

        let result = into_iter.fold(String::new(), |mut acc, name| {
            acc.push_str(name);
            acc
        });
        assert!(result.eq("AliceBob") || result.eq("BobAlice"));
    }

    #[test]
    fn test_into_iter_len() {
        let entries = vec![("Alice", ()), ("Bob", ())];
        let map_into_iter = MapIntoIter::new(entries.into_boxed_slice());
        let into_iter = IntoIter::new(map_into_iter);

        assert_eq!(into_iter.len(), 2);
    }
}
