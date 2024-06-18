use std::any::type_name;
use std::fmt::{Debug, Formatter, Result};
use std::hash::RandomState;
use std::hash::{BuildHasher, Hash};
use std::mem::transmute;
use std::mem::MaybeUninit;
use std::ops::Index;
use std::ops::IndexMut;

use bitvec::macros::internal::funty::Fundamental;

use frozen_collections_core::analyzers::{
    analyze_int_keys, analyze_slice_keys, IntKeyAnalysisResult, SliceKeyAnalysisResult,
};

use crate::specialized_maps::*;
use crate::Len;

/// The different implementations available for use, depending on the type and content of the payload.
#[derive(Clone)]
enum MapTypes<K, V, BH> {
    Scanning(ScanningMap<K, V>),

    CommonSmall(CommonMap<K, V, u8, BH>),
    CommonLarge(CommonMap<K, V, usize, BH>),

    U32Small(IntegerMap<u32, V, u8>),
    U32Large(IntegerMap<u32, V, usize>),

    U32Range(IntegerRangeMap<u32, V>),

    LeftStringSliceSmall(LeftSliceMap<String, V, u8, BH>),
    LeftStringSliceLarge(LeftSliceMap<String, V, usize, BH>),

    RightStringSliceSmall(RightSliceMap<String, V, u8, BH>),
    RightStringSliceLarge(RightSliceMap<String, V, usize, BH>),

    StringLengthSmall(LengthMap<String, V, u8>),
}

/// A map optimized for fast read access.
///
/// A frozen map differs from the traditional [`HashMap`](std::collections::HashMap) type in three key ways. First, creating
/// a mew frozen map can take a relatively long time, especially for very large maps. Second,
/// once created, the keys in frozen maps are immutable. And third, probing a frozen map is
/// typically considerably faster, which is the whole point.
///
/// The reason creating a frozen map can take some time is due to the extensive analysis that is
/// performed on the map's keys in order to determine the best implementation strategy and data
/// layout to use. This analysis is what enables frozen maps to be faster later when
/// reading from the map.
///
/// Frozen maps are intended for long-lived maps, where the cost of creating the map is made up
/// over time by the faster read performance.
///
/// A `FrozenMap` requires that the elements
/// implement the [`Eq`] and [`Hash`] traits. This can frequently be achieved by
/// using `#[derive(PartialEq, Eq, Hash)]`. If you implement these yourself,
/// it is important that the following property holds:
///
/// ```text
/// k1 == k2 -> hash(k1) == hash(k2)
/// ```
///
/// In other words, if two keys are equal, their hashes must be equal.
/// Violating this property is a logic error.
///
/// It is also a logic error for a key to be modified in such a way that the key's
/// hash, as determined by the [`Hash`] trait, or its equality, as determined by
/// the [`Eq`] trait, changes while it is in the map. This is normally only
/// possible through [`Cell`], [`RefCell`], global state, I/O, or unsafe code.
///
/// The behavior resulting from either logic error is not specified, but will
/// be encapsulated to the `FrozenMap` that observed the logic error and not
/// result in undefined behavior. This could include panics, incorrect results,
/// aborts, memory leaks, and non-termination.
///
/// # Macros are Faster
///
/// If all your keys are known at compile time, you are much better off using the
/// [`frozen_map!`](crate::frozen_map!) macro rather than this type. This will result in considerably
/// better performance.
///
/// # Implementation Limits
///
/// Although frozen maps are always faster when reading than traditional hash maps, there are some
/// caveats to be aware of:
///
/// - [`FrozenMap`] has optimized implementations for the case where the keys are
///   of type [`u32`], but not any other integer types. This limitation doesn't exist
///   for the [`frozen_map!`](crate::frozen_map!) macro.
///
/// - [`FrozenMap`] has optimized implementations for the case where the keys are
///   of type [`String`], but not for the type `&str`. You will generally get considerably faster
///   performance using [`String`].
///
/// # Examples
///
/// ```
/// # use frozen_collections::FrozenMap;
/// # use frozen_collections::Len;
/// #
/// // Type inference lets us omit an explicit type signature (which
/// // would be `FrozenMap<String, String>` in this example).
/// let book_reviews = FrozenMap::try_from([
///     ("Adventures of Huckleberry Finn".to_string(), "My favorite book.".to_string()),
///     ("Grimms' Fairy Tales".to_string(), "Masterpiece.".to_string()),
///     ("Pride and Prejudice".to_string(), "Very enjoyable.".to_string()),
///     ("The Adventures of Sherlock Holmes".to_string(), "I liked it a lot.".to_string()),
/// ]).unwrap();
///
/// // Check for a specific one.
/// if !book_reviews.contains_key(&"Les Misérables".to_string()) {
///     println!("We've got {} reviews, but Les Misérables ain't one.",
///              book_reviews.len());
/// }
///
/// // Look up the values associated with some keys.
/// let to_find = ["Pride and Prejudice", "Alice's Adventure in Wonderland"];
/// for &book in &to_find {
///     match book_reviews.get(&book.to_string()) {
///         Some(review) => println!("{book}: {review}"),
///         None => println!("{book} is unreviewed.")
///     }
/// }
///
/// // Look up the value for a key (will panic if the key is not found).
/// println!("Review for Jane: {}", book_reviews[&"Pride and Prejudice".to_string()]);
///
/// // Iterate over everything.
/// for (book, review) in &book_reviews {
///     println!("{book}: \"{review}\"");
/// }
/// ```
///
/// The easiest way to use `FrozenMap` with a custom key type is to derive [`Eq`] and [`Hash`].
/// We must also derive [`PartialEq`].
///
/// [`RefCell`]: std::cell::RefCell
/// [`Cell`]: std::cell::Cell
/// [`default`]: Default::default
/// [`with_hasher`]: Self::with_hasher
///
/// ```
/// # use frozen_collections::FrozenMap;
/// #
/// #[derive(Hash, Eq, PartialEq, Debug)]
/// struct Viking {
///     name: String,
///     country: String,
/// }
///
/// impl Viking {
///     /// Creates a new Viking.
///     fn new(name: &str, country: &str) -> Viking {
///         Viking { name: name.to_string(), country: country.to_string() }
///     }
/// }
///
/// // Use a FrozenMap to store the vikings' health points.
/// let vikings = FrozenMap::try_from([
///     (Viking::new("Einar", "Norway"), 25),
///     (Viking::new("Olaf", "Denmark"), 24),
///     (Viking::new("Harald", "Iceland"), 12),
/// ]).unwrap();
///
/// // Use derived implementation to print the status of the vikings.
/// for (viking, health) in &vikings {
///     println!("{viking:?} has {health} hp");
/// }
/// ```
#[derive(Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct FrozenMap<K, V, BH = RandomState> {
    map_impl: MapTypes<K, V, BH>,
}

