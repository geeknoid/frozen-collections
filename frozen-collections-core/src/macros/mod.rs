//! Implementation logic for the frozen collection macros.

pub use frozen_map::frozen_map_macro;
pub use frozen_set::frozen_set_macro;

mod frozen_map;
mod frozen_set;
