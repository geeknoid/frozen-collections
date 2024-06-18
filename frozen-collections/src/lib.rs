#![cfg_attr(not(any(test, feature = "std")), no_std)]

//! Frozen collections: fast partially immutable collections
//!
//! Frozen collections are designed to deliver improved
//! read performance relative to the standard [`HashMap`](std::collections::HashMap) and
//! [`HashSet`](std::collections::HashSet) types. They are ideal for use with long-lasting collections
//! which get initialized when an application starts and remain unchanged
//! permanently, or at least extended periods of time. This is a common
//! pattern in service applications.
//!
//! As part of creating a frozen collection, an analysis is performed over the data that the collection
//! will hold to determine the best layout and algorithm to use to deliver optimal performance.
//! This analysis can take some time, but the value in spending this time up front
//! is that the collections provide faster read-time performance.
//!
//! # Macros and Structs
//!
//! Frozen collections can be created in one of two ways:
//!
//! - **via Macros**. When you know the data to load into the collection at build time, you can use
//!   one of eight macros to create the collection: [`fz_hash_map!`], [`fz_ordered_map!`], [`fz_scalar_map!`],
//!   [`fz_string_map!`], [`fz_hash_set!`], or [`fz_ordered_set!`], [`fz_scalar_set!`], or [`fz_string_set!`].
//!   With these macros, analysis of the input data is done at compile time, so you avoid the analysis
//!   overhead at runtime. Additionally, using the macros is the most efficient way to use frozen
//!   collections as they generally result in the best performance overall.
//!
//! * **via Facade Types**. When you don't know the data to load into the collection at
//!   build time, you use the [`FzHashMap`], [`FzOrderedMap`], [`FzScalarMap`], [`FzStringMap`],
//!   [`FzHashSet`], [`FzOrderedSet`], [`FzScalarSet`], and [`FzStringSet`] types. With these
//!   types, analysis is performed over the data at runtime when the collections are created,
//!   which can take a while if there are millions of entries.
//!
//! Using the macros results in faster runtime performance, so they are the preferred choice if
//! possible. The facade types dispatch at runtime to
//! different specialized implementation types. When you use the macros instead, the selection
//! of implementation type is done at build time, and thus the cost of the runtime dispatch is
//! completely eliminated.
//!
//! The macros let you create static maps that are stored directly in the
//! binary image, eliminating the need to replicate the collection in the heap on startup.
//! The lookup performance for these collections tends to be the fastest because all the data is inlined
//! and indirections have been eliminated at compile time.
//!
//! # Analysis and Optimizations
//!
//! Unlike normal collections, the frozen collections require you to provide all the data for
//! the collection when you create the collection. The data you supply is analyzed which determines
//! what specific underlying implementation strategy to use and how to lay out data internally.
//!
//! The available implementation strategies are:
//!
//! - **Scalar as Hash**. When the keys are of an integer or enum type, this uses the keys themselves
//!   as hash codes, avoiding the overhead of hashing.
//!
//! - **Dense Scalar Lookup**. When the keys represent a contiguous range of integer or enum values,
//!   lookups use a simple array access instead of hashing.
//!
//! - **Sparse Scalar Lookup**. When the keys represent a sparse range of integer or enum values,
//!   lookups use a sparse array access instead of hashing.
//!
//! - **Length as Hash**. When the keys are of a slice type, this uses the length of the slices
//!   as hash codes, avoiding the overhead of hashing.
//!
//! - **Left Hand Substring Hashing**. When the keys are of a slice type, this uses sub-slices of
//!   the keys for hashing, reducing the overhead of hashing.
//!
//! - **Right Hand Substring Hashing**. Similar to the Left Hand Substring Hashing from above, but
//!   using right-aligned sub-slices instead.
//!
//! - **Linear Scan**. For very small collections, this avoids hashing completely by scanning through the
//!   keys in linear order.
//!
//! - **Ordered Scan**. For very small collections where the keys implement the [`Ord`] trait,
//!   this avoids hashing completely by scanning through the keys in linear order.
//!
//! - **Classic Hashing**. This is the fallback when none of the previous strategies apply. The
//!   frozen implementation is still generally faster than [`std::collections::HashMap`] and
//!   [`std::collections::HashSet`].
//!
//! # Features
//!
//! You can specify the following features when you include the `frozen-collections` crate in your
//! `Cargo.toml` file:
//!
//! - **`std`**. Enables small features only available when building with the standard library.
//! - **`macros`**. This feature enables the use of the [`fz_hash_map!`] and [`fz_hash_set!`] macros.
//! - **`facades`**. This feature enables the facade types, such as [`FzHashMap`] and [`FzHashSet`].
//!
//! All of the above features are enabled by default.

