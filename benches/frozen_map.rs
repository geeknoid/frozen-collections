use std::collections::HashMap;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use frozen_collections::{frozen_map, FrozenMap};

fn u32_keys(c: &mut Criterion) {
    let mut group = c.benchmark_group("u32_keys");

    let map = HashMap::from([(0u32, 1), (2, 3), (4, 5), (6, 7), (8, 9)]);
    group.bench_function("HashMap", |b| {
        b.iter(|| {
            _ = black_box(map.get(&4));
            _ = black_box(map.get(&10));
        });
    });

    group.bench_function("FrozenMap", |b| {
        let map = FrozenMap::try_from([(0u32, 1), (2, 3), (4, 5), (6, 7), (8, 9)]).unwrap();
        b.iter(|| {
            _ = black_box(map.get(&4));
            _ = black_box(map.get(&10));
        });
    });

    group.bench_function("frozen_map!", |b| {
        let map = frozen_map!(u32, 0: 1, 2: 3, 4: 5, 6: 7, 8: 9);

        b.iter(|| {
            _ = black_box(map.get(&4));
            _ = black_box(map.get(&10));
        });
    });

    group.finish();
}

fn u32_keys_range(c: &mut Criterion) {
    let mut group = c.benchmark_group("u32_keys_range");

    let map = HashMap::from([(0u32, 0), (1, 1), (2, 2), (3, 3), (4, 4)]);
    group.bench_function("HashMap", |b| {
        b.iter(|| {
            _ = black_box(map.get(&4));
            _ = black_box(map.get(&10));
        });
    });

    group.bench_function("FrozenMap", |b| {
        let map = FrozenMap::try_from([(0u32, 0), (1, 1), (2, 2), (3, 3), (4, 4)]).unwrap();
        b.iter(|| {
            _ = black_box(map.get(&4));
            _ = black_box(map.get(&10));
        });
    });

    group.bench_function("frozen_map!", |b| {
        let map = frozen_map!(u32, 0: 0, 1: 1, 2: 2, 3: 3, 4: 4);
        b.iter(|| {
            _ = black_box(map.get(&4));
            _ = black_box(map.get(&10));
        });
    });

    group.finish();
}

fn i32_keys(c: &mut Criterion) {
    let mut group = c.benchmark_group("i32_keys");

    let map = HashMap::from([(0, 1), (2, 3), (4, 5), (6, 7), (8, 9)]);
    group.bench_function("HashMap", |b| {
        b.iter(|| {
            _ = black_box(map.get(&4));
            _ = black_box(map.get(&10));
        });
    });

    group.bench_function("FrozenMap", |b| {
        let map = FrozenMap::try_from([(0, 1), (2, 3), (4, 5), (6, 7), (8, 9)]).unwrap();
        b.iter(|| {
            _ = black_box(map.get(&4));
            _ = black_box(map.get(&10));
        });
    });

    group.bench_function("frozen_map!", |b| {
        let map = frozen_map!(i32, 0: 1, 2: 3, 4: 5, 6: 7, 8: 9);
        b.iter(|| {
            _ = black_box(map.get(&4));
            _ = black_box(map.get(&10));
        });
    });

    group.finish();
}

fn string_keys_length(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_keys_length");
    let kvs = [
        ("Red".to_string(), 1),
        ("Green".to_string(), 2),
        ("Blue".to_string(), 3),
        ("Cyan".to_string(), 4),
        ("Magenta".to_string(), 5),
        ("Purple".to_string(), 6),
    ];

    let blue = "Blue".to_string();
    let black = "Black".to_string();

    let map = HashMap::from(kvs.clone());
    group.bench_function("HashMap", |b| {
        b.iter(|| {
            _ = black_box(map.get(&blue));
            _ = black_box(map.get(&black));
        });
    });

    group.bench_function("FrozenMap", |b| {
        let map = FrozenMap::try_from(kvs.clone()).unwrap();
        b.iter(|| {
            _ = black_box(map.get(&blue));
            _ = black_box(map.get(&black));
        });
    });

    group.bench_function("frozen_map!", |b| {
        let map = frozen_map!(
            &str,
            "Red": 1,
            "Green": 2,
            "Blue": 3,
            "Cyan": 4,
            "Magenta": 5,
            "Purple": 6);

        b.iter(|| {
            _ = black_box(map.get(&blue));
            _ = black_box(map.get(&black));
        });
    });

    group.finish();
}

fn string_keys_subslice(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_keys_subslice");
    let kvs = [
        ("abcdefghi0".to_string(), 1),
        ("abcdefghi1".to_string(), 2),
        ("abcdefghi2".to_string(), 3),
        ("abcdefghi3".to_string(), 4),
        ("abcdefghi4".to_string(), 5),
        ("abcdefghi5".to_string(), 6),
    ];

    let blue = "Blue".to_string();
    let black = "Black".to_string();

    let map = HashMap::from(kvs.clone());
    group.bench_function("HashMap", |b| {
        b.iter(|| {
            _ = black_box(map.get(&blue));
            _ = black_box(map.get(&black));
        });
    });

    group.bench_function("FrozenMap", |b| {
        let map = FrozenMap::try_from(kvs.clone()).unwrap();
        b.iter(|| {
            _ = black_box(map.get(&blue));
            _ = black_box(map.get(&black));
        });
    });

    group.bench_function("frozen_map!", |b| {
        let map = frozen_map!(
            &str,
            "abcdefghi0": 1,
            "abcdefghi1": 2,
            "abcdefghi2": 3,
            "abcdefghi3": 4,
            "abcdefghi4": 5,
            "abcdefghi5": 6,
        );
        b.iter(|| {
            _ = black_box(map.get(&blue));
            _ = black_box(map.get(&black));
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    string_keys_length,
    string_keys_subslice,
    u32_keys,
    u32_keys_range,
    i32_keys
);
criterion_main!(benches);
