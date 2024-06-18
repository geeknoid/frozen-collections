mod common;

use common::*;
use frozen_collections::*;
use frozen_collections_core::facade_maps::*;
use frozen_collections_core::facade_sets::*;
use frozen_collections_core::hashers::BridgeHasher;
use frozen_collections_core::maps::*;
use frozen_collections_core::sets::*;
use hashbrown::HashSet as HashbrownSet;
use std::collections::HashMap as StdHashMap;

macro_rules! test_all {
    ( $( $input:literal ),* : $( $other:literal ),*) => {
        let set_reference = HashbrownSet::from_iter(vec![ $( $input, )* ].into_iter());
        let set_other = HashbrownSet::from_iter(vec![ $( $other, )* ].into_iter());
        let set_input = vec![ $($input,)* ];

        let map_reference = StdHashMap::<_, _, ahash::RandomState>::from_iter(vec![ $( ($input, ()), )* ].into_iter());
        let map_other = StdHashMap::<_, _, ahash::RandomState>::from_iter(vec![ $( ($other, ()), )* ].into_iter());
        let map_input = vec![ $( ($input, ()), )* ];

        let mut m = fz_scalar_map!({ $( $input: (), )* });
        test_map(&m, &map_reference, &map_other);
        test_map_ops(&m, &map_reference);
        test_map_iter(&mut m, &map_reference);

        let s = fz_scalar_set!({ $($input,)* });
        test_set(&s, &set_reference, &set_other);
        test_set_ops(&s, &set_reference, &set_other);
        test_set_iter(&s, &set_reference);

        let mut m = fz_scalar_map!(map_input.clone());
        test_map(&m, &map_reference, &map_other);
        test_map_ops(&m, &map_reference);
        test_map_iter(&mut m, &map_reference);

        let s = fz_scalar_set!(set_input.clone());
        test_set(&s, &set_reference, &set_other);
        test_set_ops(&s, &set_reference, &set_other);
        test_set_iter(&s, &set_reference);

        let mut m = BinarySearchMap::new(map_input.clone());
        test_map(&m, &map_reference, &map_other);
        test_map_ops(&m, &map_reference);
        test_map_iter(&mut m, &map_reference);

        let s = BinarySearchSet::new(m);
        test_set(&s, &set_reference, &set_other);
        test_set_ops(&s, &set_reference, &set_other);
        test_set_iter(&s, &set_reference);

        let mut m = OrderedScanMap::new(map_input.clone());
        test_map(&m, &map_reference, &map_other);
        test_map_ops(&m, &map_reference);
        test_map_iter(&mut m, &map_reference);

        let s = OrderedScanSet::new(m);
        test_set(&s, &set_reference, &set_other);
        test_set_ops(&s, &set_reference, &set_other);
        test_set_iter(&s, &set_reference);

        let mut m = ScanMap::new(map_input.clone());
        test_map(&m, &map_reference, &map_other);
        test_map_ops(&m, &map_reference);
        test_map_iter(&mut m, &map_reference);

        let s = ScanSet::new(m);
        test_set(&s, &set_reference, &set_other);
        test_set_ops(&s, &set_reference, &set_other);
        test_set_iter(&s, &set_reference);

        if let Ok(mut m) = DenseScalarLookupMap::new(map_input.clone()) {
            test_map(&m, &map_reference, &map_other);
            test_map_ops(&m, &map_reference);
            test_map_iter(&mut m, &map_reference);

            let s = DenseScalarLookupSet::new(m);
            test_set(&s, &set_reference, &set_other);
            test_set_ops(&s, &set_reference, &set_other);
            test_set_iter(&s, &set_reference);
        }

        if let Ok(mut m) = SparseScalarLookupMap::<_, _>::new(map_input.clone()) {
            test_map(&m, &map_reference, &map_other);
            test_map_ops(&m, &map_reference);
            test_map_iter(&mut m, &map_reference);

            let s = SparseScalarLookupSet::new(m);
            test_set(&s, &set_reference, &set_other);
            test_set_ops(&s, &set_reference, &set_other);
            test_set_iter(&s, &set_reference);
        }

        if let Ok(mut m) = HashMap::<_, _>::new(map_input.clone(), BridgeHasher::default()) {
            test_map(&m, &map_reference, &map_other);
            test_map_ops(&m, &map_reference);
            test_map_iter(&mut m, &map_reference);

            let s = HashSet::new(m);
            test_set(&s, &set_reference, &set_other);
            test_set_ops(&s, &set_reference, &set_other);
            test_set_iter(&s, &set_reference);
        }

        let mut m = FacadeOrderedMap::new(map_input.clone());
        test_map(&m, &map_reference, &map_other);
        test_map_ops(&m, &map_reference);
        test_map_iter(&mut m, &map_reference);

        let s = FacadeOrderedSet::new(m);
        test_set(&s, &set_reference, &set_other);
        test_set_ops(&s, &set_reference, &set_other);
        test_set_iter(&s, &set_reference);

        let mut m = FacadeScalarMap::new(map_input.clone());
        test_map(&m, &map_reference, &map_other);
        test_map_ops(&m, &map_reference);
        test_map_iter(&mut m, &map_reference);

        let s = FacadeScalarSet::new(m);
        test_set(&s, &set_reference, &set_other);
        test_set_ops(&s, &set_reference, &set_other);
        test_set_iter(&s, &set_reference);

        let mut m = FacadeHashMap::<_, _>::new(map_input.clone(), BridgeHasher::default());
        test_map(&m, &map_reference, &map_other);
        test_map_ops(&m, &map_reference);
        test_map_iter(&mut m, &map_reference);

        let s = FacadeHashSet::new(m);
        test_set(&s, &set_reference, &set_other);
        test_set_ops(&s, &set_reference, &set_other);
        test_set_iter(&s, &set_reference);

        let s = std::collections::HashSet::<_, ahash::RandomState>::from_iter(set_input.clone().into_iter());
        test_set(&s, &set_reference, &set_other);

        let s = std::collections::BTreeSet::from_iter(set_input.clone().into_iter());
        test_set(&s, &set_reference, &set_other);

        let s = hashbrown::HashSet::<_, ahash::RandomState>::from_iter(set_input.clone().into_iter());
        test_set(&s, &set_reference, &set_other);

        // handle string cases

        let set_reference = HashbrownSet::<String>::from_iter(vec![ $( $input.to_string(), )* ].into_iter());
        let set_other = HashbrownSet::<String>::from_iter(vec![ $( $other.to_string(), )* ].into_iter());
        let set_input = vec![ $( $input.to_string(), )* ];

        let map_reference = StdHashMap::<_, _, ahash::RandomState>::from_iter(vec![ $( ($input.to_string(), ()), )* ].into_iter());
        let map_other = StdHashMap::<_, _, ahash::RandomState>::from_iter(vec![ $( ($other.to_string(), ()), )* ].into_iter());
        let map_input = vec![ $( ($input.to_string(), ()), )* ];

        let mut m = fz_string_map!({ $($input.to_string(): (),)* });
        test_map(&m, &map_reference, &map_other);
        test_map_ops(&m, &map_reference);
        test_map_iter(&mut m, &map_reference);

        let s = fz_string_set!({ $($input.to_string(),)* });
        test_set(&s, &set_reference, &set_other);
        test_set_ops(&s, &set_reference, &set_other);
        test_set_iter(&s, &set_reference);

        let mut m = fz_string_map!(map_input.clone());
        test_map(&m, &map_reference, &map_other);
        test_map_ops(&m, &map_reference);
        test_map_iter(&mut m, &map_reference);

        let s = fz_string_set!(set_input.clone());
        test_set(&s, &set_reference, &set_other);
        test_set_ops(&s, &set_reference, &set_other);
        test_set_iter(&s, &set_reference);

        let mut m = FacadeStringMap::new(map_input.clone(), ahash::RandomState::default());
        test_map(&m, &map_reference, &map_other);
        test_map_ops(&m, &map_reference);
        test_map_iter(&mut m, &map_reference);

        let s = FacadeStringSet::new(m);
        test_set(&s, &set_reference, &set_other);
        test_set_ops(&s, &set_reference, &set_other);
        test_set_iter(&s, &set_reference);
    }
}