extern crate alloc;

#[cfg(feature = "facades")]
mod facades;

pub use frozen_collections_core::traits::{
    CollectionMagnitude, Hasher, Len, Map, MapIterator, Scalar, Set, SetIterator,
};

#[doc(hidden)]
pub use frozen_collections_core::traits::{LargeCollection, MediumCollection, SmallCollection};

#[cfg(feature = "facades")]
pub use crate::facades::*;

/// Creates an efficient map with a fixed set of keys that implement the `Hash` trait.
///
/// The specific type of the map that gets created depends on the key-value pairs you supply.
/// Analysis is performed at build time to use the most efficient map implementation possible.
/// The types used to implement the map vary, but they all have an API that is equivalent to
/// `FzHashMap`'s API.
///
/// If your keys are either integers or string slices, you should use the `fz_scalar_map`
/// or `fz_string_map` macros instead, as they provide better performance for those key types.
///
/// # Examples
///
/// You supply this macro with the set of key-value pairs to insert into the map.
/// Here's an example of creating a local map:
///
/// ```rust
/// # use frozen_collections::fz_hash_map;
/// #
/// // The key type we use to index into our example map
/// #[derive(Eq, PartialEq, Hash)]
/// struct Key {
///     pub name: &'static str,
///     pub age: i32,
/// }
///
/// let map = fz_hash_map! {
///     &Key { name: "Alice", age: 30}: 1,
///     &Key { name: "Bob", age: 40}: 2,
///     &Key { name: "Fred", age: 40}: 2,
/// };
///
/// assert_eq!(Some(&1), map.get(&Key { name: "Alice", age: 30 }));
/// assert_eq!(Some(&2), map.get(&Key { name: "Bob", age: 40 }));
/// assert_eq!(None, map.get(&Key { name: "Alice", age: 31 }));
/// assert_eq!(None, map.get(&Key { name: "Bob", age: 41 }) );
/// ```
///
/// Creating a static map uses the same macro, but with a different syntax:
///
/// ```rust
/// # use frozen_collections::fz_hash_map;
/// #
/// // The key type we use to index into our example map
/// #[derive(Eq, PartialEq, Hash)]
/// struct Key {
///     pub name: &'static str,
///     pub age: i32,
/// }
///
/// fz_hash_map!(static MY_MAP: MyMapType<&Key, i32> = {
///     &Key { name: "Alice", age: 30}: 1,
///     &Key { name: "Bob", age: 40}: 2,
///     &Key { name: "Fred", age: 40}: 2,
/// });
///
/// fn main() {
///     assert_eq!(Some(&1), MY_MAP.get(&Key { name: "Alice", age: 30 }));
///     assert_eq!(Some(&2), MY_MAP.get(&Key { name: "Bob", age: 40 }));
///     assert_eq!(None, MY_MAP.get(&Key { name: "Alice", age: 31 }));
///     assert_eq!(None, MY_MAP.get(&Key { name: "Bob", age: 41 }) );
/// }
/// ```
///
/// This use of the macro declares a static variable called `MY_MAP` which has keys which are
/// of type `Key` and values which are integers. The specific implementation type of the map is
/// determined after analyzing the keys. `MyMapType` is created as an alias for the implementation
/// type, which provides a stable name for the implementation type, which lets you mention the
/// type elsewhere in your code.
///
/// You can add the keyword `pub` before `static` to make the map instance public.
///
/// # Performance
///
/// When possible, the generated map instance will be declared in such a way that it is stored directly
/// in the binary image, which means that the map is available immediately when the application starts
/// without needing to be copied into the heap.
#[cfg(feature = "macros")]
pub use frozen_collections_macros::fz_hash_map;

