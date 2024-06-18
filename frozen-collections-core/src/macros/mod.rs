//! Implementation logic for frozen collection macros.

pub use derive_sequence::derive_sequence_macro;
pub use proc_macros::*;

mod derive_sequence;
mod generator;
mod hash_table;
mod parsing;
mod proc_macros;
