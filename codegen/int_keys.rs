use std::collections::HashMap as StdHashMap;
use std::hint::black_box;

use frozen_collections::hashers::PassthroughHasher;
use frozen_collections::maps::{
    BinarySearchMap, DenseSequenceLookupMap, HashMap, OrderedScanMap, ScanMap,
    SparseSequenceLookupMap,
};
use frozen_collections::FzSequenceMap;
use frozen_collections::SmallCollection;

fn main() {
    let input = vec![(0, 0), (1, 0), (2, 0), (3, 0)];
    let probe = vec![0, 1, 2];

    let map: StdHashMap<_, _, ahash::RandomState> = input.clone().into_iter().collect();
    for key in probe.clone() {
        _ = black_box(call_std_hash_map(&map, key));
    }

    let map = HashMap::with_hasher(input.clone(), 128, PassthroughHasher::default()).unwrap();
    for key in probe.clone() {
        _ = black_box(call_hash_map(&map, key));
    }

    let map = DenseSequenceLookupMap::new(input.clone()).unwrap();
    for key in probe.clone() {
        _ = black_box(call_dense_sequence_lookup_map(&map, key));
    }

    let map = SparseSequenceLookupMap::new(input.clone()).unwrap();
    for key in probe.clone() {
        _ = black_box(call_sparse_sequence_lookup_map(&map, key));
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

    let map = FzSequenceMap::new(input);
    for key in probe {
        _ = black_box(call_frozen_sequence_map(&map, key));
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
fn call_dense_sequence_lookup_map(map: &DenseSequenceLookupMap<i32, i32>, key: i32) -> bool {
    map.contains_key(&key)
}

#[inline(never)]
fn call_sparse_sequence_lookup_map(
    map: &SparseSequenceLookupMap<i32, i32, SmallCollection>,
    key: i32,
) -> bool {
    map.contains_key(&key)
}

#[inline(never)]
fn call_frozen_sequence_map(map: &FzSequenceMap<i32, i32>, key: i32) -> bool {
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
