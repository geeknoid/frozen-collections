use std::collections::{BTreeMap, HashMap as StdHashMap};
use std::hash::Hash;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use frozen_collections::hashers::BridgeHasher;
use frozen_collections::maps::*;
use frozen_collections::SmallCollection;

#[derive(Hash, Eq, PartialEq, Ord, PartialOrd, Clone)]
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

fn misc_keys(c: &mut Criterion) {
    let mut group = c.benchmark_group("misc_keys");

    for size in 3..20 {
        let mut input = vec![
            (MyKey::new("Alex", "Lisbon"), 10),
            (MyKey::new("Brian", "Paris"), 20),
            (MyKey::new("Cathy", "New York"), 30),
            (MyKey::new("Dylan", "Tokyo"), 40),
            (MyKey::new("Ella", "Rio"), 50),
            (MyKey::new("Fred", "Oslo"), 60),
            (MyKey::new("Gina", "Montreal"), 70),
            (MyKey::new("Helena", "Quebec"), 80),
            (MyKey::new("Irene", "Kyiv"), 90),
            (MyKey::new("Juliano", "Milan"), 100),
            (MyKey::new("Kelly", "Ottawa"), 110),
            (MyKey::new("Liane", "Vancouver"), 120),
            (MyKey::new("Michel", "Whistler"), 130),
            (MyKey::new("Normand", "St-Sauveur"), 140),
            (MyKey::new("Ovid", "Oslo"), 150),
            (MyKey::new("Paul", "Prague"), 160),
            (MyKey::new("Quintin", "Los Angeles"), 170),
            (MyKey::new("Robert", "Seattle"), 180),
            (MyKey::new("Sam", "Eugene"), 190),
            (MyKey::new("Teddy", "San Diego"), 200),
        ];

        let mut probe = vec![
            MyKey::new("Alex", "Lisbon"),
            MyKey::new("Alex", "Lisbon2"),
            MyKey::new("Brian", "Paris"),
            MyKey::new("Brian", "2Paris"),
            MyKey::new("Cathy", "New York"),
            MyKey::new("Cathy2", "New York"),
            MyKey::new("Dylan", "Tokyo"),
            MyKey::new("2Dylan", "Tokyo"),
            MyKey::new("Ella", "Rio"),
            MyKey::new("Ella2", "Rio"),
            MyKey::new("Fred", "Oslo"),
            MyKey::new("Fred", "2Oslo"),
            MyKey::new("Gina", "Montreal"),
            MyKey::new("Gina", "Montreal2"),
            MyKey::new("Helena", "Quebec"),
            MyKey::new("Helena", "2Quebec"),
            MyKey::new("Irene", "Kyiv"),
            MyKey::new("Irene2", "Kyiv"),
            MyKey::new("Juliano", "Milan"),
            MyKey::new("2Juliano", "Milan"),
            MyKey::new("Kelly", "Ottawa"),
            MyKey::new("Kelly2", "Ottawa"),
            MyKey::new("Liane", "Vancouver"),
            MyKey::new("Liane", "2Vancouver"),
            MyKey::new("Michel", "Whistler"),
            MyKey::new("Michel", "Whistler2"),
            MyKey::new("Normand", "St-Sauveur"),
            MyKey::new("Normand", "2St-Sauveur"),
            MyKey::new("Ovid", "Oslo"),
            MyKey::new("Ovid2", "Oslo"),
            MyKey::new("Paul", "Prague"),
            MyKey::new("2Paul", "Prague"),
            MyKey::new("Quintin", "Los Angeles"),
            MyKey::new("Quintin2", "Los Angeles"),
            MyKey::new("Robert", "Seattle"),
            MyKey::new("Robert", "2Seattle"),
            MyKey::new("Sam", "Eugene"),
            MyKey::new("Sam", "Eugene2"),
            MyKey::new("Teddy", "San Diego"),
            MyKey::new("Teddy", "2San Diego"),
        ];

        input.truncate(size);

        // 50% hit rate
        probe.truncate(size * 2);

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

        let map = BTreeMap::from_iter(input.clone());
        group.bench_with_input(BenchmarkId::new("BTreeMap", size), &size, |b, _| {
            b.iter(|| {
                for key in &probe {
                    _ = black_box(map.contains_key(key));
                }
            });
        });

        let map = HashMap::<_, _, SmallCollection, _>::new(
            input.clone(),
            BridgeHasher::new(ahash::RandomState::new()),
        )
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

criterion_group!(benches, misc_keys);
criterion_main!(benches);
