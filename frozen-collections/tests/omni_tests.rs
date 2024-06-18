mod common;

use common::*;
use frozen_collections::*;
use frozen_collections_core::facade_maps::*;
use frozen_collections_core::facade_sets::*;
use frozen_collections_core::hashers::BridgeHasher;
use frozen_collections_core::macros::fz_scalar_map_macro;
use frozen_collections_core::maps::*;
use frozen_collections_core::sets::*;
use hashbrown::HashSet as HashbrownSet;
use quote::quote;
use std::collections::HashMap as StdHashMap;

macro_rules! test_str {
    ( $( $input:expr ),* ; $( $other:literal ),*) => {
        // handle &str cases

        let set_reference = HashbrownSet::<&str>::from_iter(vec![ $( $input, )* ].into_iter());
        let set_other = HashbrownSet::<&str>::from_iter(vec![ $( $other, )* ].into_iter());

        let map_reference = StdHashMap::<_, _, ahash::RandomState>::from_iter(vec![ $( ($input, ()), )* ].into_iter());
        let map_other = StdHashMap::<_, _, ahash::RandomState>::from_iter(vec![ $( ($other, ()), )* ].into_iter());

        let mut m = fz_string_map!({ $( $input: (),)* });
        test_map(&m, &map_reference, &map_other);
        test_map_ops(&m, &map_reference);
        test_map_iter(&m, &map_reference);
        test_map_iter_mut(&mut m, &map_reference);

        let s = fz_string_set!({ $( $input,)* });
        test_set(&s, &set_reference, &set_other);
        test_set_ops(&s, &set_reference, &set_other);
        test_set_iter(&s, &set_reference);
    }
}

