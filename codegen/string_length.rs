use std::collections::HashMap;
use std::hint::black_box;

use frozen_collections::specialized_maps::LengthMapNoCollisions;
use frozen_collections::*;

fn main() {
    let input = frozen_map!("1": 1, "22":2, "333":3, "4444":4, "55555":5, "666666":6, "7777777":7, "88888888":8, "999999999":9);
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

    let map: HashMap<_, _, ahash::RandomState> =
        input.iter().map(|x| (x.0.clone(), *x.1)).collect();
    for key in probe {
        _ = black_box(call_hm(&map, key));
    }

    let map: FrozenStringMap<_> = input.iter().map(|x| (x.0.clone(), *x.1)).collect();
    for key in probe {
        _ = black_box(call_fsm(&map, key));
    }

    let map = input;
    for key in probe {
        _ = black_box(call_lm(&map, key));
    }
}

#[inline(never)]
fn call_hm(map: &HashMap<String, i32, ahash::RandomState>, key: &str) -> bool {
    map.contains_key(key)
}

#[inline(never)]
fn call_fsm(map: &FrozenStringMap<i32>, key: &str) -> bool {
    map.contains_key(key)
}

#[inline(never)]
fn call_lm(map: &LengthMapNoCollisions<String, i32>, key: &str) -> bool {
    map.contains_key(key)
}
