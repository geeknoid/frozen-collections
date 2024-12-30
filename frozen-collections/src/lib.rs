#![cfg_attr(not(any(test, feature = "std")), no_std)]

//! Frozen collections: fast _partially_ immutable collections
//!
//! Frozen collections are designed to deliver improved
//! read performance relative to the standard [`HashMap`](std::collections::HashMap) and
//! [`HashSet`](std::collections::HashSet) types. They are ideal for use with long-lasting collections
//! which get initialized when an application starts and remain unchanged
//! permanently, or at least extended periods of time. This is a common
//! pattern in service applications.
//!
//! As part of creating a frozen collection, analysis is performed over the data that the collection
//! will hold to determine the best layout and algorithm to use to deliver optimal performance.
//! Depending on the situation, sometimes the analysis is done at compile-time whereas in
//! other cases it is done at runtime when the collection is initialized.
//! This analysis can take some time, but the value in spending this time up front
//! is that the collections provide faster read-time performance.
//!
//! Frozen maps are only partially immutable. The keys associated with a frozen map are determined
//! at creation time and cannot change, but the values can be updated at will if you have a
//! mutable reference to the map. Frozen sets however are completely immutable and so never
//! change after creation.
//!
//! See [BENCHMARKS.md](https://github.com/geeknoid/frozen-collections/blob/main/BENCHMARKS.md) for
//! current benchmark numbers.
//!
//! # Handling Compile-Time Data
//!
//! If you know the keys and values that will be in your collection at compile time, you can use
//! one of eight macros to create frozen collections: [`fz_hash_map!`], [`fz_ordered_map!`],
//! [`fz_scalar_map!`], [`fz_string_map!`], [`fz_hash_set!`], [`fz_ordered_set!`],
//! [`fz_scalar_set!`], or [`fz_string_set!`]. These macros analyze the data you provide
//! and return a custom implementation type that's optimized for the data. All the
//! possible types implement the [`Map`] or [`Set`] traits.
//!
//! The macros exist in a short form and a long form, described below.
//!
//! ## Short Form
//!
//! With the short form, you supply the data that
//! goes into the collection and get in return an initialized collection of an unnamed
//! type. For example:
//!
//! ```rust
//! use frozen_collections::*;
//!
//! let m = fz_string_map!({
//!     "Alice": 1,
//!     "Bob": 2,
//!     "Sandy": 3,
//!     "Tom": 4,
//! });
//! ```
//!
//! At build time, the  macro analyzes the data supplied and determines the best map
//! implementation type to use. As such, the type of `m` is not known to this code. `m` will
//! always implement the [`Map`] trait however, so you can leverage type inference even though
//! you don't know the actual type of `m`:
//!
//! ```rust
//! use frozen_collections::*;
//!
//! fn main() {
//!     let m = fz_string_map!({
//!         "Alice": 1,
//!         "Bob": 2,
//!         "Sandy": 3,
//!         "Tom": 4,
//!     });
//!
//!     more(m);
//! }
//!
//! fn more<M>(m: M)
//! where
//!     M: Map<&'static str, i32>
//! {
//!     assert!(m.contains_key(&"Alice"));
//! }
//! ```
//!
//! ## Long Form
//!
//! The long form lets you provide a type alias name which will be created to
//! correspond to the collection implementation type chosen by the macro invocation.
//! Note that you must use the long form if you want to declare a static frozen collection.
//!
//! ```rust
//! use frozen_collections::*;
//!
//! fz_string_map!(static MAP: MyMapType<&'static str, i32>, {
//!     "Alice": 1,
//!     "Bob": 2,
//!     "Sandy": 3,
//!     "Tom": 4,
//! });
//! ```
//!
//! The above creates a static variable called `MAP` with keys that are strings and values which are
//! integers. As before, you don't know the specific implementation type selected by the macro, but
//! this time you have a type alias (i.e. `MyMapType`) representing that type. You can then use this alias
//! anywhere you'd like to in your code where you'd like to mention the type explicitly.
//!
//! To use the long form for non-static uses, replace `static` with `let`:
//!
//! ```rust
//! use frozen_collections::*;
//!
//! fz_string_map!(let m: MyMapType<&'static str, i32>, {
//!     "Alice": 1,
//!     "Bob": 2,
//!     "Sandy": 3,
//!     "Tom": 4,
//! });
//!
//! more(m);
//!
//! struct S {
//!     m: MyMapType,
//! }
//!  
//! fn more(m: MyMapType) {
//!     assert!(m.contains_key("Alice"));
//! }
//! ```
//!
//! # Using in a Build Script
//!
//! You can use the [`CollectionEmitter`](crate::emit::CollectionEmitter) struct to initialize a frozen collections from a build
//! script and output the results in a file that then gets compiled into your application. Due
//! to the fact build scripts run in a richer environment than procedural macros, the resulting
//! efficiency of collections generated from build scripts can be slightly faster than the ones
//! generated with the macros.
//!
//! # Handling Runtime Data
//!
//! If you don't know the exact keys and values that will be in your collection at compile time,
//! you use the dedicated map and collection types to hold your data: [`FzHashMap`], [`FzOrderedMap`], [`FzScalarMap`],
//! [`FzStringMap`], [`FzHashSet`], [`FzOrderedSet`], [`FzScalarSet`], or [`FzStringSet`]. These
//! types analyze the data you provide at runtime and determine the best strategy to handle your
//! data dynamically.
//!
//! ```rust
//! use frozen_collections::*;
//!
//! let v = vec![
//!     ("Alice", 1),
//!     ("Bob", 2),
//!     ("Sandy", 3),
//!     ("Tom", 4),
//! ];
//!
//! let m = FzStringMap::new(v);
//! ```
//!
//! Note that in general, if possible, it's more efficient to use the macros to create your frozen
//! collection instances.
//!
//! # Traits
//!
//! The maps produced by this crate implement the following traits:
//!
//! - [`Map`]. The primary representation of a map. This trait has [`MapQuery`] and
//!   [`MapIteration`] as super-traits.
//! - [`MapQuery`]. A trait for querying maps. This is an object-safe trait.
//! - [`MapIteration`]. A trait for iterating over maps.
//!
//! The sets produced by this crate implement the following traits:
//!
//! - [`Set`]. The primary representation of a set. This trait has [`SetQuery`],
//!   [`SetIteration`] and [`SetOps`] as super-traits.
//! - [`SetQuery`]. A trait for querying sets. This is an object-safe trait.
//! - [`SetIteration`]. A trait for iterating over sets.
//! - [`SetOps`]. A trait for set operations like union and intersections.
//!
//! # Performance Considerations
//!
//! The analysis performed when creating maps tries to find the best concrete implementation type
//! given the data at hand. The macros perform analysis at build time and generally produce slightly
//! faster results. The collection types meanwhile perform analysis at runtime and the resulting
//! collections are slightly slower.
//!
//! When creating static collections using the macros, the collections produced can often be embedded directly as constant data
//! into the binary of the application, thus requiring no initialization time and no heap space. at
//! This also happens to be the fastest form for these collections. When possible, this happens
//! automatically, you don't need to do anything special to enable this behavior.
//!
//! # Analysis and Optimizations
//!
//! Unlike normal collections, the frozen collections require you to provide all the data for
//! the collection when you create the collection. The data you supply is analyzed which determines
//! which specific underlying implementation strategy to use and how to lay out data internally.
//!
//! The available implementation strategies are:
//!
//! - **Scalar as Hash**. When the keys are of an integer or enum type, this uses the keys themselves
//!   as hash codes, avoiding the overhead of hashing.
//!
//! - **Length as Hash**. When the keys are of a string type, the length of the keys
//!   are used as hash code, avoiding the overhead of hashing.
//!
//! - **Dense Scalar Lookup**. When the keys represent a contiguous range of integer or enum values,
//!   lookups use a simple array access instead of hashing.
//!
//! - **Sparse Scalar Lookup**. When the keys represent a sparse range of integer or enum values,
//!   lookups use a sparse array access instead of hashing.
//!
//! - **Left Hand Substring Hashing**. When the keys are of a string type, this uses sub-slices of
//!   the keys for hashing, reducing the overhead of hashing.
//!
//! - **Right Hand Substring Hashing**. Similar to the Left Hand Substring Hashing from above, but
//!   using right-aligned sub-slices instead.
//!
//! - **Linear Scan**. For very small collections, this avoids hashing completely by scanning through
//!   the entries in linear order.
//!
//! - **Ordered Scan**. For very small collections where the keys implement the [`Ord`] trait,
//!   this avoids hashing completely by scanning through the entries in linear order. Unlike the
//!   Linear Scan strategy, this one can early exit when keys are not found during the scan.
//!
//! - **Classic Hashing**. This is the fallback when none of the previous strategies apply. The
//!   frozen implementations are generally faster than [`std::collections::HashMap`] and
//!   [`std::collections::HashSet`].
//!
//! - **Binary Search**. For relatively small collections where the keys implement the [`Ord`] trait,
//!   classic binary searching is used.
//!
//! - **Eytzinger Search**. For larger collections where the keys implement the [`Ord`] trait,
//!   a cache-friendly Eytzinger search is used.
//!
//! # Cargo Features
//!
//! You can specify the following features when you include the `frozen_collections` crate in your
//! `Cargo.toml` file:
//!
//! - **`macros`**. Enables the macros that create frozen collections at compile time.
//! - **`emit`**. Enables the [`CollectionEmitter`](crate::emit::CollectionEmitter) struct that lets you create frozen collections from a build script.
//! - **`serde`**. Enables serialization and deserialization support for the frozen collections.
//! - **`std`**. Enables small features only available when building with the standard library.
//!
//! All features are enabled by default.