macro_rules! test_all {
    ( $( $input:expr ),* ; $( $other:literal ),*) => {
        let set_reference = HashbrownSet::from_iter(vec![ $( $input, )* ].into_iter());
        let set_other = HashbrownSet::from_iter(vec![ $( $other, )* ].into_iter());
        let set_input = vec![ $($input,)* ];

        let map_reference = StdHashMap::<_, _, ahash::RandomState>::from_iter(vec![ $( ($input, ()), )* ].into_iter());
        let map_other = StdHashMap::<_, _, ahash::RandomState>::from_iter(vec![ $( ($other, ()), )* ].into_iter());
        let map_input = vec![ $( ($input, ()), )* ];

        let mut m = fz_scalar_map!({ $( $input: (), )* });
        test_map(&m, &map_reference, &map_other);
        test_map_ops(&m, &map_reference);
        test_map_iter(&m, &map_reference);
        test_map_iter_mut(&mut m, &map_reference);

        let s = fz_scalar_set!({ $($input,)* });
        test_set(&s, &set_reference, &set_other);
        test_set_ops(&s, &set_reference, &set_other);
        test_set_iter(&s, &set_reference);

        let mut m = fz_scalar_map!(map_input.clone());
        test_map(&m, &map_reference, &map_other);
        test_map_ops(&m, &map_reference);
        test_map_iter(&m, &map_reference);
        test_map_iter_mut(&mut m, &map_reference);

        let s = fz_scalar_set!(set_input.clone());
        test_set(&s, &set_reference, &set_other);
        test_set_ops(&s, &set_reference, &set_other);
        test_set_iter(&s, &set_reference);

        let mut m = fz_hash_map!({ $( $input: (), )* });
        test_map(&m, &map_reference, &map_other);
        test_map_ops(&m, &map_reference);
        test_map_iter(&m, &map_reference);
        test_map_iter_mut(&mut m, &map_reference);

        let s = fz_hash_set!({ $($input,)* });
        test_set(&s, &set_reference, &set_other);
        test_set_ops(&s, &set_reference, &set_other);
        test_set_iter(&s, &set_reference);

        let mut m = fz_hash_map!(map_input.clone());
        test_map(&m, &map_reference, &map_other);
        test_map_ops(&m, &map_reference);
        test_map_iter(&m, &map_reference);
        test_map_iter_mut(&mut m, &map_reference);

        let s = fz_hash_set!(set_input.clone());
        test_set(&s, &set_reference, &set_other);
        test_set_ops(&s, &set_reference, &set_other);
        test_set_iter(&s, &set_reference);

        let mut m = fz_ordered_map!({ $( $input: (), )* });
        test_map(&m, &map_reference, &map_other);
        test_map_ops(&m, &map_reference);
        test_map_iter(&m, &map_reference);
        test_map_iter_mut(&mut m, &map_reference);

        let s = fz_ordered_set!({ $($input,)* });
        test_set(&s, &set_reference, &set_other);
        test_set_ops(&s, &set_reference, &set_other);
        test_set_iter(&s, &set_reference);

        let mut m = fz_ordered_map!(map_input.clone());
        test_map(&m, &map_reference, &map_other);
        test_map_ops(&m, &map_reference);
        test_map_iter(&m, &map_reference);
        test_map_iter_mut(&mut m, &map_reference);

        let s = fz_ordered_set!(set_input.clone());
        test_set(&s, &set_reference, &set_other);
        test_set_ops(&s, &set_reference, &set_other);
        test_set_iter(&s, &set_reference);

        let mut m = EytzingerSearchMap::new(map_input.clone());
        test_map(&m, &map_reference, &map_other);
        test_map_ops(&m, &map_reference);
        test_map_iter(&m, &map_reference);
        test_map_iter_mut(&mut m, &map_reference);

        let s = EytzingerSearchSet::new(m);
        test_set(&s, &set_reference, &set_other);
        test_set_ops(&s, &set_reference, &set_other);
        test_set_iter(&s, &set_reference);

        let mut m = BinarySearchMap::new(map_input.clone());
        test_map(&m, &map_reference, &map_other);
        test_map_ops(&m, &map_reference);
        test_map_iter(&m, &map_reference);
        test_map_iter_mut(&mut m, &map_reference);

        let s = BinarySearchSet::new(m);
        test_set(&s, &set_reference, &set_other);
        test_set_ops(&s, &set_reference, &set_other);
        test_set_iter(&s, &set_reference);

        let mut m = OrderedScanMap::new(map_input.clone());
        test_map(&m, &map_reference, &map_other);
        test_map_ops(&m, &map_reference);
        test_map_iter(&m, &map_reference);
        test_map_iter_mut(&mut m, &map_reference);

        let s = OrderedScanSet::new(m);
        test_set(&s, &set_reference, &set_other);
        test_set_ops(&s, &set_reference, &set_other);
        test_set_iter(&s, &set_reference);

        let mut m = ScanMap::new(map_input.clone());
        test_map(&m, &map_reference, &map_other);
        test_map_ops(&m, &map_reference);
        test_map_iter(&m, &map_reference);
        test_map_iter_mut(&mut m, &map_reference);

        let s = ScanSet::new(m);
        test_set(&s, &set_reference, &set_other);
        test_set_ops(&s, &set_reference, &set_other);
        test_set_iter(&s, &set_reference);

        if let Ok(mut m) = DenseScalarLookupMap::new(map_input.clone()) {
            test_map(&m, &map_reference, &map_other);
            test_map_ops(&m, &map_reference);
            test_map_iter(&m, &map_reference);
            test_map_iter_mut(&mut m, &map_reference);

            let s = DenseScalarLookupSet::new(m);
            test_set(&s, &set_reference, &set_other);
            test_set_ops(&s, &set_reference, &set_other);
            test_set_iter(&s, &set_reference);
        }

        let mut m = SparseScalarLookupMap::<_, _>::new(map_input.clone());
        test_map(&m, &map_reference, &map_other);
        test_map_ops(&m, &map_reference);
        test_map_iter(&m, &map_reference);
        test_map_iter_mut(&mut m, &map_reference);

        let s = SparseScalarLookupSet::new(m);
        test_set(&s, &set_reference, &set_other);
        test_set_ops(&s, &set_reference, &set_other);
        test_set_iter(&s, &set_reference);

        let mut m = HashMap::<_, _>::new(map_input.clone(), BridgeHasher::default()).unwrap();
        test_map(&m, &map_reference, &map_other);
        test_map_ops(&m, &map_reference);
        test_map_iter(&m, &map_reference);
        test_map_iter_mut(&mut m, &map_reference);

        let s = HashSet::new(m);
        test_set(&s, &set_reference, &set_other);
        test_set_ops(&s, &set_reference, &set_other);
        test_set_iter(&s, &set_reference);

        let mut m = FacadeOrderedMap::new(map_input.clone());
        test_map(&m, &map_reference, &map_other);
        test_map_ops(&m, &map_reference);
        test_map_iter(&m, &map_reference);
        test_map_iter_mut(&mut m, &map_reference);

        let s = FacadeOrderedSet::new(m);
        test_set(&s, &set_reference, &set_other);
        test_set_ops(&s, &set_reference, &set_other);
        test_set_iter(&s, &set_reference);

        let mut m = FacadeScalarMap::new(map_input.clone());
        test_map(&m, &map_reference, &map_other);
        test_map_ops(&m, &map_reference);
        test_map_iter(&m, &map_reference);
        test_map_iter_mut(&mut m, &map_reference);

        let s = FacadeScalarSet::new(m);
        test_set(&s, &set_reference, &set_other);
        test_set_ops(&s, &set_reference, &set_other);
        test_set_iter(&s, &set_reference);

        let mut m = FacadeHashMap::<_, _>::new(map_input.clone(), BridgeHasher::default());
        test_map(&m, &map_reference, &map_other);
        test_map_ops(&m, &map_reference);
        test_map_iter(&m, &map_reference);
        test_map_iter_mut(&mut m, &map_reference);

        let s = FacadeHashSet::new(m);
        test_set(&s, &set_reference, &set_other);
        test_set_ops(&s, &set_reference, &set_other);
        test_set_iter(&s, &set_reference);

        let m = std::collections::HashMap::<_, _, ahash::RandomState>::from_iter(map_input.clone().into_iter());
        test_map(&m, &map_reference, &map_other);

        let s = std::collections::HashSet::<_, ahash::RandomState>::from_iter(set_input.clone().into_iter());
        test_set(&s, &set_reference, &set_other);

        let m = std::collections::BTreeMap::from_iter(map_input.clone().into_iter());
        test_map(&m, &map_reference, &map_other);

        let s = std::collections::BTreeSet::from_iter(set_input.clone().into_iter());
        test_set(&s, &set_reference, &set_other);

        let m = hashbrown::HashMap::<_, _, ahash::RandomState>::from_iter(map_input.clone().into_iter());
        test_map(&m, &map_reference, &map_other);

        let s = hashbrown::HashSet::<_, ahash::RandomState>::from_iter(set_input.clone().into_iter());
        test_set(&s, &set_reference, &set_other);

        // handle String cases

        let set_reference = HashbrownSet::<&str>::from_iter(vec![ $( stringify!($input), )* ].into_iter());
        let set_other = HashbrownSet::<&str>::from_iter(vec![ $( stringify!($other), )* ].into_iter());
        let set_input = vec![ $( stringify!($input), )* ];

        let map_reference = StdHashMap::<_, _, ahash::RandomState>::from_iter(vec![ $( (stringify!($input), ()), )* ].into_iter());
        let map_other = StdHashMap::<_, _, ahash::RandomState>::from_iter(vec![ $( (stringify!($other), ()), )* ].into_iter());
        let map_input = vec![ $( (stringify!($input), ()), )* ];

        let mut m = fz_string_map!(map_input.clone());
        test_map(&m, &map_reference, &map_other);
        test_map_ops(&m, &map_reference);
        test_map_iter(&m, &map_reference);
        test_map_iter_mut(&mut m, &map_reference);

        let s = fz_string_set!(set_input.clone());
        test_set(&s, &set_reference, &set_other);
        test_set_ops(&s, &set_reference, &set_other);
        test_set_iter(&s, &set_reference);

        let mut m = FacadeStringMap::new(map_input.clone(), ahash::RandomState::default());
        test_map(&m, &map_reference, &map_other);
        test_map_ops(&m, &map_reference);
        test_map_iter(&m, &map_reference);
        test_map_iter_mut(&mut m, &map_reference);

        let s = FacadeStringSet::new(m);
        test_set(&s, &set_reference, &set_other);
        test_set_ops(&s, &set_reference, &set_other);
        test_set_iter(&s, &set_reference);
    }
}

