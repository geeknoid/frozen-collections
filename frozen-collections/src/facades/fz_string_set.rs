use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt::{Debug, Formatter, Result};
use core::hash::BuildHasher;
use core::ops::{BitAnd, BitOr, BitXor, Sub};

use ahash::RandomState;
use frozen_collections_core::analyzers::{
    analyze_hash_codes, analyze_slice_keys, SliceKeyAnalysisResult,
};
use frozen_collections_core::hashers::{BridgeHasher, LeftRangeHasher, RightRangeHasher};
use frozen_collections_core::sets::{HashSet, IntoIter, Iter};
use frozen_collections_core::traits::{Hasher, LargeCollection, Len, Set, SetIterator};
use frozen_collections_core::utils::dedup_keep_last;
use hashbrown::HashSet as HashbrownSet;

/// The different implementations available for use, depending on the entries.
#[derive(Clone)]
enum SetTypes<BH> {
    LeftRange(HashSet<String, LargeCollection, LeftRangeHasher<BH>>),
    RightRange(HashSet<String, LargeCollection, RightRangeHasher<BH>>),
    Hash(HashSet<String, LargeCollection, BridgeHasher<BH>>),
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
/// `frozen_set!` macro rather than this type. This will result in considerably
/// better performance.
///
/// # Examples
///
/// ```
/// # use frozen_collections::FzStringSet;
/// # use frozen_collections::Len;
/// #
/// let books = FzStringSet::new(vec![
///     "A Dance With Dragons".to_string(),
///     "To Kill a Mockingbird".to_string(),
///     "The Odyssey".to_string(),
///     "The Great Gatsby".to_string()]);
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
pub struct FzStringSet<BH = RandomState> {
    set_impl: SetTypes<BH>,
}

impl FzStringSet<RandomState> {
    /// Creates a new frozen set which will use the [`RandomState`] hasher to hash values.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FzStringSet;
    /// # use frozen_collections::Len;
    /// #
    /// let set = FzStringSet::new(vec!["1".to_string(), "2".to_string(), "3".to_string()]);
    ///
    /// assert_eq!(set.len(), 3);
    /// assert!(set.contains(&"1"));
    /// ```
    #[must_use]
    pub fn new(entries: Vec<String>) -> Self {
        Self::with_hasher(entries, RandomState::new())
    }
}

impl<BH> FzStringSet<BH>
where
    BH: BuildHasher,
{
    /// Creates a new frozen set which will use the given hasher to hash values.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FzStringSet;
    /// # use frozen_collections::Len;
    /// use ahash::RandomState;
    ///
    /// let set = FzStringSet::with_hasher(vec!["1".to_string(), "2".to_string(), "3".to_string()], RandomState::new());
    ///
    /// assert_eq!(set.len(), 3);
    /// assert!(set.contains(&"1"));
    /// ```
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn with_hasher(mut entries: Vec<String>, bh: BH) -> Self {
        entries.sort();
        dedup_keep_last(&mut entries);

        Self {
            set_impl: {
                match analyze_slice_keys(entries.iter().map(String::as_bytes), &bh) {
                    SliceKeyAnalysisResult::General | SliceKeyAnalysisResult::Length => {
                        let h = BridgeHasher::new(bh);
                        let analysis = analyze_hash_codes(entries.iter().map(|x| h.hash(&x)));
                        SetTypes::Hash(
                            HashSet::with_hasher(entries, analysis.num_hash_slots, h).unwrap(),
                        )
                    }

                    SliceKeyAnalysisResult::LeftHandSubslice(range) => {
                        let h = LeftRangeHasher::new(bh, range);
                        let analysis = analyze_hash_codes(entries.iter().map(|x| h.hash(x)));
                        SetTypes::LeftRange(
                            HashSet::with_hasher(entries, analysis.num_hash_slots, h).unwrap(),
                        )
                    }

                    SliceKeyAnalysisResult::RightHandSubslice(range) => {
                        let h = RightRangeHasher::new(bh, range);
                        let analysis = analyze_hash_codes(entries.iter().map(|x| h.hash(x)));
                        SetTypes::RightRange(
                            HashSet::with_hasher(entries, analysis.num_hash_slots, h).unwrap(),
                        )
                    }
                }
            },
        }
    }

    /// Returns `true` if the set contains a value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FzStringSet;
    /// #
    /// let set = FzStringSet::new(vec!["1".to_string(), "2".to_string(), "3".to_string()]);
    ///
    /// assert!(set.contains(&"1"));
    /// assert!(!set.contains(&"4"));
    /// ```
    #[inline(always)]
    #[must_use]
    pub fn contains(&self, value: &str) -> bool {
        match &self.set_impl {
            SetTypes::LeftRange(s) => s.contains(value),
            SetTypes::RightRange(s) => s.contains(value),
            SetTypes::Hash(s) => s.contains(value),
        }
    }

    /// Returns a reference to the value in the set, if any, that is equal to the given value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FzStringSet;
    /// #
    /// let set = FzStringSet::new(vec!["1".to_string(), "2".to_string(), "3".to_string()]);
    ///
    /// assert_eq!(set.get(&"2"), Some(&"2".to_string()));
    /// assert_eq!(set.get(&"4"), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn get(&self, value: &str) -> Option<&String> {
        match &self.set_impl {
            SetTypes::LeftRange(s) => s.get(value),
            SetTypes::RightRange(s) => s.get(value),
            SetTypes::Hash(s) => s.get(value),
        }
    }
}