extern crate alloc;

pub use frozen_collections_core::traits::{
    Map, MapIteration, MapQuery, Scalar, Set, SetIteration, SetOps, SetQuery,
};

#[doc(hidden)]
pub use frozen_collections_core::traits::{
    CollectionMagnitude, Hasher, LargeCollection, Len, MediumCollection, SmallCollection,
};

/// Creates an efficient map with a fixed set of hashable keys.
///
/// The concrete type used to implement the map is based on an analysis of the input you
/// provide. Although the types vary, they all implement the [`Map`] trait, so refer to the
/// trait for API documentation.
///
/// # Alternate Choices
///
/// If your keys are integers or enum variants, you should use the [`fz_scalar_map!`] macro instead.
/// If your keys are strings, you should use the [`fz_string_map`] macro instead. Both of these will
/// deliver better performance since they are specifically optimized for those key types.
///
/// # Example
///
/// ```
/// use frozen_collections::*;
///
/// // The key type we use to index into our example maps
/// #[derive(Eq, PartialEq, Hash, Clone, Debug)]
/// struct Key {
///     pub name: &'static str,
///     pub age: i32,
/// }
///
/// // Declare a global static map. This results in a static variable called MY_MAP_0 of type MyMapType0.
/// fz_hash_map!(static MY_MAP_0: MyMapType0<Key, i32>, {
///     Key { name: "Alice", age: 30}: 1,
///     Key { name: "Bob", age: 40}: 2,
/// });
///
/// fn variables() {
///     // Declare a local static map. This results in a local variable called MY_MAP_1 of type MyMapType1.
///     fz_hash_map!(static MY_MAP_1: MyMapType1<Key, i32>, {
///         Key { name: "Alice", age: 30}: 1,
///         Key { name: "Bob", age: 40}: 2,
///     });
///
///     // Declare a local map. This results in a local variable called my_map_2 of type MyMapType2.
///     fz_hash_map!(let my_map_2: MyMapType2<Key, i32>, {
///         Key { name: "Alice", age: 30}: 1,
///         Key { name: "Bob", age: 40}: 2,
///     });
///
///     // Declare a mutable local map. This results in a local variable called my_map_3 of type MyMapType3.
///     fz_hash_map!(let mut my_map_3: MyMapType3<Key, i32>, {
///         Key { name: "Alice", age: 30}: 1,
///         Key { name: "Bob", age: 40}: 2,
///     });
///
///     // Declare a local map. This results in a local variable called my_map_4 of an unknown type.
///     let my_map_4 = fz_hash_map!({
///         Key { name: "Alice", age: 30}: 1,
///         Key { name: "Bob", age: 40}: 2,
///     });
///
///     let v = vec![
///         (Key { name: "Alice", age: 30}, 1),
///         (Key { name: "Bob", age: 40}, 2),
///     ];
///
///     // no matter how the maps are declared, no matter the type selected to implement the map,
///     // they all work the same way and have the same API surface and implement the `Map` trait.
///
///     assert_eq!(
///         Some(&1),
///         MY_MAP_0.get(&Key {
///             name: "Alice",
///             age: 30
///         })
///     );
///
///     assert_eq!(
///         Some(&2),
///         MY_MAP_0.get(&Key {
///             name: "Bob",
///             age: 40
///         })
///     );
///
///     assert_eq!(
///         None,
///         MY_MAP_0.get(&Key {
///             name: "Fred",
///             age: 50
///         })
///     );
/// }
///
/// // How to embed a map into a struct using the map's type.
/// struct MyStruct0 {
///     map: MyMapType0,
/// }
///
/// // how to embed a map into a struct using the `Map` trait.
/// struct MyStruct1<M>
/// where
///     M: Map<Key, i32>,
/// {
///     map: M,
/// }
///
/// // You may need to have specific instances of a map in your process where the keys are the same, but
/// // the values differ. For that case, you can create a static instance with placeholder values.
/// // Then you clone this static instance when needed and set the values that you need in each case.
/// // So you'll have a single set of keys, but different values in each map instance.
/// fn structs() {
///     let mut ms0 = MyStruct0 {
///         map: MY_MAP_0.clone(),
///     };
///
///     let mut ms1 = MyStruct1 {
///         map: MY_MAP_0.clone(),
///     };
///
///     // set a custom value in ms0
///     if let Some(v) = ms0.map.get_mut(&Key {
///         name: "Alice",
///         age: 30,
///     }) {
///         *v = 3;
///     }
///
///     // set a different custom value in ms1
///     if let Some(v) = ms1.map.get_mut(&Key {
///         name: "Alice",
///         age: 30,
///     }) {
///         *v = 4;
///     }
///
///     assert_eq!(
///         Some(&3),
///         ms0.map.get(&Key {
///             name: "Alice",
///             age: 30
///         })
///     );
///
///     assert_eq!(
///         Some(&4),
///         ms1.map.get(&Key {
///             name: "Alice",
///             age: 30
///         })
///     );
/// }
///
/// fn main() {
///     variables();
///     structs();
/// }
/// ```
#[cfg(feature = "macros")]
pub use frozen_collections_macros::fz_hash_map;