#[test]
#[allow(clippy::unreadable_literal)]
fn test_common() {
    test_all!(1, 2, 3 ; 3, 4, 5);
    test_all!(0, 1 ; 0, 1);
    test_all!(3, 1, 2, 3, 3 ; 3, 4, 5);
    test_all!(1, 2, 3 ; 1, 2, 3, 4, 5);
    test_all!(1, 2, 3 ; 1, 2);
    test_all!(1, 2, 3 ; 2);
    test_all!(1, 2, 4 ; 2);
    test_all!(1, 2, 4 ; 3);
    test_all!(1, 2, 4, 1500 ; 3);
    test_all!(1, 2, 4, 1500 ; 2500);
    test_all!(1 ; 3);
    test_all!(1, 2 ; 3);
    test_all!(1, 2, 3 ; 3);
    test_all!(1, 2, 3, 4 ; 3);
    test_all!(1, 2, 3, 4, 5 ; 3);
    test_all!(1, 2, 3, 4, 5, 6 ; 3);
    test_all!(1, 2, 3, 4, 5, 6, 7 ; 3, 5);
    test_all!(1, 2, 3, 4, 5, 6, 7, 8 ; 3);
    test_all!(1, 2, 3, 4, 5, 6, 7, 8, 9 ; 3, 10);
    test_all!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10 ; 3);
    test_all!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10 ; 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 20);
    test_all!(11111, 11112, 11114, 11115, 111165, 111175 ; 2500, 333333333);

    // trigger the eytzinger facade code
    test_all!(
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
        110, 111, 112, 113, 114, 115, 116, 117, 118, 119,
        220, 221, 222, 223, 224, 225, 226, 227, 228, 229,
        330, 331, 332, 333, 334, 335, 336, 337, 338, 339,
        440, 441, 442, 443, 444, 445, 446, 447, 448, 449,
        550, 551, 552, 553, 554, 555, 556, 557, 558, 559,
        660, 661, 662, 663, 664, 665, 666, 667, 668, 669; 2500, 333333333);

    test_str!("1", "2", "3" ; "3", "4", "5");
    test_str!("0", "1" ; "0", "1");
    test_str!("3", "1", "2", "3", "3" ; "3", "4", "5");
    test_str!("1", "2", "3" ; "1", "2", "3", "4", "5");
    test_str!("1", "2", "3" ; "1", "2");
    test_str!("1", "2", "3" ; "2");
    test_str!("1", "2", "4" ; "2");
    test_str!("1", "2", "4" ; "3");
    test_str!("1", "2", "4", "1500" ; "3");
    test_str!("1", "2", "4", "1500" ; "2500");
    test_str!("1" ; "3");
    test_str!("1", "2" ; "3");
    test_str!("1", "2", "3" ; "3");
    test_str!("1", "2", "3", "4" ; "3");
    test_str!("1", "2", "3", "4", "5" ; "3");
    test_str!("1", "2", "3", "4", "5", "6" ; "3");
    test_str!("1", "2", "3", "4", "5", "6", "7" ; "3", "5");
    test_str!("1", "2", "3", "4", "5", "6", "7", "8" ; "3");
    test_str!("1", "2", "3", "4", "5", "6", "7", "8", "9" ; "3", "10");
    test_str!("1", "2", "3", "4", "5", "6", "7", "8", "9", "10" ; "3");
    test_str!("1", "2", "3", "4", "5", "6", "7", "8", "9", "10" ; "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "20");
    test_str!("11111", "11112", "11114", "11115", "111165", "111175" ; "2500", "333333333");
    test_str!("11111", "11112", "11114", "11115", "111165", "111175", "111185" ; "2500", "333333333");
    test_str!("1", "22", "333", "4444", "55555", "666666", "7777777" ; "2500", "333333333");
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
    test_set_default::<FacadeStringSet<&str>, &str>();
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
    test_map_default::<FacadeStringMap<&str, i32>, &str>();
}

