use std::any::type_name;
use std::collections::HashSet;
use std::fmt::{Debug, Formatter, Result};
use std::hash::RandomState;
use std::hash::{BuildHasher, Hash};
use std::intrinsics::transmute;
use std::ops::{BitAnd, BitOr, BitXor, Sub};

use bitvec::macros::internal::funty::Fundamental;

use frozen_collections_core::analyzers::{
    analyze_int_keys, analyze_slice_keys, IntKeyAnalysisResult, SliceKeyAnalysisResult,
};

use crate::specialized_sets::{
    CommonSet, IntegerRangeSet, IntegerSet, IntoIter, Iter, LeftSliceSet, LengthSet, RightSliceSet,
    ScanningSet,
};
use crate::Len;
use crate::Set;

/// The different implementations available for use, depending on the type and content of the payload.
#[derive(Clone)]
enum SetTypes<T, BH> {
    Scanning(ScanningSet<T>),

    CommonSmall(CommonSet<T, u8, BH>),
    CommonLarge(CommonSet<T, usize, BH>),

    U32Small(IntegerSet<u32, u8>),
    U32Large(IntegerSet<u32, usize>),

    U32Range(IntegerRangeSet<u32>),

    LeftStringSliceSmall(LeftSliceSet<String, u8, BH>),
    LeftStringSliceLarge(LeftSliceSet<String, usize, BH>),

    RightStringSliceSmall(RightSliceSet<String, u8, BH>),
    RightStringSliceLarge(RightSliceSet<String, usize, BH>),

    StringLengthSmall(LengthSet<String, u8>),
}

/// A set optimized for fast read access.
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
/// A `FrozenSet` requires that the elements
/// implement the [`Eq`] and [`Hash`] traits. This can frequently be achieved by
/// using `#[derive(PartialEq, Eq, Hash)]`. If you implement these yourself,
/// it is important that the following property holds:
///
/// ```text
/// k1 == k2 -> hash(k1) == hash(k2)
/// ```
///
/// In other words, if two values are equal, their hashes must be equal.
/// Violating this property is a logic error.
///
/// It is also a logic error for a value to be modified in such a way that the value's
/// hash, as determined by the [`Hash`] trait, or its equality, as determined by
/// the [`Eq`] trait, changes while it is in the set. This is normally only
/// possible through [`Cell`], [`RefCell`], global state, I/O, or unsafe code.
///
/// The behavior resulting from either logic error is not specified, but will
/// be encapsulated to the `FrozenSet` that observed the logic error and not
/// result in undefined behavior. This could include panics, incorrect results,
/// aborts, memory leaks, and non-termination.
///
/// # Macros are Faster
///
/// If all your values are known at compile time, you are much better off using the
/// [`frozen_set!`](crate::frozen_set!) macro rather than this type. This will result in considerably
/// better performance.
///
/// # Implementation Limits
///
/// Although frozen sets are always faster when reading than traditional hash sets, there are some
/// caveats to be aware of:
///
/// - [`FrozenSet`] has optimized implementations for the case where the values are
///   of type [`u32`], but not any other integer types. This limitation doesn't exist
///   for the [`frozen_set!`](crate::frozen_set!) macro.
///
/// - [`FrozenSet`] has optimized implementations for the case where the values are
///   of type [`String`], but not for the type `&str`. You will generally get considerably faster
///   performance using [`String`].
///
/// # Examples
///
/// ```
/// # use std::hash::RandomState;
/// # use frozen_collections::FrozenSet;
/// # use frozen_collections::Len;
/// #
/// let books = FrozenSet::try_from(vec![
///     "A Dance With Dragons".to_string(),
///     "To Kill a Mockingbird".to_string(),
///     "The Odyssey".to_string(),
///     "The Great Gatsby".to_string()]).unwrap();
///
/// // Check for a specific one.
/// if !books.contains(&"The Winds of Winter".to_string()) {
///     println!("We have {} books, but The Winds of Winter ain't one.",
///              books.len());
/// }
///
/// // Iterate over everything.
/// for book in &books {
///     println!("{book}");
/// }
/// ```
///
/// The easiest way to use `FrozenSet` with a custom type is to derive
/// [`Eq`] and [`Hash`]. We must also derive [`PartialEq`],
/// which is required if [`Eq`] is derived.
///
/// ```
/// # use frozen_collections::FrozenSet;
/// #
/// #[derive(Hash, Eq, PartialEq, Debug)]
/// struct Viking {
///     name: String,
///     power: usize,
/// }
///
/// let vikings = FrozenSet::try_from([
///     Viking {name: "Einar".to_string(), power: 9 },
///     Viking { name: "Olaf".to_string(), power: 4 },
///     Viking { name: "Harald".to_string(), power: 8 }]).unwrap();
///
/// // Use derived implementation to print the vikings.
/// for x in &vikings {
///     println!("{x:?}");
/// }
/// ```
///
/// [`HashSet`]: HashSet
/// [`HashMap`]: std::collections::HashMap
/// [`RefCell`]: std::cell::RefCell
/// [`Cell`]: std::cell::Cell
#[derive(Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct FrozenSet<T, BH = RandomState> {
    set_impl: SetTypes<T, BH>,
}

