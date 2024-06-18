//! Implementation crate for the frozen collections.
//!
//! # Compatibility Note
//!
//! This crate is not intended to be used directly. It is an implementation
//! detail of the `frozen-collections` crate. The API of this crate is therefore
//! not stable and may change at any time. If you need to use the functionality
//! of this crate, please use the `frozen-collections` crate instead which has
//! a stable API.

#![cfg_attr(not(any(test, feature = "std")), no_std)]

extern crate alloc;
extern crate core;

pub mod analyzers;
pub mod hashers;
pub mod inline_maps;
pub mod inline_sets;
pub mod maps;
pub mod sets;
pub mod traits;
pub mod utils;

#[cfg(feature = "macros")]
pub mod macros;
