//! Validates quality of generated code

#![no_std]
extern crate alloc;

use alloc::boxed::Box;
use core::hint::black_box;
use frozen_collections::{FzStringMap, fz_string_map};
use hashbrown::HashMap as HashbrownMap;

fz_string_map!(static MAP: MyMapType<&'static str, i32>, { "1": 1, "22":2, "333":3, "4444":4, "55555":5, "666666":6, "7777777":7, "88888888":8, "999999999":9 });

fn main() {
    let input = MAP.clone();
    let probe = ["0", "1", "22", "333", "4444", "A", "B", "C", "D", "ABCDEFGHIJKL"];

    let map: HashbrownMap<_, _> = input.iter().map(|x| (*x.0, *x.1)).collect();
    for key in probe {
        _ = black_box(call_hashbrown_map(&map, key));
    }

    let map = FzStringMap::new(input.iter().map(|x| (*x.0, *x.1)).collect());
    for key in probe {
        _ = black_box(call_fz_string_map(&map, key));
    }

    let map = input;
    for key in probe {
        _ = black_box(call_inline_hash_map_with_length_hasher(&map, key));
    }
}

#[inline(never)]
fn call_hashbrown_map(map: &HashbrownMap<&str, i32>, key: &str) -> bool {
    map.contains_key(key)
}

#[inline(never)]
fn call_inline_hash_map_with_length_hasher(map: &MyMapType, key: &str) -> bool {
    map.contains_key(key)
}

#[inline(never)]
fn call_fz_string_map(map: &FzStringMap<Box<str>, i32>, key: &str) -> bool {
    map.contains_key(key)
}
