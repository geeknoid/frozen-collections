use core::hint::black_box;
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use frozen_collections::{FzOrderedSet, SetQuery, fz_ordered_set};

include!(concat!(env!("OUT_DIR"), "/ordered.rs"));

criterion_group!(benches, ordered);
criterion_main!(benches);
