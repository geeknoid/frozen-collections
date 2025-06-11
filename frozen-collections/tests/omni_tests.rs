#![expect(missing_docs, reason = "Tests")]

mod common;

use common::*;
use frozen_collections::*;
use frozen_collections_core::hashers::BridgeHasher;
use frozen_collections_core::inline_maps::{InlineBinarySearchMap, InlineEytzingerSearchMap};
use frozen_collections_core::inline_sets::{InlineBinarySearchSet, InlineEytzingerSearchSet};
use frozen_collections_core::macros::fz_scalar_map_macro;
use frozen_collections_core::maps::*;
use frozen_collections_core::sets::*;
use hashbrown::HashMap as HashbrownMap;
use hashbrown::HashSet as HashbrownSet;
use quote::quote;

macro_rules! test_str {
    ( $( $input:expr ),* ; $( $other:literal ),*) => {
        // handle &str cases

        let set_reference = HashbrownSet::<&str>::from_iter(vec![ $( $input, )* ].into_iter());
        let set_other = HashbrownSet::<&str>::from_iter(vec![ $( $other, )* ].into_iter());

        let map_reference = HashbrownMap::<_, _>::from_iter(vec![ $( ($input, ()), )* ].into_iter());
        let map_other = HashbrownMap::<_, _>::from_iter(vec![ $( ($other, ()), )* ].into_iter());

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

        let map_reference = HashbrownMap::<_, _>::from_iter(vec![ $( ($input, ()), )* ].into_iter());
        let map_other = HashbrownMap::<_, _>::from_iter(vec![ $( ($other, ()), )* ].into_iter());
        let map_input = vec![ $( ($input, ()), )* ];

        let mut m = fz_scalar_map!({ $( $input: (), )* });
        test_map(&m, &map_reference, &map_other);
        test_map_ops(&m, &map_reference);
        test_map_iter(&m, &map_reference);
        test_map_iter_mut(&mut m, &map_reference);
        test_map_serialization::<_, _, _, FzScalarMap<_, _>>(&m);

        let s = fz_scalar_set!({ $($input,)* });
        test_set(&s, &set_reference, &set_other);
        test_set_ops(&s, &set_reference, &set_other);
        test_set_iter(&s, &set_reference);
        test_set_serialization::<_, _, FzScalarSet<_>>(&s);

        let mut m = fz_hash_map!({ $( $input: (), )* });
        test_map(&m, &map_reference, &map_other);
        test_map_ops(&m, &map_reference);
        test_map_iter(&m, &map_reference);
        test_map_iter_mut(&mut m, &map_reference);
        test_map_serialization::<_, _, _, FzHashMap<_, _>>(&m);

        let s = fz_hash_set!({ $($input,)* });
        test_set(&s, &set_reference, &set_other);
        test_set_ops(&s, &set_reference, &set_other);
        test_set_iter(&s, &set_reference);
        test_set_serialization::<_, _, FzHashSet<_>>(&s);

        let mut m = fz_ordered_map!({ $( $input: (), )* });
        test_map(&m, &map_reference, &map_other);
        test_map_ops(&m, &map_reference);
        test_map_iter(&m, &map_reference);
        test_map_iter_mut(&mut m, &map_reference);
        test_map_serialization::<_, _, _, FzOrderedMap<_, _>>(&m);

        let s = fz_ordered_set!({ $($input,)* });
        test_set(&s, &set_reference, &set_other);
        test_set_ops(&s, &set_reference, &set_other);
        test_set_iter(&s, &set_reference);
        test_set_serialization::<_, _, FzOrderedSet<_>>(&s);

        let mut m = EytzingerSearchMap::new(map_input.clone());
        test_map(&m, &map_reference, &map_other);
        test_map_ops(&m, &map_reference);
        test_map_iter(&m, &map_reference);
        test_map_iter_mut(&mut m, &map_reference);
        test_map_serialization::<_, _, _, FzOrderedMap<_, _>>(&m);

        let s = EytzingerSearchSet::new(m);
        test_set(&s, &set_reference, &set_other);
        test_set_ops(&s, &set_reference, &set_other);
        test_set_iter(&s, &set_reference);
        test_set_serialization::<_, _, FzOrderedSet<_>>(&s);

        let mut m = BinarySearchMap::new(map_input.clone());
        test_map(&m, &map_reference, &map_other);
        test_map_ops(&m, &map_reference);
        test_map_iter(&m, &map_reference);
        test_map_iter_mut(&mut m, &map_reference);
        test_map_serialization::<_, _, _, FzOrderedMap<_, _>>(&m);

        let s = BinarySearchSet::new(m);
        test_set(&s, &set_reference, &set_other);
        test_set_ops(&s, &set_reference, &set_other);
        test_set_iter(&s, &set_reference);
        test_set_serialization::<_, _, FzOrderedSet<_>>(&s);

        let mut m = ScanMap::new(map_input.clone());
        test_map(&m, &map_reference, &map_other);
        test_map_ops(&m, &map_reference);
        test_map_iter(&m, &map_reference);
        test_map_iter_mut(&mut m, &map_reference);
        test_map_serialization::<_, _, _, FzHashMap<_, _>>(&m);

        let s = ScanSet::new(m);
        test_set(&s, &set_reference, &set_other);
        test_set_ops(&s, &set_reference, &set_other);
        test_set_iter(&s, &set_reference);
        test_set_serialization::<_, _, FzHashSet<_>>(&s);

        if let Ok(mut m) = DenseScalarLookupMap::new(map_input.clone()) {
            test_map(&m, &map_reference, &map_other);
            test_map_ops(&m, &map_reference);
            test_map_iter(&m, &map_reference);
            test_map_iter_mut(&mut m, &map_reference);
            test_map_serialization::<_, _, _, FzScalarMap<_, _>>(&m);

            let s = DenseScalarLookupSet::new(m);
            test_set(&s, &set_reference, &set_other);
            test_set_ops(&s, &set_reference, &set_other);
            test_set_iter(&s, &set_reference);
            test_set_serialization::<_, _, FzScalarSet<_>>(&s);
        }

        let mut m = SparseScalarLookupMap::<_, _>::new(map_input.clone());
        test_map(&m, &map_reference, &map_other);
        test_map_ops(&m, &map_reference);
        test_map_iter(&m, &map_reference);
        test_map_iter_mut(&mut m, &map_reference);
        test_map_serialization::<_, _, _, FzScalarMap<_, _>>(&m);

        let s = SparseScalarLookupSet::new(m);
        test_set(&s, &set_reference, &set_other);
        test_set_ops(&s, &set_reference, &set_other);
        test_set_iter(&s, &set_reference);
        test_set_serialization::<_, _, FzScalarSet<_>>(&s);

        let mut m = HashMap::<_, _>::with_hasher(map_input.clone(), BridgeHasher::default()).unwrap();
        test_map(&m, &map_reference, &map_other);
        test_map_ops(&m, &map_reference);
        test_map_iter(&m, &map_reference);
        test_map_iter_mut(&mut m, &map_reference);
        test_map_serialization::<_, _, _, FzHashMap<_, _>>(&m);

        let s = HashSet::new(m);
        test_set(&s, &set_reference, &set_other);
        test_set_ops(&s, &set_reference, &set_other);
        test_set_iter(&s, &set_reference);
        test_set_serialization::<_, _, FzHashSet<_>>(&s);

        let mut m = FzOrderedMap::new(map_input.clone());
        test_map(&m, &map_reference, &map_other);
        test_map_ops(&m, &map_reference);
        test_map_iter(&m, &map_reference);
        test_map_iter_mut(&mut m, &map_reference);
        test_map_serialization::<_, _, _, FzOrderedMap<_, _>>(&m);

        let mut m = FzOrderedMap::from_iter(map_input.clone().into_iter());
        test_map(&m, &map_reference, &map_other);
        test_map_ops(&m, &map_reference);
        test_map_iter(&m, &map_reference);
        test_map_iter_mut(&mut m, &map_reference);
        test_map_serialization::<_, _, _, FzOrderedMap<_, _>>(&m);

        let mut m = FzOrderedMap::from([ $( ($input, ()), )* ]);
        test_map(&m, &map_reference, &map_other);
        test_map_ops(&m, &map_reference);
        test_map_iter(&m, &map_reference);
        test_map_iter_mut(&mut m, &map_reference);
        test_map_serialization::<_, _, _, FzOrderedMap<_, _>>(&m);

        let s = FzOrderedSet::from(m);
        test_set(&s, &set_reference, &set_other);
        test_set_ops(&s, &set_reference, &set_other);
        test_set_iter(&s, &set_reference);
        test_set_serialization::<_, _, FzOrderedSet<_>>(&s);

        let s = FzOrderedSet::new(set_input.clone());
        test_set(&s, &set_reference, &set_other);
        test_set_ops(&s, &set_reference, &set_other);
        test_set_iter(&s, &set_reference);
        test_set_serialization::<_, _, FzOrderedSet<_>>(&s);

        let s = FzOrderedSet::from_iter(set_input.clone().into_iter());
        test_set(&s, &set_reference, &set_other);
        test_set_ops(&s, &set_reference, &set_other);
        test_set_iter(&s, &set_reference);
        test_set_serialization::<_, _, FzOrderedSet<_>>(&s);

        let s = FzOrderedSet::from([ $( $input, )* ]);
        test_set(&s, &set_reference, &set_other);
        test_set_ops(&s, &set_reference, &set_other);
        test_set_iter(&s, &set_reference);
        test_set_serialization::<_, _, FzOrderedSet<_>>(&s);

        let mut m = FzScalarMap::new(map_input.clone());
        test_map(&m, &map_reference, &map_other);
        test_map_ops(&m, &map_reference);
        test_map_iter(&m, &map_reference);
        test_map_iter_mut(&mut m, &map_reference);
        test_map_serialization::<_, _, _, FzScalarMap<_, _>>(&m);

        let mut m = FzScalarMap::from_iter(map_input.clone().into_iter());
        test_map(&m, &map_reference, &map_other);
        test_map_ops(&m, &map_reference);
        test_map_iter(&m, &map_reference);
        test_map_iter_mut(&mut m, &map_reference);
        test_map_serialization::<_, _, _, FzOrderedMap<_, _>>(&m);

        let mut m = FzScalarMap::from([ $( ($input, ()), )* ]);
        test_map(&m, &map_reference, &map_other);
        test_map_ops(&m, &map_reference);
        test_map_iter(&m, &map_reference);
        test_map_iter_mut(&mut m, &map_reference);
        test_map_serialization::<_, _, _, FzOrderedMap<_, _>>(&m);

        let s = FzScalarSet::from(m);
        test_set(&s, &set_reference, &set_other);
        test_set_ops(&s, &set_reference, &set_other);
        test_set_iter(&s, &set_reference);
        test_set_serialization::<_, _, FzScalarSet<_>>(&s);

        let s = FzScalarSet::new(set_input.clone());
        test_set(&s, &set_reference, &set_other);
        test_set_ops(&s, &set_reference, &set_other);
        test_set_iter(&s, &set_reference);
        test_set_serialization::<_, _, FzOrderedSet<_>>(&s);

        let s = FzScalarSet::from_iter(set_input.clone().into_iter());
        test_set(&s, &set_reference, &set_other);
        test_set_ops(&s, &set_reference, &set_other);
        test_set_iter(&s, &set_reference);
        test_set_serialization::<_, _, FzOrderedSet<_>>(&s);

        let s = FzScalarSet::from([ $( $input, )* ]);
        test_set(&s, &set_reference, &set_other);
        test_set_ops(&s, &set_reference, &set_other);
        test_set_iter(&s, &set_reference);
        test_set_serialization::<_, _, FzOrderedSet<_>>(&s);

        let mut m = FzHashMap::<_, _>::new(map_input.clone());
        test_map(&m, &map_reference, &map_other);
        test_map_ops(&m, &map_reference);
        test_map_iter(&m, &map_reference);
        test_map_iter_mut(&mut m, &map_reference);
        test_map_serialization::<_, _, _, FzHashMap<_, _>>(&m);

        let mut m = FzHashMap::<_, _>::from_iter(map_input.clone().into_iter());
        test_map(&m, &map_reference, &map_other);
        test_map_ops(&m, &map_reference);
        test_map_iter(&m, &map_reference);
        test_map_iter_mut(&mut m, &map_reference);
        test_map_serialization::<_, _, _, FzOrderedMap<_, _>>(&m);

        let mut m = FzHashMap::<_, _>::from([ $( ($input, ()), )* ]);
        test_map(&m, &map_reference, &map_other);
        test_map_ops(&m, &map_reference);
        test_map_iter(&m, &map_reference);
        test_map_iter_mut(&mut m, &map_reference);
        test_map_serialization::<_, _, _, FzOrderedMap<_, _>>(&m);

        let s = FzHashSet::from(m);
        test_set(&s, &set_reference, &set_other);
        test_set_ops(&s, &set_reference, &set_other);
        test_set_iter(&s, &set_reference);
        test_set_serialization::<_, _, FzHashSet<_>>(&s);

        let s = FzHashSet::new(set_input.clone());
        test_set(&s, &set_reference, &set_other);
        test_set_ops(&s, &set_reference, &set_other);
        test_set_iter(&s, &set_reference);
        test_set_serialization::<_, _, FzOrderedSet<_>>(&s);

        let s = FzHashSet::<_>::from_iter(set_input.clone().into_iter());
        test_set(&s, &set_reference, &set_other);
        test_set_ops(&s, &set_reference, &set_other);
        test_set_iter(&s, &set_reference);
        test_set_serialization::<_, _, FzOrderedSet<_>>(&s);

        let s = FzHashSet::<_>::from([ $( $input, )* ]);
        test_set(&s, &set_reference, &set_other);
        test_set_ops(&s, &set_reference, &set_other);
        test_set_iter(&s, &set_reference);
        test_set_serialization::<_, _, FzOrderedSet<_>>(&s);

        let m = std::collections::HashMap::<_, _>::from_iter(map_input.clone().into_iter());
        test_map(&m, &map_reference, &map_other);

        let s = std::collections::HashSet::<_>::from_iter(set_input.clone().into_iter());
        test_set(&s, &set_reference, &set_other);

        let m = std::collections::BTreeMap::from_iter(map_input.clone().into_iter());
        test_map(&m, &map_reference, &map_other);

        let s = std::collections::BTreeSet::from_iter(set_input.clone().into_iter());
        test_set(&s, &set_reference, &set_other);

        let m = hashbrown::HashMap::<_, _>::from_iter(map_input.clone().into_iter());
        test_map(&m, &map_reference, &map_other);

        let s = hashbrown::HashSet::<_>::from_iter(set_input.clone().into_iter());
        test_set(&s, &set_reference, &set_other);

        // handle &str cases

        let set_reference = HashbrownSet::from_iter(vec![ $( stringify!($input).to_string().into_boxed_str(), )* ].into_iter());
        let set_other = HashbrownSet::from_iter(vec![ $( stringify!($other).to_string().into_boxed_str(), )* ].into_iter());
        let set_input = vec![ $( stringify!($input), )* ];

        let map_reference = HashbrownMap::from_iter(vec![ $( (stringify!($input).to_string().into_boxed_str(), ()), )* ].into_iter());
        let map_other = HashbrownMap::from_iter(vec![ $( (stringify!($other).to_string().into_boxed_str(), ()), )* ].into_iter());
        let map_input = vec![ $( (stringify!($input), ()), )* ];

        let mut m = FzStringMap::new(map_input.clone());
        test_map(&m, &map_reference, &map_other);
        test_map_ops(&m, &map_reference);
        test_map_iter(&m, &map_reference);
        test_map_iter_mut(&mut m, &map_reference);

        let mut m = FzStringMap::from_iter(map_input.clone().into_iter());
        test_map(&m, &map_reference, &map_other);
        test_map_ops(&m, &map_reference);
        test_map_iter(&m, &map_reference);
        test_map_iter_mut(&mut m, &map_reference);

        let s = FzStringSet::from(m);
        test_set(&s, &set_reference, &set_other);
        test_set_ops(&s, &set_reference, &set_other);
        test_set_iter(&s, &set_reference);

        let s = FzStringSet::new(set_input.clone());
        test_set(&s, &set_reference, &set_other);
        test_set_ops(&s, &set_reference, &set_other);
        test_set_iter(&s, &set_reference);

        let s = FzStringSet::from_iter(set_input.clone().into_iter());
        test_set(&s, &set_reference, &set_other);
        test_set_ops(&s, &set_reference, &set_other);
        test_set_iter(&s, &set_reference);
    }
}