impl<K, V> FrozenMap<K, V, RandomState>
where
    K: Hash + Eq,
{
    /// Creates a frozen map.
    ///
    /// # Errors
    ///
    /// This fails if there are duplicate keys within the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FrozenMap;
    /// # use std::hash::RandomState;
    /// #
    /// let map = FrozenMap::new(vec![(1, 2), (3, 4)]).unwrap();
    /// ```
    pub fn new(payload: Vec<(K, V)>) -> std::result::Result<Self, &'static str> {
        Self::with_hasher(payload, RandomState::new())
    }
}

impl<K, V, BH> FrozenMap<K, V, BH>
where
    K: Hash + Eq,
    BH: BuildHasher,
{
    /// Creates a frozen map which will use the given hash builder to hash
    /// keys.
    ///
    /// # Errors
    ///
    /// This fails if there are duplicate keys within the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FrozenMap;
    /// # use std::hash::RandomState;
    /// #
    /// let map = FrozenMap::with_hasher(vec![(1, 2), (3, 4)], RandomState::new()).unwrap();
    /// ```
    pub fn with_hasher(payload: Vec<(K, V)>, bh: BH) -> std::result::Result<Self, &'static str> {
        Ok(Self {
            map_impl: if payload.len() < 4 {
                MapTypes::Scanning(ScanningMap::try_from(payload)?)
            } else if type_name::<K>() == type_name::<u32>() {
                Self::new_u32_map(payload)?
            } else if type_name::<K>() == type_name::<String>() {
                Self::new_string_map(payload, bh)?
            } else {
                Self::new_common_map(payload, bh)?
            },
        })
    }

    #[allow(clippy::transmute_undefined_repr)]
    fn new_u32_map(payload: Vec<(K, V)>) -> std::result::Result<MapTypes<K, V, BH>, &'static str> {
        let payload: Vec<(u32, V)> = unsafe { transmute(payload) };

        let key_analysis = analyze_int_keys(payload.iter().map(|x| x.0));

        match key_analysis {
            IntKeyAnalysisResult::Range => {
                Ok(MapTypes::U32Range(IntegerRangeMap::try_from(payload)?))
            }
            IntKeyAnalysisResult::Normal => {
                if payload.len() <= u8::MAX.as_usize() {
                    Ok(MapTypes::U32Small(IntegerMap::try_from(payload)?))
                } else {
                    Ok(MapTypes::U32Large(IntegerMap::try_from(payload)?))
                }
            }
        }
    }

    #[allow(clippy::transmute_undefined_repr)]
    fn new_string_map(
        payload: Vec<(K, V)>,
        bh: BH,
    ) -> std::result::Result<MapTypes<K, V, BH>, &'static str> {
        let payload: Vec<(String, V)> = unsafe { transmute(payload) };

        let key_analysis = analyze_slice_keys(payload.iter().map(|x| x.0.as_bytes()), &bh);

        if payload.len() <= u8::MAX.as_usize() {
            match key_analysis {
                SliceKeyAnalysisResult::Normal => Ok(MapTypes::CommonSmall(
                    CommonMap::with_hasher(unsafe { transmute(payload) }, bh)?,
                )),

                SliceKeyAnalysisResult::LeftHandSubslice {
                    subslice_index,
                    subslice_len,
                } => Ok(MapTypes::LeftStringSliceSmall(LeftSliceMap::with_hasher(
                    payload,
                    subslice_index..subslice_index + subslice_len,
                    bh,
                )?)),

                SliceKeyAnalysisResult::RightHandSubslice {
                    subslice_index,
                    subslice_len,
                } => Ok(MapTypes::RightStringSliceSmall(RightSliceMap::with_hasher(
                    payload,
                    subslice_index..subslice_index + subslice_len,
                    bh,
                )?)),

                SliceKeyAnalysisResult::Length => {
                    Ok(MapTypes::StringLengthSmall(LengthMap::try_from(payload)?))
                }
            }
        } else {
            match key_analysis {
                SliceKeyAnalysisResult::Length | SliceKeyAnalysisResult::Normal => {
                    Ok(MapTypes::CommonLarge(CommonMap::with_hasher(
                        unsafe { transmute(payload) },
                        bh,
                    )?))
                }

                SliceKeyAnalysisResult::LeftHandSubslice {
                    subslice_index,
                    subslice_len,
                } => Ok(MapTypes::LeftStringSliceLarge(LeftSliceMap::with_hasher(
                    payload,
                    subslice_index..subslice_index + subslice_len,
                    bh,
                )?)),

                SliceKeyAnalysisResult::RightHandSubslice {
                    subslice_index,
                    subslice_len,
                } => Ok(MapTypes::RightStringSliceLarge(RightSliceMap::with_hasher(
                    payload,
                    subslice_index..subslice_index + subslice_len,
                    bh,
                )?)),
            }
        }
    }

    fn new_common_map(
        payload: Vec<(K, V)>,
        bh: BH,
    ) -> std::result::Result<MapTypes<K, V, BH>, &'static str> {
        if payload.len() <= u8::MAX.as_usize() {
            Ok(MapTypes::CommonSmall(CommonMap::with_hasher(payload, bh)?))
        } else {
            Ok(MapTypes::CommonLarge(CommonMap::with_hasher(payload, bh)?))
        }
    }

    /// Returns a reference to the value corresponding to the key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FrozenMap;
    /// #
    /// let map = FrozenMap::try_from([(1, "a".to_string())]).unwrap();
    /// assert_eq!(map.get(&1), Some(&"a".to_string()));
    /// assert_eq!(map.get(&2), None);
    /// ```
    #[inline]
    pub fn get(&self, key: &K) -> Option<&V> {
        match &self.map_impl {
            MapTypes::Scanning(m) => m.get(key),
            MapTypes::CommonSmall(m) => m.get(key),
            MapTypes::CommonLarge(m) => m.get(key),
            MapTypes::U32Small(m) => m.get(unsafe { transmute(key) }),
            MapTypes::U32Large(m) => m.get(unsafe { transmute(key) }),
            MapTypes::U32Range(m) => m.get(unsafe { transmute(key) }),
            MapTypes::LeftStringSliceSmall(m) => {
                let k: &String = unsafe { transmute(key) };
                m.get(k)
            }
            MapTypes::LeftStringSliceLarge(m) => {
                let k: &String = unsafe { transmute(key) };
                m.get(k)
            }
            MapTypes::RightStringSliceSmall(m) => {
                let k: &String = unsafe { transmute(key) };
                m.get(k)
            }
            MapTypes::RightStringSliceLarge(m) => {
                let k: &String = unsafe { transmute(key) };
                m.get(k)
            }
            MapTypes::StringLengthSmall(m) => {
                let k: &String = unsafe { transmute(key) };
                m.get(k)
            }
        }
    }

    /// Returns the key-value pair corresponding to the supplied key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FrozenMap;
    /// #
    /// let map = FrozenMap::try_from([(1, "a".to_string())]).unwrap();
    /// assert_eq!(map.get_key_value(&1), Some((&1, &"a".to_string())));
    /// assert_eq!(map.get_key_value(&2), None);
    /// ```
    #[inline]
    pub fn get_key_value(&self, key: &K) -> Option<(&K, &V)> {
        match &self.map_impl {
            MapTypes::Scanning(m) => m.get_key_value(key),
            MapTypes::CommonSmall(m) => m.get_key_value(key),
            MapTypes::CommonLarge(m) => m.get_key_value(key),
            MapTypes::U32Small(m) => unsafe { transmute(m.get_key_value(transmute(key))) },
            MapTypes::U32Large(m) => unsafe { transmute(m.get_key_value(transmute(key))) },
            MapTypes::U32Range(m) => unsafe { transmute(m.get_key_value(transmute(key))) },
            MapTypes::LeftStringSliceSmall(m) => unsafe {
                let k: &String = transmute(key);
                transmute(m.get_key_value(k))
            },
            MapTypes::LeftStringSliceLarge(m) => unsafe {
                let k: &String = transmute(key);
                transmute(m.get_key_value(k))
            },
            MapTypes::RightStringSliceSmall(m) => unsafe {
                let k: &String = transmute(key);
                transmute(m.get_key_value(k))
            },
            MapTypes::RightStringSliceLarge(m) => unsafe {
                let k: &String = transmute(key);
                transmute(m.get_key_value(k))
            },
            MapTypes::StringLengthSmall(m) => unsafe {
                let k: &String = transmute(key);
                transmute(m.get_key_value(k))
            },
        }
    }

    /// Returns a mutable reference to the value corresponding to the key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FrozenMap;
    /// #
    /// let mut map = FrozenMap::try_from([(1, "a".to_string())]).unwrap();
    /// assert_eq!(map.get_mut(&1), Some(&mut "a".to_string()));
    /// assert_eq!(map.get_mut(&2), None);
    /// ```
    #[inline]
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        match &mut self.map_impl {
            MapTypes::Scanning(m) => m.get_mut(key),
            MapTypes::CommonSmall(m) => m.get_mut(key),
            MapTypes::CommonLarge(m) => m.get_mut(key),
            MapTypes::U32Small(m) => m.get_mut(unsafe { transmute(key) }),
            MapTypes::U32Large(m) => m.get_mut(unsafe { transmute(key) }),
            MapTypes::U32Range(m) => {
                let k = unsafe { transmute(key) };
                m.get_mut(k)
            }
            MapTypes::LeftStringSliceSmall(m) => {
                let k: &String = unsafe { transmute(key) };
                m.get_mut(k)
            }
            MapTypes::LeftStringSliceLarge(m) => {
                let k: &String = unsafe { transmute(key) };
                m.get_mut(k)
            }
            MapTypes::RightStringSliceSmall(m) => {
                let k: &String = unsafe { transmute(key) };
                m.get_mut(k)
            }
            MapTypes::RightStringSliceLarge(m) => {
                let k: &String = unsafe { transmute(key) };
                m.get_mut(k)
            }
            MapTypes::StringLengthSmall(m) => {
                let k: &String = unsafe { transmute(key) };
                m.get_mut(k)
            }
        }
    }

    /// Attempts to get mutable references to `N` values in the map at once.
    ///
    /// Returns an array of length `N` with the results of each query. For soundness, at most one
    /// mutable reference will be returned to any value. `None` will be returned if any of the
    /// keys are duplicates or missing.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FrozenMap;
    /// #
    /// let mut libraries = FrozenMap::try_from([
    ///     ("Bodleian Library".to_string(), 1602),
    ///     ("Athenæum".to_string(), 1807),
    ///     ("Herzogin-Anna-Amalia-Bibliothek".to_string(), 1691),
    ///     ("Library of Congress".to_string(), 1800)
    /// ]).unwrap();
    ///
    /// let got = libraries.get_many_mut([
    ///     &"Athenæum".to_string(),
    ///     &"Library of Congress".to_string(),
    /// ]);
    /// assert_eq!(
    ///     got,
    ///     Some([
    ///         &mut 1807,
    ///         &mut 1800,
    ///     ]),
    /// );
    ///
    /// // Missing keys result in None
    /// let got = libraries.get_many_mut([
    ///     &"Athenæum".to_string(),
    ///     &"New York Public Library".to_string(),
    /// ]);
    /// assert_eq!(got, None);
    ///
    /// // Duplicate keys result in None
    /// let got = libraries.get_many_mut([
    ///     &"Athenæum".to_string(),
    ///     &"Athenæum".to_string(),
    /// ]);
    /// assert_eq!(got, None);
    /// ```
    #[allow(mutable_transmutes)]
    pub fn get_many_mut<const N: usize>(&mut self, keys: [&K; N]) -> Option<[&mut V; N]> {
        // ensure key uniqueness (assumes "keys" is a relatively small array)
        for i in 0..keys.len() {
            for j in 0..i {
                if keys[j].eq(keys[i]) {
                    return None;
                }
            }
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

    /// Returns `true` if the map contains a value for the specified key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FrozenMap;
    /// #
    /// let map = FrozenMap::try_from([(1, "a".to_string())]).unwrap();
    ///
    /// assert_eq!(map.contains_key(&1), true);
    /// assert_eq!(map.contains_key(&2), false);
    /// ```
    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    /// An iterator visiting all key-value pairs in arbitrary order.
    /// The iterator element type is `(&'a K, &'a V)`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FrozenMap;
    /// #
    /// let map = FrozenMap::try_from([
    ///     ("a".to_string(), 1),
    ///     ("b".to_string(), 2),
    ///     ("c".to_string(), 3),
    /// ]).unwrap();
    ///
    /// for (key, val) in map.iter() {
    ///     println!("key: {key} val: {val}");
    /// }
    /// ```
    pub const fn iter(&self) -> Iter<K, V> {
        match &self.map_impl {
            MapTypes::Scanning(m) => m.iter(),
            MapTypes::CommonSmall(m) => m.iter(),
            MapTypes::CommonLarge(m) => m.iter(),
            MapTypes::U32Small(m) => unsafe { transmute(m.iter()) },
            MapTypes::U32Large(m) => unsafe { transmute(m.iter()) },
            MapTypes::U32Range(m) => unsafe { transmute(m.iter()) },
            MapTypes::LeftStringSliceSmall(m) => unsafe { transmute(m.iter()) },
            MapTypes::LeftStringSliceLarge(m) => unsafe { transmute(m.iter()) },
            MapTypes::RightStringSliceSmall(m) => unsafe { transmute(m.iter()) },
            MapTypes::RightStringSliceLarge(m) => unsafe { transmute(m.iter()) },
            MapTypes::StringLengthSmall(m) => unsafe { transmute(m.iter()) },
        }
    }

    /// An iterator visiting all keys in arbitrary order.
    /// The iterator element type is `&'a K`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FrozenMap;
    /// #
    /// let map = FrozenMap::try_from([
    ///     ("a".to_string(), 1),
    ///     ("b".to_string(), 2),
    ///     ("c".to_string(), 3),
    /// ]).unwrap();
    ///
    /// for key in map.keys() {
    ///     println!("{key}");
    /// }
    /// ```
    pub const fn keys(&self) -> Keys<K, V> {
        match &self.map_impl {
            MapTypes::Scanning(m) => m.keys(),
            MapTypes::CommonSmall(m) => m.keys(),
            MapTypes::CommonLarge(m) => m.keys(),
            MapTypes::U32Small(m) => unsafe { transmute(m.keys()) },
            MapTypes::U32Large(m) => unsafe { transmute(m.keys()) },
            MapTypes::U32Range(m) => unsafe { transmute(m.keys()) },
            MapTypes::LeftStringSliceSmall(m) => unsafe { transmute(m.keys()) },
            MapTypes::LeftStringSliceLarge(m) => unsafe { transmute(m.keys()) },
            MapTypes::RightStringSliceSmall(m) => unsafe { transmute(m.keys()) },
            MapTypes::RightStringSliceLarge(m) => unsafe { transmute(m.keys()) },
            MapTypes::StringLengthSmall(m) => unsafe { transmute(m.keys()) },
        }
    }

    /// An iterator visiting all values in arbitrary order.
    /// The iterator element type is `&'a V`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FrozenMap;
    /// #
    /// let map = FrozenMap::try_from([
    ///     ("a".to_string(), 1),
    ///     ("b".to_string(), 2),
    ///     ("c".to_string(), 3),
    /// ]).unwrap();
    ///
    /// for val in map.values() {
    ///     println!("{val}");
    /// }
    /// ```
    pub const fn values(&self) -> Values<K, V> {
        match &self.map_impl {
            MapTypes::Scanning(m) => m.values(),
            MapTypes::CommonSmall(m) => m.values(),
            MapTypes::CommonLarge(m) => m.values(),
            MapTypes::U32Small(m) => unsafe { transmute(m.values()) },
            MapTypes::U32Large(m) => unsafe { transmute(m.values()) },
            MapTypes::U32Range(m) => unsafe { transmute(m.values()) },
            MapTypes::LeftStringSliceSmall(m) => unsafe { transmute(m.values()) },
            MapTypes::LeftStringSliceLarge(m) => unsafe { transmute(m.values()) },
            MapTypes::RightStringSliceSmall(m) => unsafe { transmute(m.values()) },
            MapTypes::RightStringSliceLarge(m) => unsafe { transmute(m.values()) },
            MapTypes::StringLengthSmall(m) => unsafe { transmute(m.values()) },
        }
    }

    /// A consuming iterator visiting all keys in arbitrary order.
    /// The iterator element type is `&'a K`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FrozenMap;
    /// #
    /// let map = FrozenMap::try_from([
    ///     ("a".to_string(), 1),
    ///     ("b".to_string(), 2),
    ///     ("c".to_string(), 3),
    /// ]).unwrap();
    ///
    /// for key in map.into_keys() {
    ///     println!("{key}");
    /// }
    /// ```
    #[allow(clippy::transmute_undefined_repr)]
    pub fn into_keys(self) -> IntoKeys<K, V> {
        match self.map_impl {
            MapTypes::Scanning(m) => m.into_keys(),
            MapTypes::CommonSmall(m) => m.into_keys(),
            MapTypes::CommonLarge(m) => m.into_keys(),
            MapTypes::U32Small(m) => unsafe { transmute(m.into_keys()) },
            MapTypes::U32Large(m) => unsafe { transmute(m.into_keys()) },
            MapTypes::U32Range(m) => unsafe { transmute(m.into_keys()) },
            MapTypes::LeftStringSliceSmall(m) => unsafe { transmute(m.into_keys()) },
            MapTypes::LeftStringSliceLarge(m) => unsafe { transmute(m.into_keys()) },
            MapTypes::RightStringSliceSmall(m) => unsafe { transmute(m.into_keys()) },
            MapTypes::RightStringSliceLarge(m) => unsafe { transmute(m.into_keys()) },
            MapTypes::StringLengthSmall(m) => unsafe { transmute(m.into_keys()) },
        }
    }

    /// A consuming iterator visiting all values in arbitrary order.
    /// The iterator element type is `&'a V`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FrozenMap;
    /// #
    /// let map = FrozenMap::try_from([
    ///     ("a".to_string(), 1),
    ///     ("b".to_string(), 2),
    ///     ("c".to_string(), 3),
    /// ]).unwrap();
    ///
    /// for val in map.into_values() {
    ///     println!("{val}");
    /// }
    /// ```
    #[allow(clippy::transmute_undefined_repr)]
    pub fn into_values(self) -> IntoValues<K, V> {
        match self.map_impl {
            MapTypes::Scanning(m) => m.into_values(),
            MapTypes::CommonSmall(m) => m.into_values(),
            MapTypes::CommonLarge(m) => m.into_values(),
            MapTypes::U32Small(m) => unsafe { transmute(m.into_values()) },
            MapTypes::U32Large(m) => unsafe { transmute(m.into_values()) },
            MapTypes::U32Range(m) => unsafe { transmute(m.into_values()) },
            MapTypes::LeftStringSliceSmall(m) => unsafe { transmute(m.into_values()) },
            MapTypes::LeftStringSliceLarge(m) => unsafe { transmute(m.into_values()) },
            MapTypes::RightStringSliceSmall(m) => unsafe { transmute(m.into_values()) },
            MapTypes::RightStringSliceLarge(m) => unsafe { transmute(m.into_values()) },
            MapTypes::StringLengthSmall(m) => unsafe { transmute(m.into_values()) },
        }
    }

    /// An iterator producing mutable references to all entries in arbitrary order.
    /// The iterator element type is `&'a V`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FrozenMap;
    /// #
    /// let mut map = FrozenMap::try_from([
    ///     ("a".to_string(), 1),
    ///     ("b".to_string(), 2),
    ///     ("c".to_string(), 3),
    /// ]).unwrap();
    ///
    /// // update all values
    /// for (_, val) in map.iter_mut() {
    ///     *val *= 2;
    /// }
    /// ```
    #[allow(clippy::transmute_undefined_repr)]
    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        match &mut self.map_impl {
            MapTypes::Scanning(m) => m.iter_mut(),
            MapTypes::CommonSmall(m) => m.iter_mut(),
            MapTypes::CommonLarge(m) => m.iter_mut(),
            MapTypes::U32Small(m) => unsafe { transmute(m.iter_mut()) },
            MapTypes::U32Large(m) => unsafe { transmute(m.iter_mut()) },
            MapTypes::U32Range(m) => unsafe { transmute(m.iter_mut()) },
            MapTypes::LeftStringSliceSmall(m) => unsafe { transmute(m.iter_mut()) },
            MapTypes::LeftStringSliceLarge(m) => unsafe { transmute(m.iter_mut()) },
            MapTypes::RightStringSliceSmall(m) => unsafe { transmute(m.iter_mut()) },
            MapTypes::RightStringSliceLarge(m) => unsafe { transmute(m.iter_mut()) },
            MapTypes::StringLengthSmall(m) => unsafe { transmute(m.iter_mut()) },
        }
    }

    /// An iterator visiting all values mutably in arbitrary order. The iterator element type is `&'a mut V`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use frozen_collections::FrozenMap;
    /// #
    /// let mut map = FrozenMap::try_from([
    ///     ("a", 1),
    ///     ("b", 2),
    ///     ("c", 3),
    ///  ]).unwrap();
    ///
    /// for val in map.values_mut() {
    ///     *val = *val + 10;
    /// }
    /// ```
    #[allow(clippy::transmute_undefined_repr)]
    pub fn values_mut(&mut self) -> ValuesMut<'_, K, V> {
        match &mut self.map_impl {
            MapTypes::Scanning(m) => m.values_mut(),
            MapTypes::CommonSmall(m) => m.values_mut(),
            MapTypes::CommonLarge(m) => m.values_mut(),
            MapTypes::U32Small(m) => unsafe { transmute(m.values_mut()) },
            MapTypes::U32Large(m) => unsafe { transmute(m.values_mut()) },
            MapTypes::U32Range(m) => unsafe { transmute(m.values_mut()) },
            MapTypes::LeftStringSliceSmall(m) => unsafe { transmute(m.values_mut()) },
            MapTypes::LeftStringSliceLarge(m) => unsafe { transmute(m.values_mut()) },
            MapTypes::RightStringSliceSmall(m) => unsafe { transmute(m.values_mut()) },
            MapTypes::RightStringSliceLarge(m) => unsafe { transmute(m.values_mut()) },
            MapTypes::StringLengthSmall(m) => unsafe { transmute(m.values_mut()) },
        }
    }
}

