#![no_std]
extern crate alloc;

use core::hint::black_box;
use frozen_collections::ahash::RandomState;
use frozen_collections::facade_maps::FacadeStringMap;
use frozen_collections::*;
use hashbrown::HashMap as HashbrownMap;

fz_string_map!(static MAP: MyMapType<&str, i32>, { "ALongPrefixRedd": 1, "ALongPrefixGree":2, "ALongPrefixBlue":3, "ALongPrefixCyan":4, "ALongPrefixPurple":5, "ALongPrefix:Yellow":6, "ALongPrefixMagenta":7 });

fn main() {
    let input = MAP.clone();
    let probe = [
        "ALongPrefixRedd",
        "ALongPrefixCyan",
        "Tomato",
        "Potato",
        "Carrot",
    ];

    let map: HashbrownMap<_, _, RandomState> = input.iter().map(|x| (*x.0, *x.1)).collect();
    for key in probe {
        _ = black_box(call_hashbrown_map(&map, key));
    }

    let map = FacadeStringMap::new(
        input.iter().map(|x| (*x.0, *x.1)).collect(),
        RandomState::default(),
    );
    for key in probe {
        _ = black_box(call_facade_string_map(&map, key));
    }

    let map = input;
    for key in probe {
        _ = black_box(call_inline_hash_map_with_inline_left_range_hasher(
            &map, key,
        ));
    }
}

#[inline(never)]
fn call_hashbrown_map(map: &HashbrownMap<&str, i32, RandomState>, key: &str) -> bool {
    map.contains_key(key)
}

#[inline(never)]
fn call_facade_string_map(map: &FacadeStringMap<&str, i32>, key: &str) -> bool {
    map.contains_key(key)
}

#[inline(never)]
fn call_inline_hash_map_with_inline_left_range_hasher(map: &MyMapType, key: &str) -> bool {
    map.contains_key(key)
}