/// Creates an efficient set with a fixed set of hashable values.
///
/// The concrete type used to implement the set is based on an analysis of the input you
/// provide. Although the types vary, they all implement the [`Set`] trait, so refer to the
/// trait for API documentation.
///
/// # Alternate Choices
///
/// If your values are integers or enum variants, you should use the [`fz_scalar_set!`] macro instead.
/// If your values are strings, you should use the [`fz_string_set`] macro instead. Both of these will
/// deliver better performance since they are specifically optimized for those value types.
///
/// # Example
///
/// ```
/// use frozen_collections::*;
///
/// // The value type we use to index into our example sets
/// #[derive(Eq, PartialEq, Hash, Clone, Debug)]
/// struct Key {
///     pub name: &'static str,
///     pub age: i32,
/// }
///
/// // Declare a global static set. This results in a static variable called MY_SET_0 of type MySetType0.
/// fz_hash_set!(static MY_SET_0: MySetType0<Key>, {
///     Key { name: "Alice", age: 30},
///     Key { name: "Bob", age: 40},
/// });
///
/// fn variables() {
///     // Declare a local static set. This results in a local variable called MY_SET_1 of type MySetType1.
///     fz_hash_set!(static MY_SET_1: MySetType1<Key>, {
///         Key { name: "Alice", age: 30},
///         Key { name: "Bob", age: 40},
///     });
///
///     // Declare a local set. This results in a local variable called my_set_2 of type MySetType2.
///     fz_hash_set!(let my_set_2: MySetType2<Key>, {
///         Key { name: "Alice", age: 30},
///         Key { name: "Bob", age: 40},
///     });
///
///     // Declare a mutable local set. This results in a local variable called my_set_3 of type MySetType3.
///     fz_hash_set!(let mut my_set_3: MySetType3<Key>, {
///         Key { name: "Alice", age: 30},
///         Key { name: "Bob", age: 40},
///     });
///
///     // Declare a local set. This results in a local variable called my_set_4 of an unknown type.
///     let my_set_4 = fz_hash_set!({
///         Key { name: "Alice", age: 30},
///         Key { name: "Bob", age: 40},
///     });
///
///     // no matter how the sets are declared, no matter the type selected to implement the set,
///     // they all work the same way and have the same API surface and implement the `Set` trait.
///
///     assert!(
///         MY_SET_0.contains(&Key {
///             name: "Alice",
///             age: 30
///         })
///     );
///
///     assert!(
///         MY_SET_0.contains(&Key {
///             name: "Bob",
///             age: 40
///         })
///     );
///
///     assert!(
///         !MY_SET_0.contains(&Key {
///             name: "Fred",
///             age: 50
///         })
///     );
/// }
///
/// fn main() {
///     variables();
/// }
/// ```
#[cfg(feature = "macros")]
pub use frozen_collections_macros::fz_hash_set;

