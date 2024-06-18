use crate::traits::Map;
use core::fmt::Debug;
use std::collections::HashMap as StdHashMap;
use std::collections::HashSet as StdHashSet;
use std::hash::Hash;
use std::ops::Index;

pub fn test_map_trait_impl<'a, K, V, MT>(
    map: &MT,
    reference: &StdHashMap<K, V>,
    other: &'a StdHashMap<K, V>,
) where
    K: 'a + Hash + Eq + Clone + Debug,
    V: Clone + Debug + Eq + Hash,
    MT: Map<K, V> + Debug + Clone + Default + Eq + Index<&'a K, Output = V>,
{
    assert_same(map, reference);

    let formatted_map = format!("{map:?}");
    for key in map.keys() {
        let key_str = format!("{key:?}");
        assert!(
            formatted_map.contains(&key_str),
            "Formatted string does not contain key: {key:?}"
        );
    }

    let m2 = map.clone();
    let r2 = reference.clone();
    assert_same(&m2, &r2);

    let m2 = MT::default();
    let r2 = StdHashMap::default();
    assert_same(&m2, &r2);

    let v1: StdHashMap<K, V> = map.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
    let v2: StdHashMap<K, V> = reference
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    assert_eq!(v1, v2);

    let v1: StdHashMap<K, V> = map
        .clone()
        .iter_mut()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    let v2: StdHashMap<K, V> = reference
        .clone()
        .iter_mut()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    assert_eq!(v1, v2);

    let v1: StdHashMap<K, V> = map.clone().into_iter().collect();
    let v2: StdHashMap<K, V> = reference.clone().into_iter().collect();
    assert_eq!(v1, v2);

    let v1: StdHashSet<K> = map.keys().cloned().collect();
    let v2: StdHashSet<K> = reference.keys().cloned().collect();
    assert_eq!(v1, v2);

    let v1: StdHashSet<K> = map.clone().into_keys().collect();
    let v2: StdHashSet<K> = reference.clone().into_keys().collect();
    assert_eq!(v1, v2);

    let v1: StdHashSet<V> = map.values().cloned().collect();
    let v2: StdHashSet<V> = reference.values().cloned().collect();
    assert_eq!(v1, v2);

    let v1: StdHashSet<V> = map.clone().values_mut().map(|v| v.clone()).collect();
    let v2: StdHashSet<V> = reference.clone().values_mut().map(|v| v.clone()).collect();
    assert_eq!(v1, v2);

    let v1: StdHashSet<V> = map.clone().into_values().collect();
    let v2: StdHashSet<V> = reference.clone().into_values().collect();
    assert_eq!(v1, v2);

    for pair in other {
        assert_eq!(map.get(pair.0), reference.get(pair.0));
        assert_eq!(map.get_key_value(pair.0), reference.get_key_value(pair.0));
        assert_eq!(
            map.clone().get_mut(pair.0),
            reference.clone().get_mut(pair.0)
        );

        if map.contains_key(pair.0) {
            assert_eq!(map.index(pair.0), reference.index(pair.0));
        }
    }

    if map.len() >= 2 {
        let keys: Vec<_> = map.keys().collect();
        let mut cloned_map = map.clone();
        let values_from_map = cloned_map.get_many_mut([keys[0], keys[1]]).unwrap();

        assert_eq!(values_from_map[0], &reference[keys[0]]);
        assert_eq!(values_from_map[1], &reference[keys[1]]);
    }
}

fn assert_same<K, V, MT>(map: &MT, reference: &StdHashMap<K, V>)
where
    K: Hash + Eq + Clone + Debug,
    V: Clone + Eq + Debug,
    MT: Map<K, V> + Clone + IntoIterator<Item = (K, V)>,
{
    assert_eq!(map.len(), reference.len());
    assert_eq!(map.is_empty(), reference.is_empty());

    for pairs in reference {
        assert!(map.contains_key(pairs.0));
    }

    for pairs in map.iter() {
        assert!(reference.contains_key(pairs.0));
    }
}
