use crate::traits::{Set, SetIterator};
use core::cmp::min;
use core::fmt::{Debug, Formatter};
use core::iter::FusedIterator;
use std::iter::Chain;

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

impl<'a, T> ExactSizeIterator for Iter<'a, T> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<'a, T> FusedIterator for Iter<'a, T> {}

impl<'a, T> Clone for Iter<'a, T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<'a, T> Debug for Iter<'a, T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_list().entries((*self).clone()).finish()
    }
}

/// A consuming iterator over the values of a set.
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
    s1_iter: <S1 as SetIterator<T>>::Iterator<'a>,
    s2: &'a S2,
    s2_iter: <S2 as SetIterator<T>>::Iterator<'a>,
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

        let mut max = None;
        if let Some(h1x) = h1.1 {
            if let Some(h2x) = h2.1 {
                max = h1x.checked_add(h2x);
            }
        }

        (min(h1.0, h2.0), max)
    }
}

impl<'a, S1, S2, T> Clone for Union<'a, S1, S2, T>
where
    S1: Set<T>,
    S2: Set<T>,
    <S1 as SetIterator<T>>::Iterator<'a>: Clone,
    <S2 as SetIterator<T>>::Iterator<'a>: Clone,
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

impl<'a, S1, S2, T> FusedIterator for Union<'a, S1, S2, T>
where
    S1: Set<T>,
    S2: Set<T>,
{
}

impl<'a, S1, S2, T> Debug for Union<'a, S1, S2, T>
where
    S1: Set<T>,
    S2: Set<T>,
    <S1 as SetIterator<T>>::Iterator<'a>: Clone,
    <S2 as SetIterator<T>>::Iterator<'a>: Clone,
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
    <S1 as SetIterator<T>>::Iterator<'a>: Clone,
    <S2 as SetIterator<T>>::Iterator<'a>: Clone,
{
    fn clone(&self) -> Self {
        Self {
            iter: self.iter.clone(),
        }
    }
}

impl<'a, S1, S2, T> FusedIterator for SymmetricDifference<'a, S1, S2, T>
where
    S1: Set<T>,
    S2: Set<T>,
{
}

impl<'a, S1, S2, T> Debug for SymmetricDifference<'a, S1, S2, T>
where
    S1: Set<T>,
    S2: Set<T>,
    <S1 as SetIterator<T>>::Iterator<'a>: Clone,
    <S2 as SetIterator<T>>::Iterator<'a>: Clone,
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
    s1_iter: <S1 as SetIterator<T>>::Iterator<'a>,
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
    <S1 as SetIterator<T>>::Iterator<'a>: Clone,
    <S2 as SetIterator<T>>::Iterator<'a>: Clone,
{
    fn clone(&self) -> Self {
        Self {
            s1: self.s1,
            s1_iter: self.s1_iter.clone(),
            s2: self.s2,
        }
    }
}

impl<'a, S1, S2, T> FusedIterator for Difference<'a, S1, S2, T>
where
    S1: Set<T>,
    S2: Set<T>,
{
}

impl<'a, S1, S2, T> Debug for Difference<'a, S1, S2, T>
where
    S1: Set<T>,
    S2: Set<T>,
    <S1 as SetIterator<T>>::Iterator<'a>: Clone,
    <S2 as SetIterator<T>>::Iterator<'a>: Clone,
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
    s1_iter: <S1 as SetIterator<T>>::Iterator<'a>,
    s2: &'a S2,
    s2_iter: <S2 as SetIterator<T>>::Iterator<'a>,
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
    <S1 as SetIterator<T>>::Iterator<'a>: Clone,
    <S2 as SetIterator<T>>::Iterator<'a>: Clone,
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

impl<'a, S1, S2, T> FusedIterator for Intersection<'a, S1, S2, T>
where
    S1: Set<T>,
    S2: Set<T>,
{
}

impl<'a, S1, S2, T> Debug for Intersection<'a, S1, S2, T>
where
    S1: Set<T>,
    S2: Set<T>,
    <S1 as SetIterator<T>>::Iterator<'a>: Clone,
    <S2 as SetIterator<T>>::Iterator<'a>: Clone,
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_list().entries((*self).clone()).finish()
    }
}