/// Creates an efficient map with a fixed set of ordered keys.
///
/// The concrete type used to implement the map is based on an analysis of the input you
/// provide. Although the types vary, they all implement the [`Map`] trait, so refer to the
/// trait for API documentation.
///
/// # Alternate Choices
///
/// If your keys are integers or enum variants, you should use the [`fz_scalar_map!`] macro instead.
/// If your keys are strings, you should use the [`fz_string_map`] macro instead. Both of these will
/// deliver better performance since they are specifically optimized for those key types.
///
/// # Example
///
/// ```
/// use frozen_collections::*;
///
/// // The key type we use to index into our example maps
/// #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Debug)]
/// struct Key {
///     pub name: &'static str,
///     pub age: i32,
/// }
///
/// // Declare a global static map. This results in a static variable called MY_MAP_0 of type MyMapType0.
/// fz_ordered_map!(static MY_MAP_0: MyMapType0<Key, i32>, {
///     Key { name: "Alice", age: 30}: 1,
///     Key { name: "Bob", age: 40}: 2,
/// });
///
/// fn variables() {
///     // Declare a local static map. This results in a local variable called MY_MAP_1 of type MyMapType1.
///     fz_ordered_map!(static MY_MAP_1: MyMapType1<Key, i32>, {
///         Key { name: "Alice", age: 30}: 1,
///         Key { name: "Bob", age: 40}: 2,
///     });
///
///     // Declare a local map. This results in a local variable called my_map_2 of type MyMapType2.
///     fz_ordered_map!(let my_map_2: MyMapType2<Key, i32>, {
///         Key { name: "Alice", age: 30}: 1,
///         Key { name: "Bob", age: 40}: 2,
///     });
///
///     // Declare a mutable local map. This results in a local variable called my_map_3 of type MyMapType3.
///     fz_ordered_map!(let mut my_map_3: MyMapType3<Key, i32>, {
///         Key { name: "Alice", age: 30}: 1,
///         Key { name: "Bob", age: 40}: 2,
///     });
///
///     // Declare a local map. This results in a local variable called my_map_4 of an unknown type.
///     let my_map_4 = fz_ordered_map!({
///         Key { name: "Alice", age: 30}: 1,
///         Key { name: "Bob", age: 40}: 2,
///     });
///
///     // no matter how the maps are declared, no matter the type selected to implement the map,
///     // they all work the same way and have the same API surface and implement the `Map` trait.
///
///     assert_eq!(
///         Some(&1),
///         MY_MAP_0.get(&Key {
///             name: "Alice",
///             age: 30
///         })
///     );
///
///     assert_eq!(
///         Some(&2),
///         MY_MAP_0.get(&Key {
///             name: "Bob",
///             age: 40
///         })
///     );
///
///     assert_eq!(
///         None,
///         MY_MAP_0.get(&Key {
///             name: "Fred",
///             age: 50
///         })
///     );
/// }
///
/// // How to embed a map into a struct using the map's type.
/// struct MyStruct0 {
///     map: MyMapType0,
/// }
///
/// // how to embed a map into a struct using the `Map` trait.
/// struct MyStruct1<M>
/// where
///     M: Map<Key, i32>,
/// {
///     map: M,
/// }
///
/// // You may need to have specific instances of a map in your process where the keys are the same, but
/// // the values differ. For that case, you can create a static instance with placeholder values.
/// // Then you clone this static instance when needed and set the values that you need in each case.
/// // So you'll have a single set of keys, but different values in each map instance.
/// fn structs() {
///     let mut ms0 = MyStruct0 {
///         map: MY_MAP_0.clone(),
///     };
///
///     let mut ms1 = MyStruct1 {
///         map: MY_MAP_0.clone(),
///     };
///
///     // set a custom value in ms0
///     if let Some(v) = ms0.map.get_mut(&Key {
///         name: "Alice",
///         age: 30,
///     }) {
///         *v = 3;
///     }
///
///     // set a different custom value in ms1
///     if let Some(v) = ms1.map.get_mut(&Key {
///         name: "Alice",
///         age: 30,
///     }) {
///         *v = 4;
///     }
///
///     assert_eq!(
///         Some(&3),
///         ms0.map.get(&Key {
///             name: "Alice",
///             age: 30
///         })
///     );
///
///     assert_eq!(
///         Some(&4),
///         ms1.map.get(&Key {
///             name: "Alice",
///             age: 30
///         })
///     );
/// }
///
/// fn main() {
///     variables();
///     structs();
/// }
/// ```
#[cfg(feature = "macros")]
pub use frozen_collections_macros::fz_ordered_map;

