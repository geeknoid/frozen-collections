use std::cmp::min;
use std::fmt::{Debug, Formatter, Result};
use std::iter::FusedIterator;

use crate::traits::Set;

/// An iterator over the values of a set.
#[derive(Clone)]
pub struct Iter<'a, T> {
    entries: &'a [(T, ())],
    index: usize,
}

impl<'a, T> Iter<'a, T> {
    pub(crate) const fn new(entries: &'a [(T, ())]) -> Self {
        Self { entries, index: 0 }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.entries.len() {
            let entry = &self.entries[self.index];
            self.index += 1;
            Some(&entry.0)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.len()
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {
    fn len(&self) -> usize {
        self.entries.len() - self.index
    }
}

impl<'a, T> FusedIterator for Iter<'a, T> {}

impl<'a, T> Debug for Iter<'a, T>
where
    T: Debug + Clone,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_list().entries((*self).clone()).finish()
    }
}

/// A consuming iterator over the values of a set.
#[derive(Clone)]
pub struct IntoIter<T> {
    iter: std::vec::IntoIter<(T, ())>,
}

impl<T> IntoIter<T> {
    pub(crate) fn new(entries: Box<[(T, ())]>) -> Self {
        Self {
            iter: entries.into_vec().into_iter(),
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.iter.next()?;
        Some(item.0)
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

impl<T> ExactSizeIterator for IntoIter<T> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<T> FusedIterator for IntoIter<T> {}

impl<T> Debug for IntoIter<T>
where
    T: Clone + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_list().entries((*self).clone()).finish()
    }
}

/// An iterator that returns the union between two sets.
pub struct Union<'a, S1, S2, T>
where
    S1: Set<T> + ?Sized,
    S2: Set<T> + ?Sized,
    T: 'a,
{
    s1: &'a S1,
    s1_iter: <S1 as Set<T>>::Iterator<'a>,
    s2: &'a S2,
    s2_iter: <S2 as Set<T>>::Iterator<'a>,
}

impl<'a, S1, S2, T> Union<'a, S1, S2, T>
where
    S1: Set<T> + ?Sized,
    S2: Set<T> + ?Sized,
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
    S1: Set<T> + ?Sized,
    S2: Set<T> + ?Sized,
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
        let h1 = self.s1_iter.size_hint().1;
        let h2 = self.s2_iter.size_hint().1;

        if let Some(h1x) = h1 {
            if let Some(h2x) = h2 {
                return (0, h1x.checked_add(h2x));
            }
        }

        (0, None)
    }
}

impl<'a, S1, S2, T> Clone for Union<'a, S1, S2, T>
where
    S1: Set<T> + ?Sized,
    S2: Set<T> + ?Sized,
    <S1 as Set<T>>::Iterator<'a>: Clone,
    <S2 as Set<T>>::Iterator<'a>: Clone,
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
    S1: Set<T> + ?Sized,
    S2: Set<T> + ?Sized,
{
}

impl<'a, S1, S2, T> Debug for Union<'a, S1, S2, T>
where
    S1: Set<T> + ?Sized,
    S2: Set<T> + ?Sized,
    <S1 as Set<T>>::Iterator<'a>: Clone,
    <S2 as Set<T>>::Iterator<'a>: Clone,
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_list().entries((*self).clone()).finish()
    }
}

/// An iterator that returns the symmetric difference between two sets.
pub struct SymmetricDifference<'a, S1, S2, T>
where
    S1: Set<T> + ?Sized,
    S2: Set<T> + ?Sized,
    T: 'a,
{
    s1: &'a S1,
    s1_iter: <S1 as Set<T>>::Iterator<'a>,
    s2: &'a S2,
    s2_iter: <S2 as Set<T>>::Iterator<'a>,
}

impl<'a, S1, S2, T> SymmetricDifference<'a, S1, S2, T>
where
    S1: Set<T> + ?Sized,
    S2: Set<T> + ?Sized,
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