impl<K, V, BH> Len for FrozenMap<K, V, BH> {
    fn len(&self) -> usize {
        match &self.map_impl {
            MapTypes::Scanning(m) => m.len(),
            MapTypes::CommonSmall(m) => m.len(),
            MapTypes::CommonLarge(m) => m.len(),
            MapTypes::U32Small(m) => m.len(),
            MapTypes::U32Large(m) => m.len(),
            MapTypes::U32Range(m) => m.len(),
            MapTypes::LeftStringSliceSmall(m) => m.len(),
            MapTypes::LeftStringSliceLarge(m) => m.len(),
            MapTypes::RightStringSliceSmall(m) => m.len(),
            MapTypes::RightStringSliceLarge(m) => m.len(),
            MapTypes::StringLengthSmall(m) => m.len(),
        }
    }
}

impl<K, V> TryFrom<Vec<(K, V)>> for FrozenMap<K, V, RandomState>
where
    K: Hash + Eq,
{
    type Error = &'static str;

    fn try_from(payload: Vec<(K, V)>) -> std::result::Result<Self, Self::Error> {
        Self::new(payload)
    }
}

impl<K, V, const N: usize> TryFrom<[(K, V); N]> for FrozenMap<K, V, RandomState>
where
    K: Hash + Eq,
{
    type Error = &'static str;

    fn try_from(payload: [(K, V); N]) -> std::result::Result<Self, Self::Error> {
        Self::new(Vec::from_iter(payload))
    }
}

