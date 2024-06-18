use core::hint::black_box;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use frozen_collections::{fz_ordered_set, SetQuery};

include!(concat!(env!("OUT_DIR"), "/ordered.rs"));

criterion_group!(benches, ordered);
criterion_main!(benches);
