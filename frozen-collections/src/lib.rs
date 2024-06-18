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
//! As part of initializing a frozen, an analysis is performed over the data that the collection
//! will hood, to determine the best layout and algorithm for the specific case.
//! This analysis can take some time. But the value in spending this time up front
//! is that the collections provide faster read-time performance.
//!
//! # Macros and Structs
//!
//! Frozen collections can be created in one of two ways:
//!
//! - **via Macros**. When you know the data to load into the collection at build time, you can use the
//!   [`fz_hash_set!`], [`fz_hash_map!`], or [`fz_string_map!`] macros. Analysis of the input
//!   data is done at build time, so there isn't any analysis cost spent when creating the
//!   collections. Additionally, using the macros results in faster runtime code.
//!
//! * **via Facade Types**. When you don't know the data to load into the collection at
//!   build time, you use the [`FzHashSet`], [`FzHashMap`], [`FzSequenceSet`], [`FzSequenceMap`],
//!   [`FzStringSet`] and [`FzStringMap`] types. Analysis is performed at runtime when
//!   the collections are created, which can take a while if there are millions of
//!   entries.
//!
//! Using the macros results in faster runtime performance, so they are the preferred choice if
//! possible. The facade type dispatch at runtime to
//! different specialized implementation types. When you use the macros instead, the selection
//! of implementation type is done at build time, and thus the cost of the runtime dispatch is
//! completely eliminated.
//!
//! The [`fz_string_map!`] macro lets you create static maps that are stored directly in the
//! binary image, thus eliminating the need to replicate the collection in the heap on startup.
//! The lookup performance for these maps tends to be the fastest because all the data is inlined
//! and indirections have been eliminated.
//!
//! # Analysis and Optimizations
//!
//! Unlike normal collections, the frozen collections require you to provide all the data for
//! the collection when you create the collection. The data you supply is analyzed which determines
//! what specific underlying implementation strategy to use and how to lay out data in the hash tables
//! (assuming the implementation uses hash tables at all)
//!
//! The available implementation strategies are:
//!
//! - **Integer as Hash**. When the keys are of an integer type, this uses the keys themselves
//!   as hash codes, avoiding the overhead of hashing.
//!
//! - **Dense Integer Lookup**. When the keys represent a contiguous range of integer values, this eliminates
//!   hashing completely and uses direct indexing instead.
//!
//! - **Sparse Integer Lookup**. When the keys represent a sparse range of integer values, this eliminates
//!   hashing completely and uses a simple table lookup instead.
//!
//! - **Length as Hash**. When the keys are of a slice type, this uses the length of the slices
//!   as hash codes, avoiding the overhead of hashing.
//!
//! - **Left Hand Hashing**. When the keys are of a slice type, this uses sub-slices of the keys
//!   for hashing, reducing the overhead of hashing.
//!
//! - **Right Hand Hashing**. Similar to the Right Hand Hashing from above, but using right-aligned
//!   sub-slices instead.
//!
//! - **Linear Scan**. For very small collections, this avoids hashing completely by scanning through the
//!   keys in linear order.
//!
//! - **Ordered Linear Scan**. For very small collections where the keys implement the [`Ord`] trait,
//!   this avoids hashing completely by scanning through the keys in linear order.
//!
//! - **Classic Hashing**. This is the fallback when none of the previous strategies apply.
//!
//! # Optimizations
//!
//! The different API surfaces offer differ features and different optimization levels.
//!
//! | API                 | String Keys | Int Keys | Enum Keys | Non Zero Keys | Complex Keys |
//! |---------------------|-------------|----------|-----------|---------------|--------------|
//! | `static_frozen_xxx` | Max         | Max      | N/A       | N/A           | N/A          |
//! | `frozen_xxx`        | Max         | Max      | Min       | Min           | Min          |
//! | `FrozenXxx`         | Min         | Min      | Min       | Min           | Min          |
//! | `FrozenSequenceXxx` | N/A         | Mid      | Mid       | Mid           | N/A          |
//! | `FrozenStringXxx`   | Mid         | N/A      | N/A       | N/A           | N/A          |
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
    CollectionMagnitude, Hasher, Len, Map, MapIterator, Sequence, Set, SetIterator,
};

#[doc(hidden)]
pub use frozen_collections_core::traits::{LargeCollection, MediumCollection, SmallCollection};

#[cfg(feature = "facades")]
pub use crate::facades::*;

#[cfg(feature = "macros")]
pub use frozen_collections_macros::*;

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