impl<T> FrozenSet<T, RandomState>
where
    T: Hash + Eq,
{
    /// Creates a new frozen set.
    ///
    /// # Errors
    ///
    /// This fails if there are duplicate items within the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FrozenSet;
    /// # use std::hash::RandomState;
    /// # use frozen_collections::Len;
    /// #
    /// let set = FrozenSet::new(vec![1, 2, 3]).unwrap();
    ///
    /// assert_eq!(set.len(), 3);
    /// assert!(set.contains(&1));
    /// ```
    pub fn new(payload: Vec<T>) -> std::result::Result<Self, &'static str> {
        Self::with_hasher(payload, RandomState::new())
    }
}

impl<T, BH> FrozenSet<T, BH>
where
    T: Hash + Eq,
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
    /// # use frozen_collections::FrozenSet;
    /// # use std::hash::RandomState;
    /// # use frozen_collections::Len;
    /// #
    /// let set = FrozenSet::with_hasher(vec![1, 2, 3], RandomState::new()).unwrap();
    ///
    /// assert_eq!(set.len(), 3);
    /// assert!(set.contains(&1));
    /// ```
    pub fn with_hasher(payload: Vec<T>, bh: BH) -> std::result::Result<Self, &'static str> {
        Ok(Self {
            set_impl: if payload.len() < 4 {
                SetTypes::Scanning(ScanningSet::try_from(payload)?)
            } else if type_name::<T>() == type_name::<u32>() {
                Self::new_u32_set(payload)?
            } else if type_name::<T>() == type_name::<String>() {
                Self::new_string_set(payload, bh)?
            } else {
                Self::new_common_set(payload, bh)?
            },
        })
    }

    #[allow(clippy::transmute_undefined_repr)]
    fn new_u32_set(payload: Vec<T>) -> std::result::Result<SetTypes<T, BH>, &'static str> {
        let payload: Vec<u32> = unsafe { transmute(payload) };

        let key_analysis = analyze_int_keys(payload.iter().copied());

        match key_analysis {
            IntKeyAnalysisResult::Range => {
                Ok(SetTypes::U32Range(IntegerRangeSet::try_from(payload)?))
            }
            IntKeyAnalysisResult::Normal => {
                if payload.len() <= u8::MAX.as_usize() {
                    Ok(SetTypes::U32Small(IntegerSet::try_from(payload)?))
                } else {
                    Ok(SetTypes::U32Large(IntegerSet::try_from(payload)?))
                }
            }
        }
    }

    #[allow(clippy::transmute_undefined_repr)]
    fn new_string_set(
        payload: Vec<T>,
        bh: BH,
    ) -> std::result::Result<SetTypes<T, BH>, &'static str> {
        let payload: Vec<String> = unsafe { transmute(payload) };

        let key_analysis = analyze_slice_keys(payload.iter().map(String::as_bytes), &bh);

        if payload.len() <= u8::MAX.as_usize() {
            match key_analysis {
                SliceKeyAnalysisResult::Normal => Ok(SetTypes::CommonSmall(
                    CommonSet::with_hasher(unsafe { transmute(payload) }, bh)?,
                )),

                SliceKeyAnalysisResult::LeftHandSubslice {
                    subslice_index,
                    subslice_len,
                } => Ok(SetTypes::LeftStringSliceSmall(LeftSliceSet::with_hasher(
                    payload,
                    subslice_index..subslice_index + subslice_len,
                    bh,
                )?)),

                SliceKeyAnalysisResult::RightHandSubslice {
                    subslice_index,
                    subslice_len,
                } => Ok(SetTypes::RightStringSliceSmall(RightSliceSet::with_hasher(
                    payload,
                    subslice_index..subslice_index + subslice_len,
                    bh,
                )?)),

                SliceKeyAnalysisResult::Length => {
                    Ok(SetTypes::StringLengthSmall(LengthSet::try_from(payload)?))
                }
            }
        } else {
            match key_analysis {
                SliceKeyAnalysisResult::Length | SliceKeyAnalysisResult::Normal => {
                    Ok(SetTypes::CommonLarge(CommonSet::with_hasher(
                        unsafe { transmute(payload) },
                        bh,
                    )?))
                }

                SliceKeyAnalysisResult::LeftHandSubslice {
                    subslice_index,
                    subslice_len,
                } => Ok(SetTypes::LeftStringSliceLarge(LeftSliceSet::with_hasher(
                    payload,
                    subslice_index..subslice_index + subslice_len,
                    bh,
                )?)),

                SliceKeyAnalysisResult::RightHandSubslice {
                    subslice_index,
                    subslice_len,
                } => Ok(SetTypes::RightStringSliceLarge(RightSliceSet::with_hasher(
                    payload,
                    subslice_index..subslice_index + subslice_len,
                    bh,
                )?)),
            }
        }
    }

    fn new_common_set(
        payload: Vec<T>,
        bh: BH,
    ) -> std::result::Result<SetTypes<T, BH>, &'static str> {
        if payload.len() <= u8::MAX.as_usize() {
            Ok(SetTypes::CommonSmall(CommonSet::with_hasher(payload, bh)?))
        } else {
            Ok(SetTypes::CommonLarge(CommonSet::with_hasher(payload, bh)?))
        }
    }

    /// Returns `true` if the set contains a value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FrozenSet;
    /// #
    /// let set = FrozenSet::try_from([1, 2, 3]).unwrap();
    ///
    /// assert!(set.contains(&1));
    /// assert!(!set.contains(&4));
    /// ```
    pub fn contains(&self, value: &T) -> bool {
        match &self.set_impl {
            SetTypes::Scanning(s) => s.contains(value),
            SetTypes::CommonSmall(s) => s.contains(value),
            SetTypes::CommonLarge(s) => s.contains(value),
            SetTypes::U32Small(s) => s.contains(unsafe { transmute(value) }),
            SetTypes::U32Large(s) => s.contains(unsafe { transmute(value) }),
            SetTypes::U32Range(s) => s.contains(unsafe { transmute(value) }),
            SetTypes::LeftStringSliceSmall(s) => {
                let v: &String = unsafe { transmute(value) };
                s.contains(v)
            }
            SetTypes::LeftStringSliceLarge(s) => {
                let v: &String = unsafe { transmute(value) };
                s.contains(v)
            }
            SetTypes::RightStringSliceSmall(s) => {
                let v: &String = unsafe { transmute(value) };
                s.contains(v)
            }
            SetTypes::RightStringSliceLarge(s) => {
                let v: &String = unsafe { transmute(value) };
                s.contains(v)
            }
            SetTypes::StringLengthSmall(s) => {
                let v: &String = unsafe { transmute(value) };
                s.contains(v)
            }
        }
    }

    /// An iterator visiting all elements in arbitrary order.
    /// The iterator element type is `&'a T`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FrozenSet;
    /// #
    /// let set = FrozenSet::try_from(["a".to_string(), "b".to_string()]).unwrap();
    ///
    /// // Will print in an arbitrary order.
    /// for x in set.iter() {
    ///     println!("{x}");
    /// }
    /// ```
    pub fn iter(&self) -> Iter<T> {
        match &self.set_impl {
            SetTypes::Scanning(s) => s.iter(),
            SetTypes::CommonSmall(s) => s.iter(),
            SetTypes::CommonLarge(s) => s.iter(),
            SetTypes::U32Small(s) => unsafe { transmute(s.iter()) },
            SetTypes::U32Large(s) => unsafe { transmute(s.iter()) },
            SetTypes::U32Range(s) => unsafe { transmute(s.iter()) },
            SetTypes::LeftStringSliceSmall(s) => unsafe { transmute(s.iter()) },
            SetTypes::LeftStringSliceLarge(s) => unsafe { transmute(s.iter()) },
            SetTypes::RightStringSliceSmall(s) => unsafe { transmute(s.iter()) },
            SetTypes::RightStringSliceLarge(s) => unsafe { transmute(s.iter()) },
            SetTypes::StringLengthSmall(s) => unsafe { transmute(s.iter()) },
        }
    }

    /// Returns a reference to the value in the set, if any, that is equal to the given value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FrozenSet;
    /// #
    /// let set = FrozenSet::try_from([1, 2, 3]).unwrap();
    ///
    /// assert_eq!(set.get(&2), Some(&2));
    /// assert_eq!(set.get(&4), None);
    /// ```
    pub fn get(&self, value: &T) -> Option<&T> {
        match &self.set_impl {
            SetTypes::Scanning(s) => s.get(value),
            SetTypes::CommonSmall(s) => s.get(value),
            SetTypes::CommonLarge(s) => s.get(value),
            SetTypes::U32Small(s) => unsafe { transmute(s.get(transmute(value))) },
            SetTypes::U32Large(s) => unsafe { transmute(s.get(transmute(value))) },
            SetTypes::U32Range(s) => unsafe { transmute(s.get(transmute(value))) },
            SetTypes::LeftStringSliceSmall(s) => unsafe {
                let v: &String = transmute(value);
                transmute(s.get(v))
            },
            SetTypes::LeftStringSliceLarge(s) => unsafe {
                let v: &String = transmute(value);
                transmute(s.get(v))
            },
            SetTypes::RightStringSliceSmall(s) => unsafe {
                let v: &String = transmute(value);
                transmute(s.get(v))
            },
            SetTypes::RightStringSliceLarge(s) => unsafe {
                let v: &String = transmute(value);
                transmute(s.get(v))
            },
            SetTypes::StringLengthSmall(s) => unsafe {
                let v: &String = transmute(value);
                transmute(s.get(v))
            },
        }
    }
}

