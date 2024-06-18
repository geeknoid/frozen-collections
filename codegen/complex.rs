use std::collections::HashMap;
use std::hint::black_box;

use frozen_collections::specialized_maps::CommonMap;
use frozen_collections::*;

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

    let fm = FrozenMap::new(v.clone()).unwrap();
    let cm = CommonMap::new(v.clone()).unwrap();
    let mut hm = HashMap::with_capacity_and_hasher(v.len(), ahash::RandomState::new());
    hm.extend(v.clone());

    _ = black_box(call_fm(&fm, &v[0].0));
    _ = black_box(call_cm(&cm, &v[0].0));
    _ = black_box(call_hm(&hm, &v[0].0));
}

#[inline(never)]
fn call_fm(map: &FrozenMap<MyKey, i32>, key: &MyKey) -> bool {
    map.contains_key(key)
}

#[inline(never)]
fn call_cm(map: &CommonMap<MyKey, i32>, key: &MyKey) -> bool {
    map.contains_key(key)
}

#[inline(never)]
fn call_hm(map: &HashMap<MyKey, i32, ahash::RandomState>, key: &MyKey) -> bool {
    map.contains_key(key)
}
