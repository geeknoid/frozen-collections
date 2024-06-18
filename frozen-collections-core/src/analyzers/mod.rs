//! Logic to analyze collection input data to assess the best implementation choices.

pub use hash_code_analyzer::*;
pub use scalar_key_analyzer::*;
pub use slice_key_analyzer::*;

mod hash_code_analyzer;
mod scalar_key_analyzer;
mod slice_key_analyzer;
