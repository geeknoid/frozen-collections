use std::collections::HashSet;
use std::hash::Hash;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use frozen_collections::{frozen_set, FrozenIntSet, FrozenSet, FrozenStringSet, Set};

fn u32_values(c: &mut Criterion) {
    let mut group = c.benchmark_group("u32_values");

    let input = frozen_set!(0u32, 2, 4, 6, 8, 100, 120, 140, 160, 1_000_000);
    let probe = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 1_000_000,
    ];
    /*
        let set: HashSet<_, std::hash::RandomState> = input.iter().copied().collect();
        group.bench_function("HashSet - classic", |b| {
            b.iter(|| {
                for key in &probe {
                    _ = black_box(set.contains(key));
                }
            });
        });
    */
    let set: HashSet<_, ahash::RandomState> = input.iter().copied().collect();
    group.bench_function("HashSet - ahash", |b| {
        b.iter(|| {
            for key in &probe {
                _ = black_box(set.contains(key));
            }
        });
    });

    let set: FrozenIntSet<_> = input.iter().copied().collect();
    group.bench_function("FrozenIntSet", |b| {
        b.iter(|| {
            for key in &probe {
                _ = black_box(set.contains(key));
            }
        });
    });

    let set = input;
    group.bench_function("frozen_set!", |b| {
        b.iter(|| {
            for key in &probe {
                _ = black_box(set.contains(key));
            }
        });
    });

    group.finish();
}

fn u32_values_range(c: &mut Criterion) {
    let mut group = c.benchmark_group("u32_values_range");

    let input = frozen_set!(0u32, 1, 2, 3, 4, 5, 6, 7, 8, 9);
    let probe = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 1_000_000,
    ];
    /*
        let set: HashSet<_, std::hash::RandomState> = input.iter().copied().collect();
        group.bench_function("HashSet - classic", |b| {
            b.iter(|| {
                for key in &probe {
                    _ = black_box(set.contains(key));
                }
            });
        });
    */
    let set: HashSet<_, ahash::RandomState> = input.iter().copied().collect();
    group.bench_function("HashSet - ahash", |b| {
        b.iter(|| {
            for key in &probe {
                _ = black_box(set.contains(key));
            }
        });
    });

    let set: FrozenIntSet<_> = input.iter().copied().collect();
    group.bench_function("FrozenIntSet", |b| {
        b.iter(|| {
            for key in &probe {
                _ = black_box(set.contains(key));
            }
        });
    });

    let set = input;
    group.bench_function("frozen_set!", |b| {
        b.iter(|| {
            for key in &probe {
                _ = black_box(set.contains(key));
            }
        });
    });

    group.finish();
}

fn string_values_length(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_values_length");

    let input = frozen_set!(
        "1",
        "22",
        "333",
        "4444",
        "55555",
        "666666",
        "7777777",
        "88888888",
        "999999999"
    );
    let probe = [
        "0",
        "1",
        "22",
        "333",
        "4444",
        "A",
        "B",
        "C",
        "D",
        "ABCDEFGHIJKL",
    ];
    /*
        let set: HashSet<_, std::hash::RandomState> = input.iter().cloned().collect();
        group.bench_function("HashSet - classic", |b| {
            b.iter(|| {
                for key in probe {
                    _ = black_box(set.contains(key));
                }
            });
        });
    */
    let set: HashSet<_, ahash::RandomState> = input.iter().cloned().collect();
    group.bench_function("HashSet - ahash", |b| {
        b.iter(|| {
            for key in probe {
                _ = black_box(set.contains(key));
            }
        });
    });

    let set = FrozenStringSet::new(input.iter().cloned().collect()).unwrap();
    group.bench_function("FrozenStringSet", |b| {
        b.iter(|| {
            for key in probe {
                _ = black_box(set.contains(key));
            }
        });
    });

    let set = input;
    group.bench_function("frozen_set!", |b| {
        b.iter(|| {
            for key in probe {
                _ = black_box(set.contains(key));
            }
        });
    });

    group.finish();
}

fn string_values_subslice(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_values_subslice");

    let input = frozen_set!(
        "ALongPrefixRedd",
        "ALongPrefixGree",
        "ALongPrefixBlue",
        "ALongPrefixCyan"
    );
    let probe = [
        "ALongPrefixRedd",
        "ALongPrefixCyan",
        "Tomato",
        "Potato",
        "Carrot",
    ];
    /*
        let set: HashSet<_, std::hash::RandomState> = input.iter().cloned().collect();
        group.bench_function("HashSet - classic", |b| {
            b.iter(|| {
                for key in probe {
                    _ = black_box(set.contains(key));
                }
            });
        });
    */
    let set: HashSet<_, ahash::RandomState> = input.iter().cloned().collect();
    group.bench_function("HashSet - ahash", |b| {
        b.iter(|| {
            for key in probe {
                _ = black_box(set.contains(key));
            }
        });
    });

    let set = FrozenStringSet::new(input.iter().cloned().collect()).unwrap();
    group.bench_function("FrozenStringSet", |b| {
        b.iter(|| {
            for key in probe {
                _ = black_box(set.contains(key));
            }
        });
    });

    let set = input;
    group.bench_function("frozen_set!", |b| {
        b.iter(|| {
            for key in probe {
                _ = black_box(set.contains(key));
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

fn complex_values(c: &mut Criterion) {
    let mut group = c.benchmark_group("complex_values");

    let input = frozen_set!(
        MyKey::new("Alex", "Lisbon"),
        MyKey::new("Brian", "Paris"),
        MyKey::new("Cathy", "New York"),
        MyKey::new("Dylan", "Tokyo"),
        MyKey::new("Ella", "Rio"),
    );

    let probe = [
        MyKey::new("Alex", "Lisbon"),
        MyKey::new("Ella", "Rio"),
        MyKey::new("Fred", "Fiji"),
        MyKey::new("Bruno", "Buenos Ares"),
    ];
    /*
        let set: HashSet<_, std::hash::RandomState> = input.iter().cloned().collect();
        group.bench_function("HashSet - classic", |b| {
            b.iter(|| {
                for key in &probe {
                    _ = black_box(set.contains(key));
                }
            });
        });
    */
    let set: HashSet<_, ahash::RandomState> = input.iter().cloned().collect();
    group.bench_function("HashSet - ahash", |b| {
        b.iter(|| {
            for key in &probe {
                _ = black_box(set.contains(key));
            }
        });
    });

    let set = FrozenSet::new(input.iter().cloned().collect()).unwrap();
    group.bench_function("FrozenSet", |b| {
        b.iter(|| {
            for key in &probe {
                _ = black_box(set.contains(key));
            }
        });
    });

    let set = input;
    group.bench_function("frozen_set!", |b| {
        b.iter(|| {
            for key in &probe {
                _ = black_box(set.contains(key));
            }
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    string_values_length,
    string_values_subslice,
    u32_values,
    u32_values_range,
    complex_values
);
criterion_main!(benches);
