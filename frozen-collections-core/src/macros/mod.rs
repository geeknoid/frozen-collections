//! Implementation logic for frozen collection macros.

pub use derive_scalar_macro::derive_scalar_macro;
pub use map_macros::*;
pub use set_macros::*;

mod derive_scalar_macro;
mod generator;
mod hash_table;
mod map_macros;
mod parsing;
mod set_macros;