impl<T> TryFrom<Vec<T>> for FrozenSet<T, RandomState>
where
    T: Hash + Eq,
{
    type Error = &'static str;

    fn try_from(payload: Vec<T>) -> std::result::Result<Self, Self::Error> {
        Self::new(payload)
    }
}

impl TryFrom<Vec<&str>> for FrozenSet<String, RandomState> {
    type Error = &'static str;

    fn try_from(payload: Vec<&str>) -> std::result::Result<Self, Self::Error> {
        Self::new(payload.into_iter().map(&str::to_string).collect())
    }
}

impl<T, const N: usize> TryFrom<[T; N]> for FrozenSet<T, RandomState>
where
    T: Hash + Eq,
{
    type Error = &'static str;

    fn try_from(payload: [T; N]) -> std::result::Result<Self, Self::Error> {
        Self::new(Vec::from_iter(payload))
    }
}

impl<const N: usize> TryFrom<[&str; N]> for FrozenSet<String, RandomState> {
    type Error = &'static str;

    fn try_from(payload: [&str; N]) -> std::result::Result<Self, Self::Error> {
        Self::new(Vec::from_iter(payload.into_iter().map(&str::to_string)))
    }
}

impl<T> FromIterator<T> for FrozenSet<T, RandomState>
where
    T: Hash + Eq,
{
    fn from_iter<U: IntoIterator<Item = T>>(iter: U) -> Self {
        Self::new(Vec::from_iter(iter)).unwrap()
    }
}

