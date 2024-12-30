//! Implementation logic for frozen collection macros.

pub use derive_scalar_macro::derive_scalar_macro;
pub use macro_api::*;

mod derive_scalar_macro;
mod macro_api;
mod parsing;
mod processor;