/// Creates an efficient set with a fixed set of values that implement the `Hash` trait.
///
/// The specific type of the set that gets created depends on the values you supply.
/// Analysis is performed at build time to use the most efficient set implementation possible.
/// The types used to implement the set vary, but they all have an API that is equivalent to
/// `FzHashSet`'s API.
///
/// If your values are either integers or string slices, you should use the `fz_scalar_set`
/// or `fz_string_set` macros instead, as they provide better performance for those key types.
///
/// # Examples
///
/// You supply this macro with the set of values to insert into the set.
/// Here's an example of creating a local map:
///
/// ```rust
/// # use frozen_collections::fz_hash_set;
/// #
/// // The value type we use to index into our example set
/// #[derive(Eq, PartialEq, Hash)]
/// struct Key {
///     pub name: &'static str,
///     pub age: i32,
/// }
///
/// let set = fz_hash_set! {
///     &Key { name: "Alice", age: 30},
///     &Key { name: "Bob", age: 40},
///     &Key { name: "Fred", age: 40},
/// };
///
/// assert!(set.contains(&Key { name: "Alice", age: 30 }));
/// assert!(set.contains(&Key { name: "Bob", age: 40 }));
/// assert!(!set.contains(&Key { name: "Alice", age: 31 }));
/// assert!(!set.contains(&Key { name: "Bob", age: 41 }));
/// ```
///
/// Creating a static set uses the same macro, but with a different syntax:
///
/// ```rust
/// # use frozen_collections::fz_hash_set;
/// #
/// // The value type we use in our example set
/// #[derive(Eq, PartialEq, Hash)]
/// struct Key {
///     pub name: &'static str,
///     pub age: i32,
/// }
///
/// fz_hash_set!(static MY_SET: MySetType<&Key> = {
///     &Key { name: "Alice", age: 30},
///     &Key { name: "Bob", age: 40},
///     &Key { name: "Fred", age: 40},
/// });
///
/// fn main() {
///     assert!(MY_SET.contains(&Key { name: "Alice", age: 30 }));
///     assert!(MY_SET.contains(&Key { name: "Bob", age: 40 }));
///     assert!(!MY_SET.contains(&Key { name: "Alice", age: 31 }));
///     assert!(!MY_SET.contains(&Key { name: "Bob", age: 41 }));
/// }
/// ```
///
/// This use of the macro declares a static variable called `MY_SET` which has values which are
/// of type `Key`. The specific implementation type of the set is
/// determined after analyzing the values. `MySetType` is created as an alias for the implementation
/// type, which provides a stable name for the implementation type, which lets you mention the
/// type elsewhere in your code.
///
/// You can add the keyword `pub` before `static` to make the set instance public.
///
/// # Performance
///
/// When possible, the generated set instance will be declared in such a way that it is stored directly
/// in the binary image, which means that the set is available immediately when the application starts
/// without needing to be copied into the heap.
#[cfg(feature = "macros")]
pub use frozen_collections_macros::fz_hash_set;

