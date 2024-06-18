use std::collections::HashMap as StdHashMap;
use std::hint::black_box;

use frozen_collections::*;

fz_string_map!(static MAP: MyMapType<&str, i32> = { "ALongPrefixRedd": 1, "ALongPrefixGree":2, "ALongPrefixBlue":3, "ALongPrefixCyan":4 });

fn main() {
    let input = MAP.clone();
    let probe = [
        "ALongPrefixRedd",
        "ALongPrefixCyan",
        "Tomato",
        "Potato",
        "Carrot",
    ];

    let map: StdHashMap<_, _, ahash::RandomState> = input.iter().map(|x| (*x.0, *x.1)).collect();
    for key in probe {
        _ = black_box(call_std_hash_map(&map, key));
    }

    let map: FzStringMap<_> = input.iter().map(|x| (*x.0, *x.1)).collect();
    for key in probe {
        _ = black_box(call_frozen_string_map(&map, key));
    }

    let map = input;
    for key in probe {
        _ = black_box(call_inline_left_range_hash_map(&map, key));
    }
}

#[inline(never)]
fn call_std_hash_map(map: &StdHashMap<&str, i32, ahash::RandomState>, key: &str) -> bool {
    map.contains_key(key)
}

#[inline(never)]
fn call_frozen_string_map(map: &FzStringMap<i32>, key: &str) -> bool {
    map.contains_key(key)
}

#[inline(never)]
fn call_inline_left_range_hash_map(map: &MyMapType, key: &str) -> bool {
    map.contains_key(key)
}
