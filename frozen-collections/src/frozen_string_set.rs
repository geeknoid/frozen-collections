use std::collections::HashSet;
use std::fmt::{Debug, Formatter, Result};
use std::hash::BuildHasher;
use std::ops::{BitAnd, BitOr, BitXor, Sub};

use ahash::RandomState;

use frozen_collections_core::analyzers::{analyze_slice_keys, SliceKeyAnalysisResult};

use crate::specialized_sets::{CommonSet, IntoIter, Iter, LeftSliceSet, RightSliceSet};
use crate::Len;
use crate::Set;

/// The different implementations available for use, depending on the payload.
#[derive(Clone)]
enum SetTypes<BH> {
    LeftSlice(LeftSliceSet<String, usize, BH>),
    RightSlice(RightSliceSet<String, usize, BH>),
    Common(CommonSet<String, usize, BH>),
}

/// A set optimized for fast read access of string values.
///
/// A frozen set differs from the traditional [`HashSet`] type in three key ways. First, creating
/// a mew frozen set can take a relatively long time, especially for very large sets. Second,
/// once created, instances of frozen sets are immutable. And third, probing a frozen set is
/// typically considerably faster, which is the whole point
///
/// The reason creating a frozen set can take some time is due to the extensive analysis that is
/// performed on the set's values in order to determine the best set implementation strategy and
/// data layout to use. This analysis is what enables frozen sets to be faster later when
/// probing the set.
///
/// Frozen sets are intended for long-lived sets, where the cost of creating the set is made up
/// over time by the faster probing performance.
///
/// # Macros are Faster
///
/// If all your values are known at compile time, you are much better off using the
/// [`frozen_set!`](crate::frozen_set!) macro rather than this type. This will result in considerably
/// better performance.
///
/// # Examples
///
/// ```
/// # use std::hash::RandomState;
/// # use frozen_collections::FrozenStringSet;
/// # use frozen_collections::Len;
/// #
/// let books = FrozenStringSet::new(vec![
///     "A Dance With Dragons".to_string(),
///     "To Kill a Mockingbird".to_string(),
///     "The Odyssey".to_string(),
///     "The Great Gatsby".to_string()]).unwrap();
///
/// // Check for a specific one.
/// if !books.contains(&"The Winds of Winter") {
///     println!("We have {} books, but The Winds of Winter ain't one.",
///              books.len());
/// }
///
/// // Iterate over everything.
/// for book in &books {
///     println!("{book}");
/// }
/// ```
#[derive(Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct FrozenStringSet<BH = RandomState> {
    set_impl: SetTypes<BH>,
}

impl FrozenStringSet<RandomState> {
    /// Creates a new frozen set which will use the [`RandomState`] hasher to hash values.
    ///
    /// # Errors
    ///
    /// This fails if there are duplicate items within the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FrozenStringSet;
    /// # use std::hash::RandomState;
    /// # use frozen_collections::Len;
    /// #
    /// let set = FrozenStringSet::new(vec!["1".to_string(), "2".to_string(), "3".to_string()]).unwrap();
    ///
    /// assert_eq!(set.len(), 3);
    /// assert!(set.contains(&"1"));
    /// ```
    pub fn new(payload: Vec<String>) -> std::result::Result<Self, &'static str> {
        Self::with_hasher(payload, RandomState::new())
    }
}