#[test]
fn test_set_empties() {
    test_set_empty(&std::collections::HashSet::<i32>::default());
    test_set_empty(&std::collections::HashSet::<i32>::from_iter(vec![]));

    test_set_empty(&std::collections::BTreeSet::<i32>::default());
    test_set_empty(&std::collections::BTreeSet::<i32>::from_iter(vec![]));

    test_set_empty(&hashbrown::HashSet::<i32>::default());
    test_set_empty(&hashbrown::HashSet::<i32>::from_iter(vec![]));

    test_set_empty(&EytzingerSearchSet::<i32>::default());
    test_set_empty(&EytzingerSearchSet::<i32>::new(EytzingerSearchMap::new(
        vec![],
    )));

    test_set_empty(&BinarySearchSet::<i32>::default());
    test_set_empty(&BinarySearchSet::<i32>::new(BinarySearchMap::new(vec![])));

    test_set_empty(&OrderedScanSet::<i32>::default());
    test_set_empty(&OrderedScanSet::<i32>::new(OrderedScanMap::new(vec![])));

    test_set_empty(&ScanSet::<i32>::default());
    test_set_empty(&ScanSet::<i32>::new(ScanMap::new(vec![])));

    test_set_empty(&DenseScalarLookupSet::<i32>::default());
    test_set_empty(&DenseScalarLookupSet::<i32>::new(
        DenseScalarLookupMap::new(vec![]).unwrap(),
    ));

    test_set_empty(&SparseScalarLookupSet::<i32>::default());
    test_set_empty(&SparseScalarLookupSet::<i32>::new(
        SparseScalarLookupMap::new(vec![]),
    ));

    test_set_empty(&HashSet::<i32>::default());
    test_set_empty(&HashSet::<i32>::new(
        HashMap::new(vec![], BridgeHasher::default()).unwrap(),
    ));

    test_set_empty(&FacadeHashSet::<i32>::default());
    test_set_empty(&FacadeHashSet::<i32>::new(FacadeHashMap::new(
        vec![],
        BridgeHasher::default(),
    )));

    test_set_empty(&FacadeOrderedSet::<i32>::default());
    test_set_empty(&FacadeOrderedSet::<i32>::new(FacadeOrderedMap::new(vec![])));

    test_set_empty(&FacadeScalarSet::<i32>::default());
    test_set_empty(&FacadeScalarSet::<i32>::new(FacadeScalarMap::new(vec![])));

    test_set_empty(&FacadeStringSet::<&str, ahash::RandomState>::default());
    test_set_empty(&FacadeStringSet::new(FacadeStringMap::new(
        vec![],
        ahash::RandomState::default(),
    )));
}

