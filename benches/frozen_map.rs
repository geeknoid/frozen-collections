use std::collections::HashMap;
use std::hash::Hash;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use frozen_collections::{frozen_map, FrozenIntMap, FrozenMap, FrozenStringMap, Map};

fn u32_keys(c: &mut Criterion) {
    let mut group = c.benchmark_group("u32_keys");

    let input = frozen_map!(0u32: 1, 2: 3, 4: 5, 6: 7, 8: 9, 100: 101, 120: 121, 140: 141, 160: 161, 1_000_000: 1_000_001);
    let probe = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 1_000_000,
    ];
    /*
        let map: HashMap<_, _, std::hash::RandomState> = input.iter().map(|x| (*x.0, *x.1)).collect();
        group.bench_function("HashMap - classic", |b| {
            b.iter(|| {
                for key in &probe {
                    _ = black_box(map.contains_key(key));
                }
            });
        });
    */
    let map: HashMap<_, _, ahash::RandomState> = input.iter().map(|x| (*x.0, *x.1)).collect();
    group.bench_function("HashMap - ahash", |b| {
        b.iter(|| {
            for key in &probe {
                _ = black_box(map.contains_key(key));
            }
        });
    });

    let map: FrozenIntMap<_, _> = input.iter().map(|x| (*x.0, *x.1)).collect();
    group.bench_function("FrozenIntMap", |b| {
        b.iter(|| {
            for key in &probe {
                _ = black_box(map.contains_key(key));
            }
        });
    });

    let map = input;
    group.bench_function("frozen_map!", |b| {
        b.iter(|| {
            for key in &probe {
                _ = black_box(map.contains_key(key));
            }
        });
    });

    group.finish();
}

fn u32_keys_range(c: &mut Criterion) {
    let mut group = c.benchmark_group("u32_keys_range");

    let input = frozen_map!(0u32: 0, 1: 1, 2:2, 3:3, 4:4, 5:5, 6:6, 7:7, 8:8, 9:9);
    let probe = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 1_000_000,
    ];
    /*
        let map: HashMap<_, _, std::hash::RandomState> = input.iter().map(|x| (*x.0, *x.1)).collect();
        group.bench_function("HashMap - classic", |b| {
            b.iter(|| {
                for key in &probe {
                    _ = black_box(map.contains_key(key));
                }
            });
        });
    */

    let map: HashMap<_, _, ahash::RandomState> = input.iter().map(|x| (*x.0, *x.1)).collect();
    group.bench_function("HashMap - ahash", |b| {
        b.iter(|| {
            for key in &probe {
                _ = black_box(map.contains_key(key));
            }
        });
    });

    let map: FrozenIntMap<_, _> = input.iter().map(|x| (*x.0, *x.1)).collect();
    group.bench_function("FrozenIntMap", |b| {
        b.iter(|| {
            for key in &probe {
                _ = black_box(map.contains_key(key));
            }
        });
    });

    let map = input;
    group.bench_function("frozen_map!", |b| {
        b.iter(|| {
            for key in &probe {
                _ = black_box(map.contains_key(key));
            }
        });
    });

    group.finish();
}

fn string_keys_length(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_keys_length");

    let input = frozen_map!("1": 1, "22":2, "333":3, "4444":4, "55555":5, "666666":6, "7777777":7, "88888888":8, "999999999":9);
    let probe = [
        "0",
        "1",
        "22",
        "333",
        "4444",
        "55555",
        "666666",
        "7777777",
        "D",
        "ABCDEFGHIJKL",
    ];
    /*
        let map: HashMap<_, _, std::hash::RandomState> =
            input.iter().map(|x| (x.0.clone(), *x.1)).collect();
        group.bench_function("HashMap - classic", |b| {
            b.iter(|| {
                for key in probe {
                    _ = black_box(map.contains_key(key));
                }
            });
        });
    */

    let map: HashMap<_, _, ahash::RandomState> =
        input.iter().map(|x| (x.0.clone(), *x.1)).collect();
    group.bench_function("HashMap - ahash", |b| {
        b.iter(|| {
            for key in probe {
                _ = black_box(map.contains_key(key));
            }
        });
    });

    let map: FrozenStringMap<_> = input.iter().map(|x| (x.0.clone(), *x.1)).collect();
    group.bench_function("FrozenStringMap", |b| {
        b.iter(|| {
            for key in probe {
                _ = black_box(map.contains_key(key));
            }
        });
    });

    let map = input;
    group.bench_function("frozen_map!", |b| {
        b.iter(|| {
            for key in probe {
                _ = black_box(map.contains_key(key));
            }
        });
    });

    group.finish();
}