impl<BH> FrozenStringSet<BH>
where
    BH: BuildHasher,
{
    /// Creates a new frozen set which will use the given hasher to hash values.
    ///
    /// # Errors
    ///
    /// This fails if there are duplicate items within the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FrozenStringSet;
    /// # use std::hash::RandomState;
    /// # use frozen_collections::Len;
    /// #
    /// let set = FrozenStringSet::with_hasher(vec!["1".to_string(), "2".to_string(), "3".to_string()], RandomState::new()).unwrap();
    ///
    /// assert_eq!(set.len(), 3);
    /// assert!(set.contains(&"1"));
    /// ```
    pub fn with_hasher(payload: Vec<String>, bh: BH) -> std::result::Result<Self, &'static str> {
        Ok(Self {
            set_impl: {
                let key_analysis = analyze_slice_keys(payload.iter().map(String::as_bytes), &bh);

                match key_analysis {
                    SliceKeyAnalysisResult::Normal | SliceKeyAnalysisResult::Length => {
                        SetTypes::Common(CommonSet::with_hasher(payload, bh)?)
                    }

                    SliceKeyAnalysisResult::LeftHandSubslice(range) => {
                        SetTypes::LeftSlice(LeftSliceSet::with_hasher(payload, range, bh)?)
                    }

                    SliceKeyAnalysisResult::RightHandSubslice(range) => {
                        SetTypes::RightSlice(RightSliceSet::with_hasher(payload, range, bh)?)
                    }
                }
            },
        })
    }

    /// Returns `true` if the set contains a value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FrozenStringSet;
    /// #
    /// let set = FrozenStringSet::new(vec!["1".to_string(), "2".to_string(), "3".to_string()]).unwrap();
    ///
    /// assert!(set.contains(&"1"));
    /// assert!(!set.contains(&"4"));
    /// ```
    #[inline(always)]
    #[must_use]
    pub fn contains(&self, value: &str) -> bool {
        match &self.set_impl {
            SetTypes::LeftSlice(s) => s.contains(value),
            SetTypes::RightSlice(s) => s.contains(value),
            SetTypes::Common(s) => s.contains(value),
        }
    }

    /// Returns a reference to the value in the set, if any, that is equal to the given value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FrozenStringSet;
    /// #
    /// let set = FrozenStringSet::new(vec!["1".to_string(), "2".to_string(), "3".to_string()]).unwrap();
    ///
    /// assert_eq!(set.get(&"2"), Some(&"2".to_string()));
    /// assert_eq!(set.get(&"4"), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn get(&self, value: &str) -> Option<&String> {
        match &self.set_impl {
            SetTypes::LeftSlice(s) => s.get(value),
            SetTypes::RightSlice(s) => s.get(value),
            SetTypes::Common(s) => s.get(value),
        }
    }
}

impl<BH> TryFrom<Vec<String>> for FrozenStringSet<BH>
where
    BH: BuildHasher + Default,
{
    type Error = &'static str;

    fn try_from(payload: Vec<String>) -> std::result::Result<Self, Self::Error> {
        Self::with_hasher(payload, BH::default())
    }
}

impl<BH> TryFrom<Vec<&str>> for FrozenStringSet<BH>
where
    BH: BuildHasher + Default,
{
    type Error = &'static str;

    fn try_from(payload: Vec<&str>) -> std::result::Result<Self, Self::Error> {
        Self::with_hasher(
            Vec::from_iter(payload.into_iter().map(str::to_string)),
            BH::default(),
        )
    }
}

impl<const N: usize, BH> TryFrom<[String; N]> for FrozenStringSet<BH>
where
    BH: BuildHasher + Default,
{
    type Error = &'static str;

    fn try_from(payload: [String; N]) -> std::result::Result<Self, Self::Error> {
        Self::with_hasher(Vec::from_iter(payload), BH::default())
    }
}

impl<const N: usize, BH> TryFrom<[&str; N]> for FrozenStringSet<BH>
where
    BH: BuildHasher + Default,
{
    type Error = &'static str;

    fn try_from(payload: [&str; N]) -> std::result::Result<Self, Self::Error> {
        Self::try_from(Vec::from_iter(payload.map(str::to_string)))
    }
}

impl<BH> FromIterator<String> for FrozenStringSet<BH>
where
    BH: BuildHasher + Default,
{
    fn from_iter<U: IntoIterator<Item = String>>(iter: U) -> Self {
        Self::try_from(Vec::from_iter(iter)).unwrap()
    }
}