/// Creates an efficient map with a fixed set of ordered keys.
///
/// The specific type of the map that gets created depends on the key-value pairs you supply.
/// Analysis is performed at build time to use the most efficient map implementation possible.
/// The types used to implement the map vary, but they all have an API that is equivalent to
/// `FzOrderedMap`'s API.
///
/// If your keys are either integers or string slices, you should use the `fz_scalar_map`
/// or `fz_string_map` macros instead, as they provide better performance for those key types.
///
/// # Examples
///
/// You supply this macro with the set of key-value pairs to insert into the map.
/// Here's an example of creating a local map:
///
/// ```rust
/// # use frozen_collections::fz_ordered_map;
/// #
/// // The key type we use to index into our example map
/// #[derive(Eq, PartialEq, Ord, PartialOrd)]
/// struct Key {
///     pub name: &'static str,
///     pub age: i32,
/// }
///
/// let map = fz_ordered_map! {
///     &Key { name: "Alice", age: 30}: 1,
///     &Key { name: "Bob", age: 40}: 2,
///     &Key { name: "Fred", age: 40}: 2,
/// };
///
/// assert_eq!(Some(&1), map.get(&Key { name: "Alice", age: 30 }));
/// assert_eq!(Some(&2), map.get(&Key { name: "Bob", age: 40 }));
/// assert_eq!(None, map.get(&Key { name: "Alice", age: 31 }));
/// assert_eq!(None, map.get(&Key { name: "Bob", age: 41 }) );
/// ```
///
/// Creating a static map uses the same macro, but with a different syntax:
///
/// ```rust
/// # use frozen_collections::fz_ordered_map;
/// #
/// // The key type we use to index into our example map
/// #[derive(Eq, PartialEq, Ord, PartialOrd)]
/// struct Key {
///     pub name: &'static str,
///     pub age: i32,
/// }
///
/// fz_ordered_map!(static MY_MAP: MyMapType<&Key, i32> = {
///     &Key { name: "Alice", age: 30}: 1,
///     &Key { name: "Bob", age: 40}: 2,
///     &Key { name: "Fred", age: 40}: 2,
/// });
///
/// fn main() {
///     assert_eq!(Some(&1), MY_MAP.get(&Key { name: "Alice", age: 30 }));
///     assert_eq!(Some(&2), MY_MAP.get(&Key { name: "Bob", age: 40 }));
///     assert_eq!(None, MY_MAP.get(&Key { name: "Alice", age: 31 }));
///     assert_eq!(None, MY_MAP.get(&Key { name: "Bob", age: 41 }) );
/// }
/// ```
///
/// This use of the macro declares a static variable called `MY_MAP` which has keys which are
/// of type `Key` and values which are integers. The specific implementation type of the map is
/// determined after analyzing the keys. `MyMapType` is created as an alias for the implementation
/// type, which provides a stable name for the implementation type, which lets you mention the
/// type elsewhere in your code.
///
/// You can add the keyword `pub` before `static` to make the map instance public.
///
/// # Performance
///
/// When possible, the generated map instance will be declared in such a way that it is stored directly
/// in the binary image, which means that the map is available immediately when the application starts
/// without needing to be copied into the heap.
#[cfg(feature = "macros")]
pub use frozen_collections_macros::fz_ordered_map;