fn string_keys_subslice(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_keys_subslice");

    let input = frozen_map!("ALongPrefixRedd": 1, "ALongPrefixGree":2, "ALongPrefixBlue":3, "ALongPrefixCyan":4);
    let probe = ["ALongPrefixRedd", "ALongPrefixCyan", "Tomato"];

    /*
        let map: HashMap<_, _, std::hash::RandomState> =
            input.iter().map(|x| (x.0.clone(), *x.1)).collect();
        group.bench_function("HashMap - classic", |b| {
            b.iter(|| {
                for key in probe {
                    _ = black_box(map.contains_key(key));
                }
            });
        });
    */
    let map: HashMap<_, _, ahash::RandomState> =
        input.iter().map(|x| (x.0.clone(), *x.1)).collect();
    group.bench_function("HashMap - ahash", |b| {
        b.iter(|| {
            for key in probe {
                _ = black_box(map.contains_key(key));
            }
        });
    });

    let map: FrozenStringMap<_> = input.iter().map(|x| (x.0.clone(), *x.1)).collect();
    group.bench_function("FrozenStringMap", |b| {
        b.iter(|| {
            for key in probe {
                _ = black_box(map.contains_key(key));
            }
        });
    });

    let map = input;
    group.bench_function("frozen_map!", |b| {
        b.iter(|| {
            for key in probe {
                _ = black_box(map.contains_key(key));
            }
        });
    });

    group.finish();
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct MyKey {
    name: String,
    city: String,
}

impl MyKey {
    fn new(name: &str, city: &str) -> Self {
        Self {
            name: name.to_string(),
            city: city.to_string(),
        }
    }
}

fn complex_keys(c: &mut Criterion) {
    let mut group = c.benchmark_group("complex_keys");

    let input = frozen_map!(
        MyKey::new("Alex", "Lisbon"): 10,
        MyKey::new("Brian", "Paris"): 20,
        MyKey::new("Cathy", "New York"): 30,
        MyKey::new("Dylan", "Tokyo"): 40,
        MyKey::new("Ella", "Rio"): 50,
    );

    let probe = [
        MyKey::new("Alex", "Lisbon"),
        MyKey::new("Cathy", "New York"),
        MyKey::new("Ella", "Rio"),
        MyKey::new("Fred", "Fiji"),
    ];

    /*
        let map: HashMap<_, _, std::hash::RandomState> =
            input.iter().map(|x| (x.0.clone(), *x.1)).collect();
        group.bench_function("HashMap - classic", |b| {
            b.iter(|| {
                for key in &probe {
                    _ = black_box(map.contains_key(key));
                }
            });
        });
    */
    let map: HashMap<_, _, ahash::RandomState> =
        input.iter().map(|x| (x.0.clone(), *x.1)).collect();
    group.bench_function("HashMap - ahash", |b| {
        b.iter(|| {
            for key in &probe {
                _ = black_box(map.contains_key(key));
            }
        });
    });

    let map: FrozenMap<_, _> = input.iter().map(|x| (x.0.clone(), *x.1)).collect();
    group.bench_function("FrozenMap", |b| {
        b.iter(|| {
            for key in &probe {
                _ = black_box(map.contains_key(key));
            }
        });
    });

    let map = input;
    group.bench_function("frozen_map!", |b| {
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
    string_keys_length,
    string_keys_subslice,
    u32_keys,
    u32_keys_range,
    complex_keys
);
criterion_main!(benches);