impl<K, V> FromIterator<(K, V)> for FrozenMap<K, V, RandomState>
where
    K: Hash + Eq,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        Self::new(Vec::from_iter(iter)).unwrap()
    }
}

impl<K, V, BH> Index<&K> for FrozenMap<K, V, BH>
where
    K: Hash + Eq,
    BH: BuildHasher,
{
    type Output = V;

    fn index(&self, index: &K) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<K, V, BH> IndexMut<&K> for FrozenMap<K, V, BH>
where
    K: Hash + Eq,
    BH: BuildHasher,
{
    fn index_mut(&mut self, index: &K) -> &mut V {
        self.get_mut(index).unwrap()
    }
}

impl<K, V, BH> Default for FrozenMap<K, V, BH>
where
    K: Hash + Eq + Default,
    V: Default,
    BH: BuildHasher + Default,
{
    fn default() -> Self {
        Self {
            map_impl: MapTypes::Scanning(ScanningMap::<K, V>::default()),
        }
    }
}

impl<K, V, BH> Debug for FrozenMap<K, V, BH>
where
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.map_impl {
            MapTypes::Scanning(m) => m.fmt(f),
            MapTypes::CommonSmall(m) => m.fmt(f),
            MapTypes::CommonLarge(m) => m.fmt(f),
            MapTypes::U32Small(m) => m.fmt(f),
            MapTypes::U32Large(m) => m.fmt(f),
            MapTypes::U32Range(m) => m.fmt(f),
            MapTypes::LeftStringSliceSmall(m) => m.fmt(f),
            MapTypes::LeftStringSliceLarge(m) => m.fmt(f),
            MapTypes::RightStringSliceSmall(m) => m.fmt(f),
            MapTypes::RightStringSliceLarge(m) => m.fmt(f),
            MapTypes::StringLengthSmall(m) => m.fmt(f),
        }
    }
}

