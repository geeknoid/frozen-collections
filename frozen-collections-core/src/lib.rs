//! Implementation crate for the frozen collections.
//!
//! <div class="warning">
//! This crate is an implementation detail of the `frozen_collections` crate.
//! This crate's API is therefore not stable and may change at any time. Please do not
//! use this crate directly, and instead use the public API provided by the
//! `frozen_collections` crate.
//! </div>

#![cfg_attr(not(any(test, feature = "std")), no_std)]

extern crate alloc;
extern crate core;

mod analyzers;
pub mod facade_maps;
pub mod facade_sets;
pub mod hash_tables;
pub mod hashers;
pub mod inline_maps;
pub mod inline_sets;
pub mod macros;
pub mod maps;
pub mod sets;
pub mod traits;
mod utils;