/// Creates an efficient set with a fixed set of ordered values.
///
/// The concrete type used to implement the set is based on an analysis of the input you
/// provide. Although the types vary, they all implement the [`Set`] trait, so refer to the
/// trait for API documentation.
///
/// # Alternate Choices
///
/// If your values are integers or enum variants, you should use the [`fz_scalar_set!`] macro instead.
/// If your values are strings, you should use the [`fz_string_set`] macro instead. Both of these will
/// deliver better performance since they are specifically optimized for those value types.
///
/// # Example
///
/// ```
/// use frozen_collections::*;
///
/// // The value type we use to index into our example sets
/// #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Debug)]
/// struct Key {
///     pub name: &'static str,
///     pub age: i32,
/// }
///
/// // Declare a global static set. This results in a static variable called MY_SET_0 of type MySetType0.
/// fz_ordered_set!(static MY_SET_0: MySetType0<Key>, {
///     Key { name: "Alice", age: 30},
///     Key { name: "Bob", age: 40},
/// });
///
/// fn variables() {
///     // Declare a local static set. This results in a local variable called MY_SET_1 of type MySetType1.
///     fz_ordered_set!(static MY_SET_1: MySetType1<Key>, {
///         Key { name: "Alice", age: 30},
///         Key { name: "Bob", age: 40},
///     });
///
///     // Declare a local set. This results in a local variable called my_set_2 of type MySetType2.
///     fz_ordered_set!(let my_set_2: MySetType2<Key>, {
///         Key { name: "Alice", age: 30},
///         Key { name: "Bob", age: 40},
///     });
///
///     // Declare a mutable local set. This results in a local variable called my_set_3 of type MySetType3.
///     fz_ordered_set!(let mut my_set_3: MySetType3<Key>, {
///         Key { name: "Alice", age: 30},
///         Key { name: "Bob", age: 40},
///     });
///
///     // Declare a local set. This results in a local variable called my_set_4 of an unknown type.
///     let my_set_4 = fz_ordered_set!({
///         Key { name: "Alice", age: 30},
///         Key { name: "Bob", age: 40},
///     });
///
///     // no matter how the sets are declared, no matter the type selected to implement the set,
///     // they all work the same way and have the same API surface and implement the `Set` trait.
///
///     assert!(
///         MY_SET_0.contains(&Key {
///             name: "Alice",
///             age: 30
///         })
///     );
///
///     assert!(
///         MY_SET_0.contains(&Key {
///             name: "Bob",
///             age: 40
///         })
///     );
///
///     assert!(
///         !MY_SET_0.contains(&Key {
///             name: "Fred",
///             age: 50
///         })
///     );
/// }
///
/// fn main() {
///     variables();
/// }
/// ```
#[cfg(feature = "macros")]
pub use frozen_collections_macros::fz_ordered_set;