#[test]
fn test_common() {
    let m = EytzingerSearchMap::new(vec![(1, 2)]);
    test_map_serialization::<_, _, _, FzOrderedMap<_, _>>(&m);

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
    test_all!(11_111, 11_112, 11_114, 11_115, 111_165, 111_175 ; 2500, 333_333_333);

    // trigger the eytzinger facade code
    test_all!(
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
        110, 111, 112, 113, 114, 115, 116, 117, 118, 119,
        220, 221, 222, 223, 224, 225, 226, 227, 228, 229,
        330, 331, 332, 333, 334, 335, 336, 337, 338, 339,
        440, 441, 442, 443, 444, 445, 446, 447, 448, 449,
        550, 551, 552, 553, 554, 555, 556, 557, 558, 559,
        660, 661, 662, 663, 664, 665, 666, 667, 668, 669; 2500, 333_333_333);

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
    test_set_default::<ScanSet<i32>, i32>();
    test_set_default::<DenseScalarLookupSet<i32>, i32>();
    test_set_default::<SparseScalarLookupSet<i32>, i32>();
    test_set_default::<HashSet<i32>, i32>();

    test_set_default::<FzHashSet<i32>, i32>();
    test_set_default::<FzOrderedSet<i32>, i32>();
    test_set_default::<FzScalarSet<i32>, i32>();

    test_set_default::<FzStringSet<Box<str>>, Box<str>>();
}