/// Creates an efficient set with a fixed set of ordered values.
///
/// The specific type of the set that gets created depends on the values you supply.
/// Analysis is performed at build time to use the most efficient set implementation possible.
/// The types used to implement the set vary, but they all have an API that is equivalent to
/// `FzOrderedSet`'s API.
///
/// If your values are either integers or string slices, you should use the `fz_scalar_set`
/// or `fz_string_set` macros instead, as they provide better performance for those key types.
///
/// # Examples
///
/// You supply this macro with the set of values to insert into the set.
/// Here's an example of creating a local set:
///
/// ```rust
/// # use frozen_collections::fz_ordered_set;
/// #
/// // The value type we use to in our example set
/// #[derive(Eq, PartialEq, Ord, PartialOrd)]
/// struct Key {
///     pub name: &'static str,
///     pub age: i32,
/// }
///
/// let set = fz_ordered_set! {
///     &Key { name: "Alice", age: 30},
///     &Key { name: "Bob", age: 40},
///     &Key { name: "Fred", age: 40},
/// };
///
/// assert!(set.contains(&Key { name: "Alice", age: 30 }));
/// assert!(set.contains(&Key { name: "Bob", age: 40 }));
/// assert!(!set.contains(&Key { name: "Alice", age: 31 }));
/// assert!(!set.contains(&Key { name: "Bob", age: 41 }));
/// ```
///
/// Creating a static set uses the same macro, but with a different syntax:
///
/// ```rust
/// # use frozen_collections::fz_ordered_set;
/// #
/// // The value type we use in our example set
/// #[derive(Eq, PartialEq, Ord, PartialOrd)]
/// struct Key {
///     pub name: &'static str,
///     pub age: i32,
/// }
///
/// fz_ordered_set!(static MY_SET: MySetType<&Key> = {
///     &Key { name: "Alice", age: 30},
///     &Key { name: "Bob", age: 40},
///     &Key { name: "Fred", age: 40},
/// });
///
/// fn main() {
///     assert!(MY_SET.contains(&Key { name: "Alice", age: 30 }));
///     assert!(MY_SET.contains(&Key { name: "Bob", age: 40 }));
///     assert!(!MY_SET.contains(&Key { name: "Alice", age: 31 }));
///     assert!(!MY_SET.contains(&Key { name: "Bob", age: 41 }) );
/// }
/// ```
///
/// This use of the macro declares a static variable called `MY_SET` which has values which are
/// of type `Key`. The specific implementation type of the set is
/// determined after analyzing the values. `MySetType` is created as an alias for the implementation
/// type, which provides a stable name for the implementation type, which lets you mention the
/// type elsewhere in your code.
///
/// You can add the keyword `pub` before `static` to make the set instance public.
///
/// # Performance
///
/// When possible, the generated set instance will be declared in such a way that it is stored directly
/// in the binary image, which means that the map is available immediately when the application starts
/// without needing to be copied into the heap.
#[cfg(feature = "macros")]
pub use frozen_collections_macros::fz_ordered_set;

/// Creates an efficient map with a fixed set of scalar keys.
///
/// The specific type of the map that gets created depends on the keys and values you supply.
/// Analysis is performed at build time to use the most efficient map implementation possible.
/// The types used to implement the map vary, but they all have an API that is equivalent to
/// `FzScalarMap`'s API.
///
/// # Examples
///
/// You supply this macro with the set of key-value pairs to insert into the map.
/// Here's an example of creating a local map:
///
/// ```rust
/// # use frozen_collections::fz_scalar_map;
/// #
/// let map = fz_scalar_map! {
///     10: 1,
///     20: 2,
///     30: 3,
/// };
///
/// assert_eq!(map.get(&10), Some(&1));
/// assert_eq!(map.get(&20), Some(&2));
/// assert_eq!(map.get(&30), Some(&3));
/// assert_eq!(map.get(&40), None);
/// ```
///
/// Creating a static map uses the same macro, but with a different syntax:
///
/// ```rust
/// # use frozen_collections::fz_scalar_map;
/// #
/// fz_scalar_map!(static MY_MAP: MyMapType<i32, i32> = {
///     10: 1,
///     20: 2,
///     30: 3,
/// });
///
/// fn main() {
///     assert_eq!(MY_MAP.get(&10), Some(&1));
///     assert_eq!(MY_MAP.get(&20), Some(&2));
///     assert_eq!(MY_MAP.get(&30), Some(&3));
///     assert_eq!(MY_MAP.get(&40), None);
/// }
/// ```
///
/// This use of the macro declares a static variable called `MY_MAP` which has keys which are
/// integers and values which are integers. The specific implementation type of the map is
/// determined after analyzing the keys. `MyMapType` is created as an alias for the implementation
/// type, which provides a stable name for the implementation type, which lets you mention the
/// type elsewhere in your code.
///
/// You can add the keyword `pub` before `static` to make the map instance public.
///
/// You can also use this macro with an enum as key type:
///
/// ```rust
/// # use frozen_collections::{fz_scalar_map, Scalar};
/// #
/// // To use this enum with the macro, have it derive the `Scalar` trait.
/// #[derive(Scalar, Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
/// enum Color {
///     Red,
///     Green,
///     Blue,
/// }
///
/// let map = fz_scalar_map!(
///     Color::Red: 1,
///     Color::Green: 2,
/// );
///
/// assert_eq!(Some(&1), map.get(&Color::Red));
/// assert_eq!(Some(&2), map.get(&Color::Green));
/// assert_eq!(None, map.get(&Color::Blue));
/// ```
///
/// # Performance
///
/// When possible, the generated map instance will be declared in such a way that it is stored directly
/// in the binary image, which means that the map is available immediately when the application starts
/// without needing to be copied into the heap.
#[cfg(feature = "macros")]
pub use frozen_collections_macros::fz_scalar_map;

