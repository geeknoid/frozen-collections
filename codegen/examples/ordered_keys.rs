#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec;
use core::hint::black_box;
use frozen_collections::maps::EytzingerSearchMap;
use frozen_collections::{FzOrderedMap, MapQuery};

#[derive(Ord, PartialOrd, PartialEq, Eq, Clone)]
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

    let fm = FzOrderedMap::new(v.clone());
    let esm = EytzingerSearchMap::new(v.clone());

    _ = black_box(call_fz_ordered_map(&fm, &v[0].0));
    _ = black_box(call_eytzinger_search_map(&esm, &v[0].0));
}

#[inline(never)]
fn call_fz_ordered_map(map: &FzOrderedMap<MyKey, i32>, key: &MyKey) -> bool {
    map.contains_key(key)
}

#[inline(never)]
fn call_eytzinger_search_map(map: &EytzingerSearchMap<MyKey, i32>, key: &MyKey) -> bool {
    map.contains_key(key)
}