impl<T, BH> Default for FrozenSet<T, BH>
where
    T: Hash + Eq + Default,
    BH: BuildHasher + Default,
{
    fn default() -> Self {
        Self {
            set_impl: SetTypes::Scanning(ScanningSet::<T>::default()),
        }
    }
}

impl<T, BH> Debug for FrozenSet<T, BH>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.set_impl {
            SetTypes::Scanning(s) => s.fmt(f),
            SetTypes::CommonSmall(s) => s.fmt(f),
            SetTypes::CommonLarge(s) => s.fmt(f),
            SetTypes::U32Small(s) => s.fmt(f),
            SetTypes::U32Large(s) => s.fmt(f),
            SetTypes::U32Range(s) => s.fmt(f),
            SetTypes::LeftStringSliceSmall(s) => s.fmt(f),
            SetTypes::LeftStringSliceLarge(s) => s.fmt(f),
            SetTypes::RightStringSliceSmall(s) => s.fmt(f),
            SetTypes::RightStringSliceLarge(s) => s.fmt(f),
            SetTypes::StringLengthSmall(s) => s.fmt(f),
        }
    }
}

impl<T, BH> PartialEq<Self> for FrozenSet<T, BH>
where
    T: Hash + Eq,
    BH: BuildHasher,
{
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self.iter().all(|value| other.get(value).is_some())
    }
}

