#![expect(missing_docs, reason = "Benchmark")]
#![allow(clippy::allow_attributes, reason = "Generated code")]
#![allow(clippy::unreadable_literal, reason = "Generated code")]
#![allow(clippy::items_after_statements, reason = "Generated code")]
#![allow(clippy::explicit_auto_deref, reason = "Generated code")]
#![allow(
    clippy::redundant_closure_for_method_calls,
    reason = "Generated code"
)]
#![allow(unused_results, reason = "Generated code")]
#![allow(unused_qualifications, reason = "Generated code")]

use core::hash::Hash;
use core::hint::black_box;
use frozen_collections::{FzHashSet, fz_hash_set};
use gungraun::{Callgrind, LibraryBenchmarkConfig, library_benchmark, library_benchmark_group, main};

include!(concat!(env!("OUT_DIR"), "/gungraun_hashed.rs"));

main!(
    config = LibraryBenchmarkConfig::default()
        .tool(Callgrind::with_args(["--branch-sim=yes"]));
    library_benchmark_groups = hashed_group
);