impl<'a, BH> FromIterator<&'a str> for FrozenStringSet<BH>
where
    BH: BuildHasher + Default,
{
    fn from_iter<U: IntoIterator<Item = &'a str>>(iter: U) -> Self {
        Self::try_from(Vec::from_iter(iter)).unwrap()
    }
}

impl<BH> Default for FrozenStringSet<BH>
where
    BH: BuildHasher + Default,
{
    fn default() -> Self {
        Self {
            set_impl: SetTypes::Common(CommonSet::default()),
        }
    }
}

impl<BH> Debug for FrozenStringSet<BH>
where
    BH: BuildHasher,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.set_impl {
            SetTypes::LeftSlice(s) => s.fmt(f),
            SetTypes::RightSlice(s) => s.fmt(f),
            SetTypes::Common(s) => s.fmt(f),
        }
    }
}

impl<ST, BH> PartialEq<ST> for FrozenStringSet<BH>
where
    ST: Set<String>,
    BH: BuildHasher,
{
    fn eq(&self, other: &ST) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self.iter().all(|value| other.contains(value))
    }
}

impl<BH> Eq for FrozenStringSet<BH> where BH: BuildHasher {}

impl<ST, BH> BitOr<&ST> for &FrozenStringSet<BH>
where
    ST: Set<String>,
    BH: BuildHasher + Default,
{
    type Output = HashSet<String, BH>;

    fn bitor(self, rhs: &ST) -> Self::Output {
        self.union(rhs).cloned().collect()
    }
}

impl<ST, BH> BitAnd<&ST> for &FrozenStringSet<BH>
where
    ST: Set<String>,
    BH: BuildHasher + Default,
{
    type Output = HashSet<String, BH>;

    fn bitand(self, rhs: &ST) -> Self::Output {
        self.intersection(rhs).cloned().collect()
    }
}

impl<ST, BH> BitXor<&ST> for &FrozenStringSet<BH>
where
    ST: Set<String>,
    BH: BuildHasher + Default,
{
    type Output = HashSet<String, BH>;

    fn bitxor(self, rhs: &ST) -> Self::Output {
        self.symmetric_difference(rhs).cloned().collect()
    }
}

impl<ST, BH> Sub<&ST> for &FrozenStringSet<BH>
where
    ST: Set<String>,
    BH: BuildHasher + Default,
{
    type Output = HashSet<String, BH>;

    fn sub(self, rhs: &ST) -> Self::Output {
        self.difference(rhs).cloned().collect()
    }
}

impl<BH> IntoIterator for FrozenStringSet<BH>
where
    BH: BuildHasher,
{
    type Item = String;
    type IntoIter = IntoIter<String>;

    fn into_iter(self) -> Self::IntoIter {
        match self.set_impl {
            SetTypes::LeftSlice(s) => s.into_iter(),
            SetTypes::RightSlice(s) => s.into_iter(),
            SetTypes::Common(s) => s.into_iter(),
        }
    }
}

impl<'a, BH> IntoIterator for &'a FrozenStringSet<BH>
where
    BH: BuildHasher,
{
    type Item = &'a String;
    type IntoIter = Iter<'a, String>;

    fn into_iter(self) -> Iter<'a, String> {
        self.iter()
    }
}

impl<BH> Len for FrozenStringSet<BH> {
    fn len(&self) -> usize {
        match &self.set_impl {
            SetTypes::LeftSlice(s) => Len::len(s),
            SetTypes::RightSlice(s) => Len::len(s),
            SetTypes::Common(s) => Len::len(s),
        }
    }
}

impl<BH> Set<String> for FrozenStringSet<BH>
where
    BH: BuildHasher,
{
    type Iterator<'a> = Iter<'a, String>
    where
        BH: 'a;

    fn iter(&self) -> Iter<'_, String> {
        match &self.set_impl {
            SetTypes::LeftSlice(s) => s.iter(),
            SetTypes::RightSlice(s) => s.iter(),
            SetTypes::Common(s) => s.iter(),
        }
    }

    fn contains(&self, value: &String) -> bool {
        self.contains(value)
    }
}