/// Creates an efficient set with a fixed set of scalar values.
///
/// The specific type of the set that gets created depends on the values you supply.
/// Analysis is performed at build time to use the most efficient set implementation possible.
/// The types used to implement the set vary, but they all have an API that is equivalent to
/// `FzScalarSet`'s API.
///
/// # Examples
///
/// You supply this macro with the set of values to insert into the set.
/// Here's an example of creating a local map:
///
/// ```rust
/// use frozen_collections::fz_scalar_set;
///
/// let set = fz_scalar_set! {
///     10,
///     20,
///     30,
/// };
///
/// assert!(set.contains(&10));
/// assert!(set.contains(&20));
/// assert!(set.contains(&30));
/// assert!(!set.contains(&40));
/// ```
///
/// Creating a static set uses the same macro, but with a different syntax:
///
/// ```rust
/// use frozen_collections::fz_scalar_set;
///
/// fz_scalar_set!(static MY_SET: MySetType<i32> = {
///     10,
///     20,
///     30,
/// });
///
/// fn main() {
///     assert!(MY_SET.contains(&10));
///     assert!(MY_SET.contains(&20));
///     assert!(MY_SET.contains(&30));
///     assert!(!MY_SET.contains(&40));
/// }
/// ```
///
/// This use of the macro declares a static variable called `MY_SET` which has values which are
/// integers. The specific implementation type of the set is
/// determined after analyzing the values. `MySetType` is created as an alias for the implementation
/// type, which provides a stable name for the implementation type, which lets you mention the
/// type elsewhere in your code.
///
/// You can add the keyword `pub` before `static` to make the set instance public.
///
/// You can also use this macro with an enum as key type:
///
/// ```rust
/// use frozen_collections::{fz_scalar_set, Scalar};
///
/// // To use this enum with the macro, have it derive the `Scalar` trait.
/// #[derive(Scalar, Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
/// enum Color {
///     Red,
///     Green,
///     Blue,
/// }
///
/// let set = fz_scalar_set!(
///     Color::Red,
///     Color::Green,
/// );
///
/// assert!(set.contains(&Color::Red));
/// assert!(set.contains(&Color::Green));
/// assert!(!set.contains(&Color::Blue));
/// ```
///
/// # Performance
///
/// When possible, the generated set instance will be declared in such a way that it is stored directly
/// in the binary image, which means that the set is available immediately when the application starts
/// without needing to be copied into the heap.
#[cfg(feature = "macros")]
pub use frozen_collections_macros::fz_scalar_set;

/// Creates an efficient map with a fixed set of string keys.
///
/// The specific type of the map that gets created depends on the keys and values you supply.
/// Analysis is performed at build time to use the most efficient map implementation possible.
/// The types used to implement the map vary, but they all have an API that is equivalent to
/// `FzStringMap`'s API.
///
/// # Examples
///
/// You supply this macro with the set of key-value pairs to insert into the map.
/// Here's an example of creating a local map:
///
/// ```rust
/// # use frozen_collections::fz_string_map;
/// #
/// let map = fz_string_map! {
///     "one": 1,
///     "two": 2,
///     "three": 3,
/// };
///
/// assert_eq!(map.get("one"), Some(&1));
/// assert_eq!(map.get("two"), Some(&2));
/// assert_eq!(map.get("three"), Some(&3));
/// assert_eq!(map.get("four"), None);
/// ```
///
/// Creating a static map uses the same macro, but with a different syntax:
///
/// ```rust
/// # use frozen_collections::fz_string_map;
/// #
/// fz_string_map!(static MY_MAP: MyMapType<&str, i32> = {
///     "one": 1,
///     "two": 2,
///     "three": 3,
/// });
///
/// fn main() {
///     assert_eq!(MY_MAP.get("one"), Some(&1));
///     assert_eq!(MY_MAP.get("two"), Some(&2));
///     assert_eq!(MY_MAP.get("three"), Some(&3));
///     assert_eq!(MY_MAP.get("four"), None);
/// }
/// ```
///
/// This use of the macro declares a static variable called `MY_MAP` which has keys which are
/// string slices and values which are integers. The specific implementation type of the map is
/// determined after analyzing the keys. `MyMapType` is created as an alias for the implementation
/// type, which provides a stable name for the implementation type, which lets you mention the
/// type elsewhere in your code.
///
/// You can add the keyword `pub` before `static` to make the map instance public.
///
/// # Performance
///
/// When possible, the generated map instance will be declared in such a way that it is stored directly
/// in the binary image, which means that the map is available immediately when the application starts
/// without needing to be copied into the heap.
#[cfg(feature = "macros")]
pub use frozen_collections_macros::fz_string_map;

