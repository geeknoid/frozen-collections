#![no_std]
extern crate alloc;

use alloc::vec;
use core::hint::black_box;
use frozen_collections::facade_maps::FacadeScalarMap;
use frozen_collections::hashers::PassthroughHasher;
use frozen_collections::maps::{
    BinarySearchMap, DenseScalarLookupMap, EytzingerSearchMap, HashMap, OrderedScanMap, ScanMap,
    SparseScalarLookupMap,
};
use frozen_collections::*;
use hashbrown::HashMap as HashbrownMap;

fn main() {
    let input = vec![(0, 0), (1, 0), (2, 0), (3, 0)];
    let probe = vec![0, 1, 2];

    let map: HashbrownMap<_, _, ahash::RandomState> = input.clone().into_iter().collect();
    for key in probe.clone() {
        _ = black_box(call_hashbrown_map(&map, key));
    }

    let map = HashMap::new(input.clone(), PassthroughHasher::default()).unwrap();
    for key in probe.clone() {
        _ = black_box(call_hash_map_with_passthrough_hasher(&map, key));
    }

    let map = DenseScalarLookupMap::new(input.clone()).unwrap();
    for key in probe.clone() {
        _ = black_box(call_dense_scalar_lookup_map(&map, key));
    }

    let map = SparseScalarLookupMap::new(input.clone());
    for key in probe.clone() {
        _ = black_box(call_sparse_scalar_lookup_map(&map, key));
    }

    let map = ScanMap::new(input.clone());
    for key in probe.clone() {
        _ = black_box(call_scan_map(&map, key));
    }

    let map = OrderedScanMap::new(input.clone());
    for key in probe.clone() {
        _ = black_box(call_ordered_scan_map(&map, key));
    }

    let map = BinarySearchMap::new(input.clone());
    for key in probe.clone() {
        _ = black_box(call_binary_search_map(&map, key));
    }

    let map = EytzingerSearchMap::new(input.clone());
    for key in probe.clone() {
        _ = black_box(call_eytzinger_search_map(&map, key));
    }

    let map = FacadeScalarMap::new(input);
    for key in probe {
        _ = black_box(call_facade_scalar_map(&map, key));
    }
}

#[inline(never)]
fn call_hashbrown_map(map: &HashbrownMap<i32, i32, ahash::RandomState>, key: i32) -> bool {
    map.contains_key(&key)
}

#[inline(never)]
fn call_hash_map_with_passthrough_hasher(
    map: &HashMap<i32, i32, SmallCollection, PassthroughHasher>,
    key: i32,
) -> bool {
    map.contains_key(&key)
}

#[inline(never)]
fn call_dense_scalar_lookup_map(map: &DenseScalarLookupMap<i32, i32>, key: i32) -> bool {
    map.contains_key(&key)
}

#[inline(never)]
fn call_sparse_scalar_lookup_map(map: &SparseScalarLookupMap<i32, i32>, key: i32) -> bool {
    map.contains_key(&key)
}

#[inline(never)]
fn call_facade_scalar_map(map: &FacadeScalarMap<i32, i32>, key: i32) -> bool {
    map.contains_key(&key)
}

#[inline(never)]
fn call_binary_search_map(map: &BinarySearchMap<i32, i32>, key: i32) -> bool {
    map.contains_key(&key)
}

#[inline(never)]
fn call_eytzinger_search_map(map: &EytzingerSearchMap<i32, i32>, key: i32) -> bool {
    map.contains_key(&key)
}

#[inline(never)]
fn call_scan_map(map: &ScanMap<i32, i32>, key: i32) -> bool {
    map.contains_key(&key)
}

#[inline(never)]
fn call_ordered_scan_map(map: &OrderedScanMap<i32, i32>, key: i32) -> bool {
    map.contains_key(&key)
}