/// Creates an efficient map with a fixed set of scalar keys.
///
/// The concrete type used to implement the map is based on an analysis of the input you
/// provide. Although the types vary, they all implement the [`Map`] trait, so refer to the
/// trait for API documentation.
///
/// # Example
///
/// ```
/// use frozen_collections::*;
///
/// // The enum type we use to index into our example maps
/// #[derive(Scalar, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Debug)]
/// enum Person {
///     Alice,
///     Bob,
///     Fred,
/// }
///
/// // Declare a global static map. This results in a static variable called MY_MAP_0 of type MyMapType0.
/// fz_scalar_map!(static MY_MAP_0: MyMapType0<Person, i32>, {
///     Person::Alice: 1,
///     Person::Bob: 2,
/// });
///
/// // Scalar maps can also be used with any integer type as key.
/// //
/// // This declares a global static map. This results in a static variable called MY_INT_MAP of type MyIntMapType.
/// fz_scalar_map!(static MY_INT_MAP: MyIntMapType<i32, i32>, {
///     10: 1,
///     20: 2,
/// });
///
/// fn variables() {
///     // Declare a local static map. This results in a local variable called MY_MAP_1 of type MyMapType1.
///     fz_scalar_map!(static MY_MAP_1: MyMapType1<Person, i32>, {
///         Person::Alice: 1,
///         Person::Bob: 2,
///     });
///
///     // Declare a local map. This results in a local variable called my_map_2 of type MyMapType2.
///     fz_scalar_map!(let my_map_2: MyMapType2<Person, i32>, {
///         Person::Alice: 1,
///         Person::Bob: 2,
///     });
///
///     // Declare a mutable local map. This results in a local variable called my_map_3 of type MyMapType3.
///     fz_scalar_map!(let mut my_map_3: MyMapType3<Person, i32>, {
///         Person::Alice: 1,
///         Person::Bob: 2,
///     });
///
///     // Declare a local map. This results in a local variable called my_map_4 of an unknown type.
///     let my_map_4 = fz_scalar_map!({
///         Person::Alice: 1,
///         Person::Bob: 2,
///     });
///
///     // no matter how the maps are declared, no matter the type selected to implement the map,
///     // they all work the same way and have the same API surface and implement the `Map` trait.
///
///     assert_eq!(Some(&1), MY_MAP_0.get(&Person::Alice));
///     assert_eq!(Some(&2), MY_MAP_0.get(&Person::Bob));
///     assert_eq!(None, MY_MAP_0.get(&Person::Fred));
/// }
///
/// // How to embed a map into a struct using the map's type.
/// struct MyStruct0 {
///     map: MyMapType0,
/// }
///
/// // how to embed a map into a struct using the `Map` trait.
/// struct MyStruct1<M>
/// where
///     M: Map<Person, i32>,
/// {
///     map: M,
/// }
///
/// // You may need to have specific instances of a map in your process where the keys are the same, but
/// // the values differ. For that case, you can create a static instance with placeholder values.
/// // Then you clone this static instance when needed and set the values that you need in each case.
/// // So you'll have a single set of keys, but different values in each map instance.
/// fn structs() {
///     let mut ms0 = MyStruct0 {
///         map: MY_MAP_0.clone(),
///     };
///
///     let mut ms1 = MyStruct1 {
///         map: MY_MAP_0.clone(),
///     };
///
///     // set a custom value in ms0
///     if let Some(v) = ms0.map.get_mut(&Person::Alice) {
///         *v = 3;
///     }
///
///     // set a different custom value in ms1
///     if let Some(v) = ms1.map.get_mut(&Person::Alice) {
///         *v = 4;
///     }
///
///     assert_eq!(
///         Some(&3),
///         ms0.map.get(&Person::Alice)
///     );
///
///     assert_eq!(
///         Some(&4),
///         ms1.map.get(&Person::Alice)
///     );
/// }
///
/// fn main() {
///     variables();
///     structs();
/// }
/// ```
#[cfg(feature = "macros")]
pub use frozen_collections_macros::fz_scalar_map;