/// Creates an efficient set with a fixed set of string values.
///
/// The specific type of the set that gets created depends on the values you supply.
/// Analysis is performed at build time to use the most efficient set implementation possible.
/// The types used to implement the set vary, but they all have an API that is equivalent to
/// `FzStringSet`'s API.
///
/// # Examples
///
/// You supply this macro with the set of values to insert into the set.
/// Here's an example of creating a local set:
///
/// ```rust
/// # use frozen_collections::fz_string_set;
/// #
/// let set = fz_string_set! {
///     "one",
///     "two",
///     "three",
/// };
///
/// assert!(set.contains("one"));
/// assert!(set.contains("two"));
/// assert!(set.contains("three"));
/// assert!(!set.contains("four"));
/// ```
///
/// Creating a static set uses the same macro, but with a different syntax:
///
/// ```rust
/// # use frozen_collections::fz_string_set;
/// #
/// fz_string_set!(static MY_SET: MySetType<&str> = {
///     "one",
///     "two",
///     "three",
/// });
///
/// fn main() {
///     assert!(MY_SET.contains("one"));
///     assert!(MY_SET.contains("two"));
///     assert!(MY_SET.contains("three"));
///     assert!(!MY_SET.contains("four"));
/// }
/// ```
///
/// This use of the macro declares a static variable called `MY_SET` which has values which are
/// string slices. The specific implementation type of the set is
/// determined after analyzing the values. `MySetType` is created as an alias for the implementation
/// type, which provides a stable name for the implementation type, which lets you mention the
/// type elsewhere in your code.
///
/// You can add the keyword `pub` before `static` to make the set instance public.
///
/// # Performance
///
/// When possible, the generated set instance will be declared in such a way that it is stored directly
/// in the binary image, which means that the set is available immediately when the application starts
/// without needing to be copied into the heap.
#[cfg(feature = "macros")]
pub use frozen_collections_macros::fz_string_set;

/// Implements the `Scalar` trait for an enum.
///
/// Implementing the `Scalar` trait for an enum allows you to use the enum with the `fz_scalar_map`
/// and `fz_scalar_set` macro and the `FzScalarMap` and `FzScalarSet` types.
///
/// The `Scalar` macro can only be used with enums that only include unit variants without
/// explicit discriminants.
#[cfg(feature = "macros")]
pub use frozen_collections_macros::Scalar;

#[doc(hidden)]
pub mod sets {
    pub use frozen_collections_core::sets::*;
}

#[doc(hidden)]
pub mod maps {
    pub use frozen_collections_core::maps::*;
}

#[doc(hidden)]
pub mod inline_maps {
    pub use frozen_collections_core::inline_maps::*;
}

#[doc(hidden)]
pub mod inline_sets {
    pub use frozen_collections_core::inline_sets::*;
}

#[doc(hidden)]
pub mod hashers {
    pub use frozen_collections_core::hashers::*;
}

#[doc(hidden)]
pub mod ahash {
    pub use ahash::RandomState;
}