#[test]
fn test_map_empties() {
    test_map_empty(&std::collections::HashMap::<i32, i32>::default());
    test_map_empty(&std::collections::HashMap::<i32, i32>::from_iter(vec![]));

    test_map_empty(&std::collections::BTreeMap::<i32, i32>::default());
    test_map_empty(&std::collections::BTreeMap::<i32, i32>::from_iter(vec![]));

    test_map_empty(&hashbrown::HashMap::<i32, i32>::default());
    test_map_empty(&hashbrown::HashMap::<i32, i32>::from_iter(vec![]));

    test_map_empty(&EytzingerSearchMap::<i32, i32>::default());
    test_map_empty(&EytzingerSearchMap::<i32, i32>::new(vec![]));

    test_map_empty(&BinarySearchMap::<i32, i32>::default());
    test_map_empty(&BinarySearchMap::<i32, i32>::new(vec![]));

    test_map_empty(&OrderedScanMap::<i32, i32>::default());
    test_map_empty(&OrderedScanMap::<i32, i32>::new(vec![]));

    test_map_empty(&ScanMap::<i32, i32>::default());
    test_map_empty(&ScanMap::<i32, i32>::new(vec![]));

    test_map_empty(&DenseScalarLookupMap::<i32, i32>::default());
    test_map_empty(&DenseScalarLookupMap::<i32, i32>::new(vec![]).unwrap());

    test_map_empty(&SparseScalarLookupMap::<i32, i32>::default());
    test_map_empty(&SparseScalarLookupMap::<i32, i32>::new(vec![]));

    test_map_empty(&HashMap::<i32, i32>::default());
    test_map_empty(&HashMap::<i32, i32>::new(vec![], BridgeHasher::default()).unwrap());

    test_map_empty(&FacadeHashMap::<i32, i32>::default());
    test_map_empty(&FacadeHashMap::<i32, i32>::new(
        vec![],
        BridgeHasher::default(),
    ));

    test_map_empty(&FacadeOrderedMap::<i32, i32>::default());
    test_map_empty(&FacadeOrderedMap::<i32, i32>::new(vec![]));

    test_map_empty(&FacadeScalarMap::<i32, i32>::default());
    test_map_empty(&FacadeScalarMap::<i32, i32>::new(vec![]));

    test_map_empty(&FacadeStringMap::<&str, i32, ahash::RandomState>::default());
    test_map_empty(&FacadeStringMap::<&str, i32, ahash::RandomState>::new(
        vec![],
        ahash::RandomState::default(),
    ));

    fz_hash_map!(let m: MyHashMap<i32, i32>, {});
    test_map_empty(&m);

    fz_ordered_map!(let m: MyOrderedMap<i32, i32>, {});
    test_map_empty(&m);

    fz_string_map!(let m: MyStringMap<&str, i32>, {});
    test_map_empty(&m);

    fz_scalar_map!(let m: MyScalarMap<i32, i32>, {});
    test_map_empty(&m);
}

