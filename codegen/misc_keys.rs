use frozen_collections::facade_maps::FacadeHashMap;
use frozen_collections::hashers::BridgeHasher;
use frozen_collections::*;
use std::collections::HashMap as StdHashMap;
use std::hint::black_box;

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
    let mut hm = StdHashMap::with_capacity_and_hasher(v.len(), ahash::RandomState::new());
    hm.extend(v.clone());

    _ = black_box(call_frozen_map(&fm, &v[0].0));
    _ = black_box(call_hash_map(&cm, &v[0].0));
    _ = black_box(call_std_hash_map(&hm, &v[0].0));
}

#[inline(never)]
fn call_frozen_map(map: &FacadeHashMap<MyKey, i32>, key: &MyKey) -> bool {
    map.contains_key(key)
}

#[inline(never)]
fn call_hash_map(map: &maps::HashMap<MyKey, i32, SmallCollection>, key: &MyKey) -> bool {
    map.contains_key(key)
}

#[inline(never)]
fn call_std_hash_map(map: &StdHashMap<MyKey, i32, ahash::RandomState>, key: &MyKey) -> bool {
    map.contains_key(key)
}
