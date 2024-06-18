use std::collections::HashMap as StdHashMap;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use frozen_collections::hashers::PassthroughHasher;
use frozen_collections::maps::*;
use frozen_collections::{fz_scalar_map, SmallCollection};

fn int_keys_dense_lookup(c: &mut Criterion) {
    let mut group = c.benchmark_group("int_keys_dense_lookup");

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

    let map = StdHashMap::<_, _, std::hash::RandomState>::from_iter(input.clone());
    group.bench_function("StdHashMap", |b| {
        b.iter(|| {
            for key in &probe {
                _ = black_box(map.contains_key(key));
            }
        });
    });

    let map = StdHashMap::<_, _, ahash::RandomState>::from_iter(input.clone());
    group.bench_function("StdHashMap(ahash)", |b| {
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

fn int_keys_sparse_lookup(c: &mut Criterion) {
    let mut group = c.benchmark_group("int_keys_sparse_lookup");

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

    let map = StdHashMap::<_, _, std::hash::RandomState>::from_iter(input.clone());
    group.bench_function("StdHashMap", |b| {
        b.iter(|| {
            for key in &probe {
                _ = black_box(map.contains_key(key));
            }
        });
    });

    let map = StdHashMap::<_, _, ahash::RandomState>::from_iter(input.clone());
    group.bench_function("StdHashMap(ahash)", |b| {
        b.iter(|| {
            for key in &probe {
                _ = black_box(map.contains_key(key));
            }
        });
    });

    let map = SparseScalarLookupMap::<_, _, SmallCollection>::new(input).unwrap();
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

fn int_keys(c: &mut Criterion) {
    let mut group = c.benchmark_group("int_keys");

    for size in 3..14 {
        // 50% hit rate
        let input: Vec<_> = (100..100 + size).map(|x| (x * 2, x)).collect();
        let probe: Vec<_> = (100 - size / 2..100 + size + size / 2)
            .map(|x| x * 2)
            .collect();

        let map = StdHashMap::<_, _, std::hash::RandomState>::from_iter(input.clone());
        group.bench_with_input(BenchmarkId::new("StdHashMap", size), &size, |b, _| {
            b.iter(|| {
                for key in &probe {
                    _ = black_box(map.contains_key(key));
                }
            });
        });

        let map = StdHashMap::<_, _, ahash::RandomState>::from_iter(input.clone());
        group.bench_with_input(
            BenchmarkId::new("StdHashMap(ahash)", size),
            &size,
            |b, _| {
                b.iter(|| {
                    for key in &probe {
                        _ = black_box(map.contains_key(key));
                    }
                });
            },
        );

        let map =
            HashMap::<_, _, SmallCollection, _>::new(input.clone(), PassthroughHasher::default())
                .unwrap();
        group.bench_with_input(BenchmarkId::new("HashMap", size), &size, |b, _| {
            b.iter(|| {
                for key in &probe {
                    _ = black_box(map.contains_key(key));
                }
            });
        });

        let map = ScanMap::new(input.clone());
        group.bench_with_input(BenchmarkId::new("ScanMap", size), &size, |b, _| {
            b.iter(|| {
                for key in &probe {
                    _ = black_box(map.contains_key(key));
                }
            });
        });

        let map = OrderedScanMap::new(input.clone());
        group.bench_with_input(BenchmarkId::new("OrderedScanMap", size), &size, |b, _| {
            b.iter(|| {
                for key in &probe {
                    _ = black_box(map.contains_key(key));
                }
            });
        });

        let map = BinarySearchMap::new(input.clone());
        group.bench_with_input(BenchmarkId::new("BinarySearchMap", size), &size, |b, _| {
            b.iter(|| {
                for key in &probe {
                    _ = black_box(map.contains_key(key));
                }
            });
        });
    }

    group.finish();
}
criterion_group!(
    benches,
    int_keys,
    int_keys_dense_lookup,
    int_keys_sparse_lookup,
);
criterion_main!(benches);
