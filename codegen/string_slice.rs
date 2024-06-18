use std::collections::HashMap;
use std::hint::black_box;

use frozen_collections::specialized_maps::LeftSliceMapNoCollisions;
use frozen_collections::*;

fn main() {
    let input = frozen_map!("ALongPrefixRedd": 1, "ALongPrefixGree":2, "ALongPrefixBlue":3, "ALongPrefixCyan":4);
    let probe = [
        "ALongPrefixRedd",
        "ALongPrefixCyan",
        "Tomato",
        "Potato",
        "Carrot",
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
        _ = black_box(call_lsm(&map, key));
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
fn call_lsm(map: &LeftSliceMapNoCollisions<String, i32>, key: &str) -> bool {
    map.contains_key(key)
}
