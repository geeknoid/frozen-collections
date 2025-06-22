#![expect(missing_docs, reason = "Benchmark")]

use core::hint::black_box;
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use frozen_collections::{FzScalarSet, fz_scalar_set};

include!(concat!(env!("OUT_DIR"), "/dense_scalar.rs"));
include!(concat!(env!("OUT_DIR"), "/sparse_scalar.rs"));
include!(concat!(env!("OUT_DIR"), "/random_scalar.rs"));

criterion_group!(benches, dense_scalar, sparse_scalar, random_scalar,);
criterion_main!(benches);