#[cfg(test)]
mod tests {
    use ahash::RandomState;

    use super::*;

    #[test]
    fn little() {
        let mut payload = Vec::new();
        for i in 0..2 {
            payload.push(i.to_string());
        }
        run_tests(payload);
    }

    #[test]
    fn medium() {
        let mut payload = Vec::new();
        for i in 0..10 {
            payload.push(i.to_string());
        }
        run_tests(payload);
    }

    #[test]
    fn large() {
        let mut payload = Vec::new();
        for i in 0..300 {
            payload.push(i.to_string());
        }
        run_tests(payload);
    }

    fn run_tests(payload: Vec<String>) {
        misc(payload.clone());
        iter(payload.clone());
        fmt(payload.clone());
        bits(payload);
    }

    fn misc(payload: Vec<String>) {
        let set = FrozenStringSet::new(payload.clone()).unwrap();
        let mut hs = HashSet::<String, RandomState>::from_iter(payload.iter().cloned());

        assert_eq!(payload.len(), set.len());

        for value in &payload {
            assert!(set.contains(value));
            assert_eq!(value, set.get(value).unwrap());
        }

        assert!(!set.contains("XYZ"));
        assert_eq!(payload.len(), set.iter().count());
        assert_eq!(set, hs);

        hs.insert("XYZ".to_string());
        assert_ne!(set, hs);

        let set3 = FrozenStringSet::<RandomState>::from_iter(payload.clone());
        assert_eq!(set, set3);

        let set4 = FrozenStringSet::<RandomState>::try_from(payload).unwrap();
        assert_eq!(set, set4);

        let set5 = FrozenStringSet::<RandomState>::try_from([
            "0".to_string(),
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
        ])
        .unwrap();
        assert_eq!(4, set5.len());

        let set6 = FrozenStringSet::<RandomState>::try_from(["0", "1", "2", "3"]).unwrap();
        assert_eq!(4, set6.len());
    }

    fn iter(payload: Vec<String>) {
        let set = FrozenStringSet::new(payload).unwrap();

        let hs = HashSet::<String, RandomState>::from_iter(set.iter().cloned());
        assert_eq!(set, hs);

        let other_hs = HashSet::from_iter(set.clone());
        assert_eq!(hs, other_hs);

        let other_hs = HashSet::from_iter(set.clone());
        assert_eq!(hs, other_hs);

        let other_hs = HashSet::from_iter(set);
        assert_eq!(hs, other_hs);
    }

    fn fmt(payload: Vec<String>) {
        let set = FrozenStringSet::new(payload.clone()).unwrap();

        let formatted_set = format!("{set:?}");
        for value in payload {
            let value_str = format!("{value:?}");
            assert!(
                formatted_set.contains(&value_str),
                "Formatted string does not contain value: {value:?}"
            );
        }
    }

    fn bits(payload: Vec<String>) {
        let fs1 = FrozenStringSet::new(payload.clone()).unwrap();
        let fs2 = FrozenStringSet::new(
            payload
                .iter()
                .map(|x| {
                    let mut c = x.clone();
                    c.push('1');
                    c
                })
                .collect(),
        )
        .unwrap();

        let hs1 = HashSet::from_iter(payload.clone());
        let hs2 = HashSet::from_iter(payload.into_iter().map(|x| {
            let mut c = x;
            c.push('1');
            c
        }));

        assert_eq!(&fs1 | &fs2, &hs1 | &hs2);
        assert_eq!(&fs1 & &fs2, &hs1 & &hs2);
        assert_eq!(&fs1 ^ &fs2, &hs1 ^ &hs2);
        assert_eq!(&fs1 - &fs2, &hs1 - &hs2);
    }

    #[test]
    fn default() {
        let fs = FrozenStringSet::<RandomState>::default();
        assert_eq!(0, fs.len());
    }
}
