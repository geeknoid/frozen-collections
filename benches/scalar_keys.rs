extern crate alloc;

use alloc::vec::Vec;
use core::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use frozen_collections::fz_scalar_map;
use frozen_collections::hashers::PassthroughHasher;
use frozen_collections::maps::*;
use hashbrown::HashMap as HashbrownMap;

fn scalar_keys_dense_lookup(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar_keys_dense_lookup");

    let frozen = fz_scalar_map!({
        0: 0,
        1: 1,
        2: 2,
        3: 3,
        4: 4,
        5: 5,
        6: 6,
        7: 7,
        8: 8,
        9: 9,
    });

    let input: Vec<(i32, i32)> = frozen.clone().into_iter().collect();

    // 50% hit rate
    let probe = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
    ];

    let map = HashbrownMap::<_, _, std::hash::RandomState>::from_iter(input.clone());
    group.bench_function("HashbrownMap", |b| {
        b.iter(|| {
            for key in &probe {
                _ = black_box(map.contains_key(key));
            }
        });
    });

    let map = HashbrownMap::<_, _, ahash::RandomState>::from_iter(input.clone());
    group.bench_function("HashbrownMap(ahash)", |b| {
        b.iter(|| {
            for key in &probe {
                _ = black_box(map.contains_key(key));
            }
        });
    });

    let map = DenseScalarLookupMap::new(input).unwrap();
    group.bench_function("DenseScalarLookupMap", |b| {
        b.iter(|| {
            for key in &probe {
                _ = black_box(map.contains_key(key));
            }
        });
    });

    let map = frozen;
    group.bench_function("InlineDenseScalarLookupMap", |b| {
        b.iter(|| {
            for key in &probe {
                _ = black_box(map.contains_key(key));
            }
        });
    });

    group.finish();
}

fn scalar_keys_sparse_lookup(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar_keys_sparse_lookup");

    let frozen = fz_scalar_map!({
        0: 0,
        1: 1,
        2: 2,
        3: 3,
        4: 4,
        5: 5,
        6: 6,
        7: 7,
        8: 8,
        9: 9,
        19: 19,
        20: 20
    });

    let input: Vec<(i32, i32)> = frozen.clone().into_iter().collect();

    // 50% hit rate, 25% miss within range, 25% miss outside range
    let probe = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 19, 20, 10, 11, 12, 13, 14, 15, 21, 22, 23, 24, 25, 26,
    ];

    let map = HashbrownMap::<_, _, std::hash::RandomState>::from_iter(input.clone());
    group.bench_function("HashbrownMap", |b| {
        b.iter(|| {
            for key in &probe {
                _ = black_box(map.contains_key(key));
            }
        });
    });

    let map = HashbrownMap::<_, _, ahash::RandomState>::from_iter(input.clone());
    group.bench_function("HashbrownMap(ahash)", |b| {
        b.iter(|| {
            for key in &probe {
                _ = black_box(map.contains_key(key));
            }
        });
    });

    let map = SparseScalarLookupMap::<_, _>::new(input);
    group.bench_function("SparseScalarLookupMap", |b| {
        b.iter(|| {
            for key in &probe {
                _ = black_box(map.contains_key(key));
            }
        });
    });

    let map = frozen;
    group.bench_function("InlineSparseScalarLookupMap", |b| {
        b.iter(|| {
            for key in &probe {
                _ = black_box(map.contains_key(key));
            }
        });
    });

    group.finish();
}

fn scalar_keys_hash(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar_keys_hash");

    let frozen = fz_scalar_map!({
        0: 0,
        11: 1,
        222: 2,
        3333: 3,
        44444: 4,
        555555: 5,
        6666666: 6,
        77777777: 7,
        888888888: 8,
        999999999: 9,
        19: 19,
        20: 20
    });

    let input: Vec<(i32, i32)> = frozen.clone().into_iter().collect();

    // 50% hit rate, 25% miss within range, 25% miss outside range
    let probe = [
        0,
        11,
        222,
        3333,
        44444,
        555_555,
        6_666_666,
        77_777_777,
        888_888_888,
        999_999_999,
        19,
        20,
        10,
        11,
        12,
        13,
        14,
        15,
        21,
        22,
        23,
        24,
        25,
        26,
    ];

    let map = HashbrownMap::<_, _, std::hash::RandomState>::from_iter(input.clone());
    group.bench_function("HashbrownMap", |b| {
        b.iter(|| {
            for key in &probe {
                _ = black_box(map.contains_key(key));
            }
        });
    });

    let map = HashbrownMap::<_, _, ahash::RandomState>::from_iter(input.clone());
    group.bench_function("HashbrownMap(ahash)", |b| {
        b.iter(|| {
            for key in &probe {
                _ = black_box(map.contains_key(key));
            }
        });
    });

    let map = HashMap::<_, _, _>::new(input, PassthroughHasher::default());
    group.bench_function("HashMap", |b| {
        b.iter(|| {
            for key in &probe {
                _ = black_box(map.contains_key(key));
            }
        });
    });

    let map = frozen;
    group.bench_function("InlineHashMap", |b| {
        b.iter(|| {
            for key in &probe {
                _ = black_box(map.contains_key(key));
            }
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    scalar_keys_hash,
    scalar_keys_dense_lookup,
    scalar_keys_sparse_lookup,
);

criterion_main!(benches);