/// Creates an efficient set with a fixed set of scalar values.
///
/// The concrete type used to implement the set is based on an analysis of the input you
/// provide. Although the types vary, they all implement the [`Set`] trait, so refer to the
/// trait for API documentation.
///
/// # Example
///
/// ```
/// use frozen_collections::*;
///
/// // The enum type we use to index into our example sets
/// #[derive(Scalar, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Debug)]
/// enum Person {
///     Alice,
///     Bob,
///     Fred,
/// }
///
/// // Declare a global static set. This results in a static variable called MY_SET_0 of type MySetType0.
/// fz_scalar_set!(static MY_SET_0: MySetType0<Person>, {
///     Person::Alice,
///     Person::Bob,
/// });
///
/// // Scalar sets can also be used with any integer type as value.
/// //
/// // This declares a global static set. This results in a static variable called MY_INT_SET of type MyIntSetType.
/// fz_scalar_set!(static MY_INT_SET: MyIntMapType<i32>, {
///     10,
///     20,
/// });
///
/// fn variables() {
///     // Declare a local static set. This results in a local variable called MY_SET_1 of type MySetType1.
///     fz_scalar_set!(static MY_SET_1: MySetType1<Person>, {
///         Person::Alice,
///         Person::Bob,
///     });
///
///     // Declare a local set. This results in a local variable called my_set_2 of type MySetType2.
///     fz_scalar_set!(let my_set_2: MySetType2<Person>, {
///         Person::Alice,
///         Person::Bob,
///     });
///
///     // Declare a mutable local set. This results in a local variable called my_set_3 of type MySetType3.
///     fz_scalar_set!(let mut my_set_3: MySetType3<Person>, {
///         Person::Alice,
///         Person::Bob,
///     });
///
///     // Declare a local set. This results in a local variable called my_set_4 of an unknown type.
///     let my_set_4 = fz_scalar_set!({
///         Person::Alice,
///         Person::Bob,
///     });
///
///     // no matter how the sets are declared, no matter the type selected to implement the set,
///     // they all work the same way and have the same API surface and implement the `Set` trait.
///
///     assert!(MY_SET_0.contains(&Person::Alice));
///     assert!(MY_SET_0.contains(&Person::Bob));
///     assert!(!MY_SET_0.contains(&Person::Fred));
/// }
///
/// fn main() {
///     variables();
/// }
/// ```
#[cfg(feature = "macros")]
pub use frozen_collections_macros::fz_scalar_set;