impl<K, V, BH> PartialEq<Self> for FrozenMap<K, V, BH>
where
    K: Hash + Eq,
    V: PartialEq,
    BH: BuildHasher,
{
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self.iter()
            .all(|(key, value)| other.get(key).map_or(false, |v| *value == *v))
    }
}

impl<K, V, BH> Eq for FrozenMap<K, V, BH>
where
    K: Hash + Eq,
    V: Eq,
    BH: BuildHasher,
{
}

impl<'a, K, V, BH> IntoIterator for &'a FrozenMap<K, V, BH>
where
    K: Hash + Eq,
    BH: BuildHasher,
{
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, K, V, BH> IntoIterator for &'a mut FrozenMap<K, V, BH>
where
    K: Hash + Eq,
    BH: BuildHasher,
{
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<K, V, BH> IntoIterator for FrozenMap<K, V, BH>
where
    K: Hash + Eq,
    BH: BuildHasher,
{
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    #[allow(clippy::transmute_undefined_repr)]
    fn into_iter(self) -> Self::IntoIter {
        match self.map_impl {
            MapTypes::Scanning(m) => m.into_iter(),
            MapTypes::CommonSmall(m) => m.into_iter(),
            MapTypes::CommonLarge(m) => m.into_iter(),
            MapTypes::U32Small(m) => unsafe { transmute(m.into_iter()) },
            MapTypes::U32Large(m) => unsafe { transmute(m.into_iter()) },
            MapTypes::U32Range(m) => unsafe { transmute(m.into_iter()) },
            MapTypes::LeftStringSliceSmall(m) => unsafe { transmute(m.into_iter()) },
            MapTypes::LeftStringSliceLarge(m) => unsafe { transmute(m.into_iter()) },
            MapTypes::RightStringSliceSmall(m) => unsafe { transmute(m.into_iter()) },
            MapTypes::RightStringSliceLarge(m) => unsafe { transmute(m.into_iter()) },
            MapTypes::StringLengthSmall(m) => unsafe { transmute(m.into_iter()) },
        }
    }
}
