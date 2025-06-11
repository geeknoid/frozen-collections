#![expect(missing_docs, reason = "Benchmark")]

use core::hash::Hash;
use core::hint::black_box;
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use frozen_collections::{FzHashSet, fz_hash_set};

include!(concat!(env!("OUT_DIR"), "/hashed.rs"));

criterion_group!(benches, hashed);
criterion_main!(benches);