impl<'a, S1, S2, T> Iterator for SymmetricDifference<'a, S1, S2, T>
where
    S1: Set<T> + ?Sized,
    S2: Set<T> + ?Sized,
{
    type Item = &'a T;

    #[allow(clippy::needless_borrow)]
    fn next(&mut self) -> Option<Self::Item> {
        for item in self.s1_iter.by_ref() {
            if !self.s2.contains(&item) {
                return Some(item);
            }
        }

        self.s2_iter.by_ref().find(|&item| !self.s1.contains(&item))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let h1 = self.s1_iter.size_hint().1;
        let h2 = self.s2_iter.size_hint().1;

        if let Some(h1x) = h1 {
            if let Some(h2x) = h2 {
                return (0, h1x.checked_add(h2x));
            }
        }

        (0, None)
    }
}

impl<'a, S1, S2, T> Clone for SymmetricDifference<'a, S1, S2, T>
where
    S1: Set<T> + ?Sized,
    S2: Set<T> + ?Sized,
    <S1 as Set<T>>::Iterator<'a>: Clone,
    <S2 as Set<T>>::Iterator<'a>: Clone,
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

impl<'a, S1, S2, T> FusedIterator for SymmetricDifference<'a, S1, S2, T>
where
    S1: Set<T> + ?Sized,
    S2: Set<T> + ?Sized,
{
}

impl<'a, S1, S2, T> Debug for SymmetricDifference<'a, S1, S2, T>
where
    S1: Set<T> + ?Sized,
    S2: Set<T> + ?Sized,
    <S1 as Set<T>>::Iterator<'a>: Clone,
    <S2 as Set<T>>::Iterator<'a>: Clone,
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_list().entries((*self).clone()).finish()
    }
}

/// An iterator that returns the difference between two sets.
pub struct Difference<'a, S1, S2, T>
where
    S1: Set<T> + ?Sized,
    S2: Set<T> + ?Sized,
    T: 'a,
{
    s1: &'a S1,
    s1_iter: <S1 as Set<T>>::Iterator<'a>,
    s2: &'a S2,
}

impl<'a, S1, S2, T> Difference<'a, S1, S2, T>
where
    S1: Set<T> + ?Sized,
    S2: Set<T> + ?Sized,
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
    S1: Set<T> + ?Sized,
    S2: Set<T> + ?Sized,
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
}

impl<'a, S1, S2, T> Clone for Difference<'a, S1, S2, T>
where
    S1: Set<T> + ?Sized,
    S2: Set<T> + ?Sized,
    <S1 as Set<T>>::Iterator<'a>: Clone,
    <S2 as Set<T>>::Iterator<'a>: Clone,
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
    S1: Set<T> + ?Sized,
    S2: Set<T> + ?Sized,
{
}

impl<'a, S1, S2, T> Debug for Difference<'a, S1, S2, T>
where
    S1: Set<T> + ?Sized,
    S2: Set<T> + ?Sized,
    <S1 as Set<T>>::Iterator<'a>: Clone,
    <S2 as Set<T>>::Iterator<'a>: Clone,
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_list().entries((*self).clone()).finish()
    }
}

/// An iterator that returns the intersection between two sets.
pub struct Intersection<'a, S1, S2, T>
where
    S1: Set<T> + ?Sized,
    S2: Set<T> + ?Sized,
    T: 'a,
{
    s1: &'a S1,
    s1_iter: <S1 as Set<T>>::Iterator<'a>,
    s2: &'a S2,
    s2_iter: <S2 as Set<T>>::Iterator<'a>,
}

impl<'a, S1, S2, T> Intersection<'a, S1, S2, T>
where
    S1: Set<T> + ?Sized,
    S2: Set<T> + ?Sized,
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
    S1: Set<T> + ?Sized,
    S2: Set<T> + ?Sized,
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
    S1: Set<T> + ?Sized,
    S2: Set<T> + ?Sized,
    <S1 as Set<T>>::Iterator<'a>: Clone,
    <S2 as Set<T>>::Iterator<'a>: Clone,
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
    S1: Set<T> + ?Sized,
    S2: Set<T> + ?Sized,
{
}

impl<'a, S1, S2, T> Debug for Intersection<'a, S1, S2, T>
where
    S1: Set<T> + ?Sized,
    S2: Set<T> + ?Sized,
    <S1 as Set<T>>::Iterator<'a>: Clone,
    <S2 as Set<T>>::Iterator<'a>: Clone,
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_list().entries((*self).clone()).finish()
    }
}