impl<T, BH> Eq for FrozenSet<T, BH>
where
    T: Hash + Eq,
    BH: BuildHasher,
{
}

impl<T, ST, BH> BitOr<&ST> for &FrozenSet<T, BH>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
    BH: BuildHasher + Default,
{
    type Output = HashSet<T, BH>;

    fn bitor(self, rhs: &ST) -> Self::Output {
        self.union(rhs).cloned().collect()
    }
}

impl<T, ST, BH> BitAnd<&ST> for &FrozenSet<T, BH>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
    BH: BuildHasher + Default,
{
    type Output = HashSet<T, BH>;

    fn bitand(self, rhs: &ST) -> Self::Output {
        self.intersection(rhs).cloned().collect()
    }
}

impl<T, ST, BH> BitXor<&ST> for &FrozenSet<T, BH>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
    BH: BuildHasher + Default,
{
    type Output = HashSet<T, BH>;

    fn bitxor(self, rhs: &ST) -> Self::Output {
        self.symmetric_difference(rhs).cloned().collect()
    }
}

impl<T, ST, BH> Sub<&ST> for &FrozenSet<T, BH>
where
    T: Hash + Eq + Clone,
    ST: Set<T>,
    BH: BuildHasher + Default,
{
    type Output = HashSet<T, BH>;

    fn sub(self, rhs: &ST) -> Self::Output {
        self.difference(rhs).cloned().collect()
    }
}

impl<T, BH> IntoIterator for FrozenSet<T, BH>
where
    T: Hash + Eq,
    BH: BuildHasher,
{
    type Item = T;
    type IntoIter = IntoIter<T>;

    #[allow(clippy::transmute_undefined_repr)]
    fn into_iter(self) -> Self::IntoIter {
        match self.set_impl {
            SetTypes::Scanning(s) => s.into_iter(),
            SetTypes::CommonSmall(s) => s.into_iter(),
            SetTypes::CommonLarge(s) => s.into_iter(),
            SetTypes::U32Small(s) => unsafe { transmute(s.into_iter()) },
            SetTypes::U32Large(s) => unsafe { transmute(s.into_iter()) },
            SetTypes::U32Range(s) => unsafe { transmute(s.into_iter()) },
            SetTypes::LeftStringSliceSmall(s) => unsafe { transmute(s.into_iter()) },
            SetTypes::LeftStringSliceLarge(s) => unsafe { transmute(s.into_iter()) },
            SetTypes::RightStringSliceSmall(s) => unsafe { transmute(s.into_iter()) },
            SetTypes::RightStringSliceLarge(s) => unsafe { transmute(s.into_iter()) },
            SetTypes::StringLengthSmall(s) => unsafe { transmute(s.into_iter()) },
        }
    }
}

impl<'a, T, BH> IntoIterator for &'a FrozenSet<T, BH>
where
    T: Hash + Eq,
    BH: BuildHasher,
{
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

impl<T, BH> Len for FrozenSet<T, BH> {
    fn len(&self) -> usize {
        match &self.set_impl {
            SetTypes::Scanning(s) => Len::len(s),
            SetTypes::CommonSmall(s) => Len::len(s),
            SetTypes::CommonLarge(s) => Len::len(s),
            SetTypes::U32Small(s) => Len::len(s),
            SetTypes::U32Large(s) => Len::len(s),
            SetTypes::U32Range(s) => Len::len(s),
            SetTypes::LeftStringSliceSmall(s) => Len::len(s),
            SetTypes::LeftStringSliceLarge(s) => Len::len(s),
            SetTypes::RightStringSliceSmall(s) => Len::len(s),
            SetTypes::RightStringSliceLarge(s) => Len::len(s),
            SetTypes::StringLengthSmall(s) => Len::len(s),
        }
    }
}

impl<T, BH> Set<T> for FrozenSet<T, BH>
where
    T: Hash + Eq,
    BH: BuildHasher,
{
    type Iterator<'a> = Iter<'a, T>
    where
        T: 'a,
        BH: 'a;

    fn iter(&self) -> Iter<'_, T> {
        self.iter()
    }

    fn contains(&self, value: &T) -> bool {
        self.contains(value)
    }
}