#[test]
fn test_sets() {
    test_all!(1, 2, 3 : 3, 4, 5);
    test_all!(0, 1 : 0, 1);
    test_all!(3, 1, 2, 3, 3 : 3, 4, 5);
    test_all!(1, 2, 3 : 1, 2, 3, 4, 5);
    test_all!(1, 2, 3 : 1, 2);
    test_all!(1, 2, 3 : 2);
    test_all!(1, 2, 4 : 2);
    test_all!(1, 2, 4 : 3);
    test_all!(1, 2, 4, 1500 : 3);
    test_all!(1, 2, 4, 1500 : 2500);

    test_all!(1 : 3);
    test_all!(1, 2 : 3);
    test_all!(1, 2, 3 : 3);
    test_all!(1, 2, 3, 4 : 3);
    test_all!(1, 2, 3, 4, 5 : 3);
    test_all!(1, 2, 3, 4, 5, 6 : 3);
    test_all!(1, 2, 3, 4, 5, 6, 7 : 3, 5);
    test_all!(1, 2, 3, 4, 5, 6, 7, 8 : 3);
    test_all!(1, 2, 3, 4, 5, 6, 7, 8, 9 : 3, 10);
    test_all!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10 : 3);
}

#[test]
fn test_set_defaults() {
    test_set_default::<BinarySearchSet<i32>, i32>();
    test_set_default::<OrderedScanSet<i32>, i32>();
    test_set_default::<ScanSet<i32>, i32>();
    test_set_default::<DenseScalarLookupSet<i32>, i32>();
    test_set_default::<SparseScalarLookupSet<i32>, i32>();
    test_set_default::<HashSet<i32>, i32>();

    test_set_default::<FacadeHashSet<i32>, i32>();
    test_set_default::<FacadeOrderedSet<i32>, i32>();
    test_set_default::<FacadeScalarSet<i32>, i32>();
    test_set_default::<FacadeStringSet, String>();
}

#[test]
fn test_map_defaults() {
    test_map_default::<BinarySearchMap<i32, i32>, i32>();
    test_map_default::<OrderedScanMap<i32, i32>, i32>();
    test_map_default::<ScanMap<i32, i32>, i32>();
    test_map_default::<DenseScalarLookupMap<i32, i32>, i32>();
    test_map_default::<SparseScalarLookupMap<i32, i32>, i32>();
    test_map_default::<HashMap<i32, i32>, i32>();

    test_map_default::<FacadeHashMap<i32, i32>, i32>();
    test_map_default::<FacadeOrderedMap<i32, i32>, i32>();
    test_map_default::<FacadeScalarMap<i32, i32>, i32>();
    test_map_default::<FacadeStringMap<i32>, String>();
}