impl<BH> From<Vec<String>> for FzStringSet<BH>
where
    BH: BuildHasher + Default,
{
    fn from(entries: Vec<String>) -> Self {
        Self::with_hasher(entries, BH::default())
    }
}

impl<BH> From<Vec<&str>> for FzStringSet<BH>
where
    BH: BuildHasher + Default,
{
    fn from(entries: Vec<&str>) -> Self {
        Self::with_hasher(
            Vec::from_iter(entries.into_iter().map(str::to_string)),
            BH::default(),
        )
    }
}

impl<const N: usize, BH> From<[String; N]> for FzStringSet<BH>
where
    BH: BuildHasher + Default,
{
    fn from(entries: [String; N]) -> Self {
        Self::with_hasher(Vec::from_iter(entries), BH::default())
    }
}

impl<const N: usize, BH> From<[&str; N]> for FzStringSet<BH>
where
    BH: BuildHasher + Default,
{
    fn from(entries: [&str; N]) -> Self {
        Self::with_hasher(Vec::from_iter(entries.map(str::to_string)), BH::default())
    }
}

impl<BH> FromIterator<String> for FzStringSet<BH>
where
    BH: BuildHasher + Default,
{
    fn from_iter<U: IntoIterator<Item = String>>(iter: U) -> Self {
        Self::with_hasher(Vec::from_iter(iter), BH::default())
    }
}

impl<'a, BH> FromIterator<&'a str> for FzStringSet<BH>
where
    BH: BuildHasher + Default,
{
    fn from_iter<U: IntoIterator<Item = &'a str>>(iter: U) -> Self {
        Self::with_hasher(
            Vec::from_iter(iter.into_iter().map(str::to_string)),
            BH::default(),
        )
    }
}

impl<BH> Len for FzStringSet<BH> {
    fn len(&self) -> usize {
        match &self.set_impl {
            SetTypes::LeftRange(s) => Len::len(s),
            SetTypes::RightRange(s) => Len::len(s),
            SetTypes::Hash(s) => Len::len(s),
        }
    }
}

impl<BH> Default for FzStringSet<BH>
where
    BH: Default,
{
    fn default() -> Self {
        Self {
            set_impl: SetTypes::Hash(frozen_collections_core::sets::HashSet::default()),
        }
    }
}

impl<BH> Debug for FzStringSet<BH> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.set_impl {
            SetTypes::LeftRange(s) => s.fmt(f),
            SetTypes::RightRange(s) => s.fmt(f),
            SetTypes::Hash(s) => s.fmt(f),
        }
    }
}

impl<ST, BH> PartialEq<ST> for FzStringSet<BH>
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

impl<BH> Eq for FzStringSet<BH> where BH: BuildHasher {}

impl<ST, BH> BitOr<&ST> for &FzStringSet<BH>
where
    ST: Set<String>,
    BH: BuildHasher + Default,
{
    type Output = HashbrownSet<String>;

    fn bitor(self, rhs: &ST) -> Self::Output {
        self.union(rhs).cloned().collect()
    }
}

impl<ST, BH> BitAnd<&ST> for &FzStringSet<BH>
where
    ST: Set<String>,
    BH: BuildHasher + Default,
{
    type Output = HashbrownSet<String>;

    fn bitand(self, rhs: &ST) -> Self::Output {
        self.intersection(rhs).cloned().collect()
    }
}

impl<ST, BH> BitXor<&ST> for &FzStringSet<BH>
where
    ST: Set<String>,
    BH: BuildHasher + Default,
{
    type Output = HashbrownSet<String>;

    fn bitxor(self, rhs: &ST) -> Self::Output {
        self.symmetric_difference(rhs).cloned().collect()
    }
}

