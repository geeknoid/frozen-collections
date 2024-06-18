//! Logic to analyze collection input data to assess the best implementation choices.

pub use duplicate_key_detector::*;
pub use hash_code_analyzer::*;
pub use int_key_analyzer::*;
pub use slice_key_analyzer::*;

mod duplicate_key_detector;
mod hash_code_analyzer;
mod int_key_analyzer;
mod slice_key_analyzer;
