#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec;
use core::hint::black_box;
use frozen_collections::facade_maps::FacadeHashMap;
use frozen_collections::hashers::BridgeHasher;
use frozen_collections::*;
use hashbrown::HashMap as HashbrownMap;

#[derive(Hash, PartialEq, Eq, Clone)]
struct MyKey {
    name: String,
    city: String,
}

fn main() {
    let v = vec![
        (
            MyKey {
                name: "Helter1".to_string(),
                city: "Skelter".to_string(),
            },
            42,
        ),
        (
            MyKey {
                name: "Helter2".to_string(),
                city: "Skelter".to_string(),
            },
            42,
        ),
        (
            MyKey {
                name: "Helter3".to_string(),
                city: "Skelter".to_string(),
            },
            42,
        ),
        (
            MyKey {
                name: "Helter4".to_string(),
                city: "Skelter".to_string(),
            },
            42,
        ),
        (
            MyKey {
                name: "Helter5".to_string(),
                city: "Skelter".to_string(),
            },
            42,
        ),
        (
            MyKey {
                name: "Helter6".to_string(),
                city: "Skelter".to_string(),
            },
            42,
        ),
    ];

    let fm = FacadeHashMap::new(v.clone(), BridgeHasher::default());
    let cm = maps::HashMap::new(v.clone(), BridgeHasher::new(ahash::RandomState::new())).unwrap();
    let mut hm = HashbrownMap::with_capacity_and_hasher(v.len(), ahash::RandomState::new());
    hm.extend(v.clone());

    _ = black_box(call_facade_hash_map_with_bridge_hasher(&fm, &v[0].0));
    _ = black_box(call_hash_map_with_bridge_hasher(&cm, &v[0].0));
    _ = black_box(call_hashbrown_map(&hm, &v[0].0));
}

#[inline(never)]
fn call_facade_hash_map_with_bridge_hasher(map: &FacadeHashMap<MyKey, i32>, key: &MyKey) -> bool {
    map.contains_key(key)
}

#[inline(never)]
fn call_hash_map_with_bridge_hasher(map: &maps::HashMap<MyKey, i32>, key: &MyKey) -> bool {
    map.contains_key(key)
}

#[inline(never)]
fn call_hashbrown_map(map: &HashbrownMap<MyKey, i32, ahash::RandomState>, key: &MyKey) -> bool {
    map.contains_key(key)
}