impl<ST, BH> Sub<&ST> for &FzStringSet<BH>
where
    ST: Set<String>,
    BH: BuildHasher + Default,
{
    type Output = HashbrownSet<String>;

    fn sub(self, rhs: &ST) -> Self::Output {
        self.difference(rhs).cloned().collect()
    }
}

impl<BH> IntoIterator for FzStringSet<BH> {
    type Item = String;
    type IntoIter = IntoIter<String>;

    fn into_iter(self) -> Self::IntoIter {
        match self.set_impl {
            SetTypes::LeftRange(s) => s.into_iter(),
            SetTypes::RightRange(s) => s.into_iter(),
            SetTypes::Hash(s) => s.into_iter(),
        }
    }
}

impl<'a, BH> IntoIterator for &'a FzStringSet<BH> {
    type Item = &'a String;
    type IntoIter = Iter<'a, String>;

    fn into_iter(self) -> Iter<'a, String> {
        self.iter()
    }
}

impl<BH> SetIterator<String> for FzStringSet<BH> {
    type Iterator<'a> = Iter<'a, String>
    where
        BH: 'a;

    fn iter(&self) -> Iter<'_, String> {
        match &self.set_impl {
            SetTypes::LeftRange(s) => s.iter(),
            SetTypes::RightRange(s) => s.iter(),
            SetTypes::Hash(s) => s.iter(),
        }
    }
}

impl<BH> Set<String> for FzStringSet<BH>
where
    BH: BuildHasher,
{
    fn contains(&self, value: &String) -> bool {
        self.contains(value)
    }
}

#[cfg(test)]
mod tests {
    use ahash::RandomState;
    use hashbrown::HashSet as HashbrownSet;

    use super::*;

    #[test]
    fn little() {
        let mut entries = Vec::new();
        for i in 0..2 {
            entries.push(i.to_string());
        }
        run_tests(entries);
    }

    #[test]
    fn medium() {
        let mut entries = Vec::new();
        for i in 0..10 {
            entries.push(i.to_string());
        }
        run_tests(entries);
    }

    #[test]
    fn large() {
        let mut entries = Vec::new();
        for i in 0..300 {
            entries.push(i.to_string());
        }
        run_tests(entries);
    }

    fn run_tests(entries: Vec<String>) {
        misc(entries.clone());
        fmt(entries.clone());
        bits(entries);
    }

    fn misc(entries: Vec<String>) {
        let set = FzStringSet::new(entries.clone());
        let mut hs = HashbrownSet::<String, RandomState>::from_iter(entries.iter().cloned());

        assert_eq!(entries.len(), set.len());

        for value in &entries {
            assert!(set.contains(value));
            assert_eq!(value, set.get(value).unwrap());
        }

        assert!(!set.contains("XYZ"));
        assert_eq!(entries.len(), set.iter().count());
        assert_eq!(set, hs);

        hs.insert("XYZ".to_string());
        assert_ne!(set, hs);

        let set3 = FzStringSet::<RandomState>::from_iter(entries.clone());
        assert_eq!(set, set3);

        let set4 = FzStringSet::<RandomState>::from(entries);
        assert_eq!(set, set4);

        let set5 = FzStringSet::<RandomState>::from([
            "0".to_string(),
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
        ]);
        assert_eq!(4, set5.len());

        let set6 = FzStringSet::<RandomState>::from(["0", "1", "2", "3"]);
        assert_eq!(4, set6.len());
    }

    fn fmt(entries: Vec<String>) {
        let set = FzStringSet::new(entries.clone());

        let formatted_set = format!("{set:?}");
        for value in entries {
            let value_str = format!("{value:?}");
            assert!(
                formatted_set.contains(&value_str),
                "Formatted string does not contain value: {value:?}"
            );
        }
    }

    fn bits(entries: Vec<String>) {
        let fs1 = FzStringSet::new(entries.clone());
        let fs2 = FzStringSet::new(
            entries
                .iter()
                .map(|x| {
                    let mut c = x.clone();
                    c.push('1');
                    c
                })
                .collect(),
        );

        let hs1 = HashbrownSet::<String>::from_iter(entries.clone());
        let hs2 = HashbrownSet::<String>::from_iter(entries.into_iter().map(|x| {
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
        let fs = FzStringSet::<RandomState>::default();
        assert_eq!(0, fs.len());
    }
}
