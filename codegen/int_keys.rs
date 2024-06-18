use frozen_collections::facade_maps::FacadeScalarMap;
use frozen_collections::hashers::PassthroughHasher;
use frozen_collections::maps::{
    BinarySearchMap, DenseScalarLookupMap, HashMap, OrderedScanMap, ScanMap, SparseScalarLookupMap,
};
use frozen_collections::SmallCollection;
use std::collections::HashMap as StdHashMap;
use std::hint::black_box;

fn main() {
    let input = vec![(0, 0), (1, 0), (2, 0), (3, 0)];
    let probe = vec![0, 1, 2];

    let map: StdHashMap<_, _, ahash::RandomState> = input.clone().into_iter().collect();
    for key in probe.clone() {
        _ = black_box(call_std_hash_map(&map, key));
    }

    let map = HashMap::new(input.clone(), PassthroughHasher::default()).unwrap();
    for key in probe.clone() {
        _ = black_box(call_hash_map(&map, key));
    }

    let map = DenseScalarLookupMap::new(input.clone()).unwrap();
    for key in probe.clone() {
        _ = black_box(call_dense_scalar_lookup_map(&map, key));
    }

    let map = SparseScalarLookupMap::new(input.clone()).unwrap();
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

    let map = FacadeScalarMap::new(input);
    for key in probe {
        _ = black_box(call_frozen_scalar_map(&map, key));
    }
}

#[inline(never)]
fn call_std_hash_map(map: &StdHashMap<i32, i32, ahash::RandomState>, key: i32) -> bool {
    map.contains_key(&key)
}

#[inline(never)]
fn call_hash_map(map: &HashMap<i32, i32, SmallCollection, PassthroughHasher>, key: i32) -> bool {
    map.contains_key(&key)
}

#[inline(never)]
fn call_dense_scalar_lookup_map(map: &DenseScalarLookupMap<i32, i32>, key: i32) -> bool {
    map.contains_key(&key)
}

#[inline(never)]
fn call_sparse_scalar_lookup_map(
    map: &SparseScalarLookupMap<i32, i32, SmallCollection>,
    key: i32,
) -> bool {
    map.contains_key(&key)
}

#[inline(never)]
fn call_frozen_scalar_map(map: &FacadeScalarMap<i32, i32>, key: i32) -> bool {
    map.contains_key(&key)
}

#[inline(never)]
fn call_binary_search_map(map: &BinarySearchMap<i32, i32>, key: i32) -> bool {
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
