//! Implementation logic for frozen collection macros.

pub use derive_scalar::derive_scalar_macro;
pub use proc_macros::*;

mod derive_scalar;
mod generator;
mod hash_table;
mod parsing;
mod proc_macros;