#[test]
fn edge_cases() {
    let b = "B";
    let set_reference = HashbrownSet::from_iter(vec!["A", b, "C"]);
    let set_other = HashbrownSet::from_iter(vec!["A", b, "C"]);

    let s = fz_string_set!({ "A", b, "C", });
    test_set(&s, &set_reference, &set_other);
    test_set_ops(&s, &set_reference, &set_other);
    test_set_iter(&s, &set_reference);

    let a = 1;
    let b = 2;
    let set_reference = HashbrownSet::from_iter(vec![a, b]);
    let set_other = HashbrownSet::from_iter(vec![a, b]);

    let s = fz_scalar_set!({ a, b, });
    test_set(&s, &set_reference, &set_other);
    test_set_ops(&s, &set_reference, &set_other);
    test_set_iter(&s, &set_reference);

    let map_reference = StdHashMap::from_iter(vec![(a, 1), (b, 2), (32, 3), (42, 4), (55, 5)]);
    let map_other = StdHashMap::from_iter(vec![(a, 2), (b, 3)]);

    _ = fz_scalar_map_macro(quote!({ a:1, b:2, 32: 3, 42: 4, 55: 5}));
    let m = fz_scalar_map!({ a:1, b:2, 32: 3, 42: 4, 55: 5});
    test_map(&m, &map_reference, &map_other);
    test_map_ops(&m, &map_reference);
    test_map_iter(&m, &map_reference);
}
