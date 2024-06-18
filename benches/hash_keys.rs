extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use core::hash::Hash;
use core::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use frozen_collections::hashers::BridgeHasher;
use frozen_collections::maps::*;
use frozen_collections::{fz_hash_map, SmallCollection};
use hashbrown::HashMap as HashbrownMap;

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

fn hash_keys(c: &mut Criterion) {
    let mut group = c.benchmark_group("hash_keys");

    let frozen = fz_hash_map!({
        MyKey::new("Alex", "Lisbon"): 10,
        MyKey::new("Brian", "Paris"): 20,
        MyKey::new("Cathy", "New York"): 30,
        MyKey::new("Dylan", "Tokyo"): 40,
        MyKey::new("Ella", "Rio"): 50,
        MyKey::new("Fred", "Oslo"): 60,
        MyKey::new("Gina", "Montreal"): 70,
        MyKey::new("Helena", "Quebec"): 80,
        MyKey::new("Irene", "Kyiv"): 90,
        MyKey::new("Juliano", "Milan"): 100,
        MyKey::new("Kelly", "Ottawa"): 110,
        MyKey::new("Liane", "Vancouver"): 120,
        MyKey::new("Michel", "Whistler"): 130,
        MyKey::new("Normand", "St-Sauveur"): 140,
        MyKey::new("Ovid", "Oslo"): 150,
        MyKey::new("Paul", "Prague"): 160,
        MyKey::new("Quintin", "Los Angeles"): 170,
        MyKey::new("Robert", "Seattle"): 180,
        MyKey::new("Sam", "Eugene"): 190,
        MyKey::new("Teddy", "San Diego"): 200,
    });

    let input: Vec<(MyKey, i32)> = frozen.into_iter().collect();

    // 50% hit rate
    let probe = vec![
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

    let map = HashMap::<_, _, SmallCollection, _>::new(
        input,
        BridgeHasher::new(ahash::RandomState::new()),
    )
    .unwrap();
    group.bench_function("HashMap", |b| {
        b.iter(|| {
            for key in &probe {
                _ = black_box(map.contains_key(key));
            }
        });
    });

    group.finish();
}

criterion_group!(benches, hash_keys);
criterion_main!(benches);
