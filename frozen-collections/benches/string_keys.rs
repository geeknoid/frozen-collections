extern crate alloc;

use alloc::vec::Vec;
use core::hint::black_box;
use core::ops::Add;
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use frozen_collections::{FzStringSet, SetQuery, fz_string_set};

include!(concat!(env!("OUT_DIR"), "/random_string.rs"));
include!(concat!(env!("OUT_DIR"), "/prefixed_string.rs"));

criterion_group!(benches, random_string, prefixed_string,);
criterion_main!(benches);
