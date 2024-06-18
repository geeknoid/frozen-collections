//! Random # utilities for frozen collections.

use const_random::const_random;

/// Pick four random seeds at compile time.
#[must_use]
#[mutants::skip]
pub const fn pick_compile_time_random_seeds() -> (u64, u64, u64, u64) {
    (
        const_random!(u64),
        const_random!(u64),
        const_random!(u64),
        const_random!(u64),
    )
}
