extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use core::hash::Hash;
use core::hint::black_box;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use frozen_collections::{fz_hash_set, FzHashSet, SetQuery};

include!(concat!(env!("OUT_DIR"), "/hashed.rs"));

criterion_group!(benches, hashed);
criterion_main!(benches);