#[test]
fn test_map_defaults() {
    test_map_default::<BinarySearchMap<i32, i32>, i32>();
    test_map_default::<ScanMap<i32, i32>, i32>();
    test_map_default::<DenseScalarLookupMap<i32, i32>, i32>();
    test_map_default::<SparseScalarLookupMap<i32, i32>, i32>();
    test_map_default::<HashMap<i32, i32>, i32>();

    test_map_default::<FzHashMap<i32, i32>, i32>();
    test_map_default::<FzOrderedMap<i32, i32>, i32>();
    test_map_default::<FzScalarMap<i32, i32>, i32>();
    test_map_default::<FzStringMap<Box<str>, i32>, Box<str>>();
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
    test_set_empty(&EytzingerSearchSet::<i32>::new(EytzingerSearchMap::new(vec![])));

    test_set_empty(&BinarySearchSet::<i32>::default());
    test_set_empty(&BinarySearchSet::<i32>::new(BinarySearchMap::new(vec![])));

    test_set_empty(&ScanSet::<i32>::default());
    test_set_empty(&ScanSet::<i32>::new(ScanMap::new(vec![])));

    test_set_empty(&DenseScalarLookupSet::<i32>::default());
    test_set_empty(&DenseScalarLookupSet::<i32>::new(DenseScalarLookupMap::new(vec![]).unwrap()));

    test_set_empty(&SparseScalarLookupSet::<i32>::default());
    test_set_empty(&SparseScalarLookupSet::<i32>::new(SparseScalarLookupMap::new(vec![])));

    test_set_empty(&HashSet::<i32>::default());
    test_set_empty(&HashSet::<i32>::new(HashMap::with_hasher(vec![], BridgeHasher::default()).unwrap()));

    test_set_empty(&FzHashSet::<i32>::default());
    test_set_empty(&FzHashSet::<i32>::from(FzHashMap::new(vec![])));

    test_set_empty(&FzOrderedSet::<i32>::default());
    test_set_empty(&FzOrderedSet::<i32>::from(FzOrderedMap::new(vec![])));

    test_set_empty(&FzScalarSet::<i32>::default());
    test_set_empty(&FzScalarSet::<i32>::from(FzScalarMap::new(vec![])));

    let v: Vec<(&str, ())> = Vec::new();
    test_set_empty(&FzStringSet::<Box<str>>::default());
    test_set_empty(&FzStringSet::from(FzStringMap::new(v)));

    let v: Vec<(&str, ())> = Vec::new();
    test_set_empty(&FzStringSet::<Box<str>>::default());
    test_set_empty(&FzStringSet::from(FzStringMap::new(v)));
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

    test_map_empty(&ScanMap::<i32, i32>::default());
    test_map_empty(&ScanMap::<i32, i32>::new(vec![]));

    test_map_empty(&DenseScalarLookupMap::<i32, i32>::default());
    test_map_empty(&DenseScalarLookupMap::<i32, i32>::new(vec![]).unwrap());

    test_map_empty(&SparseScalarLookupMap::<i32, i32>::default());
    test_map_empty(&SparseScalarLookupMap::<i32, i32>::new(vec![]));

    test_map_empty(&HashMap::<i32, i32>::default());
    test_map_empty(&HashMap::<i32, i32>::with_hasher(vec![], BridgeHasher::default()).unwrap());

    test_map_empty(&FzHashMap::<i32, i32>::default());
    test_map_empty(&FzHashMap::<i32, i32>::new(vec![]));

    test_map_empty(&FzOrderedMap::<i32, i32>::default());
    test_map_empty(&FzOrderedMap::<i32, i32>::new(vec![]));

    test_map_empty(&FzScalarMap::<i32, i32>::default());
    test_map_empty(&FzScalarMap::<i32, i32>::new(vec![]));

    let v: Vec<(&str, i32)> = Vec::new();
    test_map_empty(&FzStringMap::<Box<str>, i32>::default());
    test_map_empty(&FzStringMap::<Box<str>, i32>::new(v));

    let v: Vec<(&str, i32)> = Vec::new();
    test_map_empty(&FzStringMap::<Box<str>, i32>::default());
    test_map_empty(&FzStringMap::<Box<str>, i32>::new(v));

    fz_hash_map!(let m: MyHashMap<i32, i32>, {});
    test_map_empty(&m);

    fz_ordered_map!(let m: MyOrderedMap<i32, i32>, {});
    test_map_empty(&m);

    fz_string_map!(let m: MyStringMap<&'static str, i32>, {});
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

    let map_reference = HashbrownMap::from_iter(vec![(a, 1), (b, 2), (32, 3), (42, 4), (55, 5)]);
    let map_other = HashbrownMap::from_iter(vec![(a, 2), (b, 3)]);

    _ = fz_scalar_map_macro(quote!({ a:1, b:2, 32: 3, 42: 4, 55: 5}));
    let m = fz_scalar_map!({ a:1, b:2, 32: 3, 42: 4, 55: 5});
    test_map(&m, &map_reference, &map_other);
    test_map_ops(&m, &map_reference);
    test_map_iter(&m, &map_reference);
}

#[test]
fn str_type_serialization() {
    let m1 = FzStringMap::<_, _>::from([("A", 1), ("B", 2)]);
    let json = serde_json::to_string(&m1).unwrap();
    let m2: FzStringMap<Box<str>, i32> = serde_json::from_str(&json).unwrap();
    assert_eq_map(&m1, &m2);

    let s1 = FzStringSet::<_>::from(["A", "B"]);
    let json = serde_json::to_string(&s1).unwrap();
    let s2: FzStringSet<Box<str>> = serde_json::from_str(&json).unwrap();
    assert_eq_set(&s1, &s2);

    let m: serde_json::Result<FzStringMap<Box<str>, i32>> = serde_json::from_str("[\"123\": 2]");
    assert!(m.is_err());

    let s: serde_json::Result<FzStringSet<Box<str>>> = serde_json::from_str("{XXX: XXX,}");
    assert!(s.is_err());
}

#[test]
fn string_type_serialization() {
    let m1 = FzStringMap::<_, _>::from([("A".to_string(), 1), ("B".to_string(), 2)]);
    let json = serde_json::to_string(&m1).unwrap();
    let m2: FzStringMap<Box<str>, i32> = serde_json::from_str(&json).unwrap();
    assert_eq_map(&m1, &m2);

    let s1 = FzStringSet::<_>::from(["A".to_string(), "B".to_string()]);
    let json = serde_json::to_string(&s1).unwrap();
    let s2: FzStringSet<Box<str>> = serde_json::from_str(&json).unwrap();
    assert_eq_set(&s1, &s2);

    let m: serde_json::Result<FzStringMap<Box<str>, i32>> = serde_json::from_str("[\"123\": 2]");
    assert!(m.is_err());

    let s: serde_json::Result<FzStringSet<Box<str>>> = serde_json::from_str("{XXX: XXX,}");
    assert!(s.is_err());
}

#[test]
fn binary_search() {
    let mut m = InlineBinarySearchMap::<i32, (), 10>::new_raw([
        (1, ()),
        (2, ()),
        (3, ()),
        (4, ()),
        (5, ()),
        (6, ()),
        (7, ()),
        (8, ()),
        (9, ()),
        (10, ()),
    ]);
    let map_reference = HashbrownMap::<i32, ()>::from_iter(m.clone());
    let map_other = HashbrownMap::<i32, ()>::from_iter(m.clone());

    test_map(&m, &map_reference, &map_other);
    test_map_ops(&m, &map_reference);
    test_map_iter(&m, &map_reference);
    test_map_iter_mut(&mut m, &map_reference);
    test_map_serialization::<_, _, _, FzOrderedMap<_, _>>(&m);

    let s = InlineBinarySearchSet::<i32, 10>::new(m);
    let set_reference = HashbrownSet::<i32>::from_iter(s.clone());
    let set_other = HashbrownSet::<i32>::from_iter(s.clone());

    test_set(&s, &set_reference, &set_other);
    test_set_iter(&s, &set_reference);
    test_set_ops(&s, &set_reference, &set_other);
    test_set_serialization::<_, _, FzOrderedSet<_>>(&s);
}

#[test]
fn eytzinger_search() {
    let mut m = InlineEytzingerSearchMap::<i32, (), 10>::new_raw([
        (7, ()),
        (4, ()),
        (9, ()),
        (2, ()),
        (6, ()),
        (8, ()),
        (10, ()),
        (1, ()),
        (3, ()),
        (5, ()),
    ]);
    let map_reference = HashbrownMap::<i32, ()>::from_iter(m.clone());
    let map_other = HashbrownMap::<i32, ()>::from_iter(m.clone());

    test_map(&m, &map_reference, &map_other);
    test_map_ops(&m, &map_reference);
    test_map_iter(&m, &map_reference);
    test_map_iter_mut(&mut m, &map_reference);
    test_map_serialization::<_, _, _, FzOrderedMap<_, _>>(&m);

    let s = InlineEytzingerSearchSet::<i32, 10>::new(m);
    let set_reference = HashbrownSet::<i32>::from_iter(s.clone());
    let set_other = HashbrownSet::<i32>::from_iter(s.clone());

    test_set(&s, &set_reference, &set_other);
    test_set_iter(&s, &set_reference);
    test_set_ops(&s, &set_reference, &set_other);
    test_set_serialization::<_, _, FzOrderedSet<_>>(&s);
}