/// Creates an efficient map with a fixed set of string keys.
///
/// The concrete type used to implement the map is based on an analysis of the input you
/// provide. Although the types vary, they all implement the [`Map`] trait, so refer to the
/// trait for API documentation.
///
/// # Example
///
/// ```
/// use frozen_collections::*;
///
/// // Declare a global static map. This results in a static variable called MY_MAP_0 of type MyMapType0.
/// fz_string_map!(static MY_MAP_0: MyMapType0<&'static str, i32>, {
///     "Alice": 1,
///     "Bob": 2,
/// });
///
/// fn variables() {
///     // Declare a local static map. This results in a local variable called MY_MAP_1 of type MyMapType1.
///     fz_string_map!(static MY_MAP_1: MyMapType1<&'static str, i32>, {
///         "Alice": 1,
///         "Bob": 2,
///     });
///
///     // Declare a local map. This results in a local variable called my_map_2 of type MyMapType2.
///     fz_string_map!(let my_map_2: MyMapType2<&'static str, i32>, {
///         "Alice": 1,
///         "Bob": 2,
///     });
///
///     // Declare a mutable local map. This results in a local variable called my_map_3 of type MyMapType3.
///     fz_string_map!(let mut my_map_3: MyMapType3<&'static str, i32>, {
///         "Alice": 1,
///         "Bob": 2,
///     });
///
///     // Declare a local map. This results in a local variable called my_map_4 of an unknown type.
///     let my_map_4 = fz_string_map!({
///         "Alice": 1,
///         "Bob": 2,
///     });
///
///     // no matter how the maps are declared, no matter the type selected to implement the map,
///     // they all work the same way and have the same API surface and implement the `Map` trait.
///
///     assert_eq!(
///         Some(&1),
///         MY_MAP_0.get("Alice")
///     );
///
///     assert_eq!(
///         Some(&2),
///         MY_MAP_0.get("Bob")
///     );
///
///     assert_eq!(
///         None,
///         MY_MAP_0.get("Fred")
///     );
/// }
///
/// // How to embed a map into a struct using the map's type.
/// struct MyStruct0 {
///     map: MyMapType0,
/// }
///
/// // how to embed a map into a struct using the `Map` trait.
/// struct MyStruct1<M>
/// where
///     M: Map<&'static str, i32>,
/// {
///     map: M,
/// }
///
/// // You may need to have specific instances of a map in your process where the keys are the same, but
/// // the values differ. For that case, you can create a static instance with placeholder values.
/// // Then you clone this static instance when needed and set the values that you need in each case.
/// // So you'll have a single set of keys, but different values in each map instance.
/// fn structs() {
///     let mut ms0 = MyStruct0 {
///         map: MY_MAP_0.clone(),
///     };
///
///     let mut ms1 = MyStruct1 {
///         map: MY_MAP_0.clone(),
///     };
///
///     // set a custom value in ms0
///     if let Some(v) = ms0.map.get_mut("Alice") {
///         *v = 3;
///     }
///
///     // set a different custom value in ms1
///     if let Some(v) = ms1.map.get_mut("Alice") {
///         *v = 4;
///     }
///
///     assert_eq!(
///         Some(&3),
///         ms0.map.get(&"Alice")
///     );
///
///     assert_eq!(
///         Some(&4),
///         ms1.map.get("Alice")
///     );
/// }
///
/// fn main() {
///     variables();
///     structs();
/// }
/// ```
#[cfg(feature = "macros")]
pub use frozen_collections_macros::fz_string_map;

/// Creates an efficient set with a fixed set of string values.
///
/// The concrete type used to implement the set is based on an analysis of the input you
/// provide. Although the types vary, they all implement the [`Set`] trait, so refer to the
/// trait for API documentation.
///
/// # Example
///
/// ```
/// use frozen_collections::*;
///
/// // This example shows the various uses of a frozen set whose values are strings.
///
/// // Declare a global static set. This results in a static variable called MY_SET_0 of type MySetType0.
/// fz_string_set!(static MY_SET_0: MySetType0<&'static str>, {
///     "Alice",
///     "Bob",
/// });
///
/// fn variables() {
///     // Declare a local static set. This results in a local variable called MY_SET_1 of type MySetType1.
///     fz_string_set!(static MY_SET_1: MySetType1<&'static str>, {
///         "Alice",
///         "Bob",
///     });
///
///     // Declare a local set. This results in a local variable called my_set_2 of type MySetType2.
///     fz_string_set!(let my_set_2: MySetType2<&'static str>, {
///         "Alice",
///         "Bob",
///     });
///
///     // Declare a mutable local set. This results in a local variable called my_set_3 of type MySetType3.
///     fz_string_set!(let mut my_set_3: MySetType3<&'static str>, {
///         "Alice",
///         "Bob",
///     });
///
///     // Declare a local set. This results in a local variable called my_set_4 of an unknown type.
///     let my_set_4 = fz_string_set!({
///         "Alice",
///         "Bob",
///     });
///
///     // no matter how the sets are declared, no matter the type selected to implement the set,
///     // they all work the same way and have the same API surface and implement the `Set` trait.
///
///     assert!(
///         MY_SET_0.contains("Alice"));
///
///     assert!(
///         MY_SET_0.contains("Bob")
///     );
///
///     assert!(
///         !MY_SET_0.contains("Fred")
///     );
/// }
///
/// fn main() {
///     variables();
/// }
/// ```
#[cfg(feature = "macros")]
pub use frozen_collections_macros::fz_string_set;

/// Implements the `Scalar` trait for an enum.
///
/// Implementing the `Scalar` trait for an enum allows you to use the enum with the [`fz_scalar_map`]
/// and [`fz_scalar_set`] macros. The `Scalar` macro can only be used with enums that only include
/// unit variants without explicit discriminants.
#[cfg(feature = "macros")]
pub use frozen_collections_macros::Scalar;

/// Facilities to generate frozen collections within a Rust build script.
#[cfg(feature = "emit")]
pub mod emit {
    pub use frozen_collections_core::emit::*;
}

pub use frozen_collections_core::fz_maps::*;
pub use frozen_collections_core::fz_sets::*;

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
pub mod hash_tables {
    pub use frozen_collections_core::hash_tables::*;
}

#[doc(hidden)]
pub mod foldhash {
    pub use foldhash::fast::FixedState;
}

pub use frozen_collections_core::DefaultHashBuilder;
