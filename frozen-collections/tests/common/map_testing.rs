use core::fmt::Debug;
use frozen_collections_core::traits::Map;
use std::collections::HashMap as StdHashMap;
use std::collections::HashSet as StdHashSet;
use std::hash::Hash;
use std::ops::Index;

pub fn test_map<MT, K, V>(
    map: &MT,
    reference: &StdHashMap<K, V, ahash::RandomState>,
    other: &StdHashMap<K, V, ahash::RandomState>,
) where
    K: Hash + Eq + Clone + Debug + Default,
    V: Hash + Eq + Clone + Debug + Default,
    MT: Map<K, V> + Debug + Clone + Eq,
{
    assert_same(map, reference);

    let formatted_map = format!("{map:?}");
    for key in map.keys() {
        let key_str = format!("{key:?}");
        assert!(formatted_map.contains(&key_str));
    }

    let m2 = map.clone();
    let r2 = reference.clone();
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

    let v1: StdHashSet<&K> = map.keys().collect();
    let v2: StdHashSet<&K> = reference.keys().collect();
    assert_eq!(v1, v2);

    let v1: StdHashSet<K> = map.clone().into_keys().collect();
    let v2: StdHashSet<K> = reference.clone().into_keys().collect();
    assert_eq!(v1, v2);

    let v1: StdHashSet<&V> = map.values().collect();
    let v2: StdHashSet<&V> = reference.values().collect();
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
    }

    if map.len() >= 2 {
        let keys: Vec<_> = map.keys().collect();
        let mut cloned_map = map.clone();
        let values_from_map = cloned_map.get_many_mut([keys[0], keys[1]]).unwrap();

        assert_eq!(values_from_map[0], &reference[keys[0]]);
        assert_eq!(values_from_map[1], &reference[keys[1]]);
    }
}

pub fn test_map_default<MT, K>()
where
    K: Hash + Eq + Clone + Debug + Default,
    MT: Map<K, i32> + Debug + Clone + Default + Eq,
{
    let m = MT::default();
    let r = StdHashMap::default();
    assert_same(&m, &r);
    assert!(!m.contains_key(&K::default()));
    assert_eq!(0, m.len());
    assert!(m.is_empty());
}

pub fn test_map_ops<'a, MT, K, V>(map: &'a MT, reference: &'a StdHashMap<K, V, ahash::RandomState>)
where
    K: 'a + Hash + Eq,
    V: 'a + Hash + Eq + Debug,
    MT: 'a
        + Map<K, V>
        + Debug
        + Clone
        + PartialEq<StdHashMap<K, V, ahash::RandomState>>
        + Index<&'a K, Output = V>,
{
    assert!(map.eq(reference));

    if !map.is_empty() {
        assert!(!map.eq(&StdHashMap::default()));

        for pair in reference {
            assert_eq!(&map[pair.0], pair.1);
        }
    }
}

#[allow(clippy::needless_pass_by_ref_mut)]
pub fn test_map_iter<'a, MT, K, V>(
    map: &'a mut MT,
    reference: &'a StdHashMap<K, V, ahash::RandomState>,
) where
    K: 'a + Hash + Eq + Clone + Debug,
    V: Eq,
    MT: 'a + Map<K, V> + Debug + Clone,
    &'a MT: IntoIterator<Item = (&'a K, &'a V)>,
    &'a mut MT: IntoIterator<Item = (&'a K, &'a mut V)>,
{
    let cloned = map.clone();

    // operates on a MT
    assert_eq!(map.len(), map.clone().into_iter().count());
    for pair in map.clone() {
        assert!(map.contains_key(&pair.0));
    }

    // operates on a &MT
    let m: &MT = map;
    assert_eq!(m.len(), m.iter().count());
    for pair in m {
        assert!(reference.contains_key(pair.0));
    }

    // operates on a &mut MT
    assert_eq!(map.len(), map.iter().count());
    for pair in map.iter() {
        assert!(reference.contains_key(pair.0));
    }

    // operates on a &mut MT
    assert_eq!(cloned.len(), cloned.clone().into_iter().count());
    for pair in cloned {
        assert!(reference.contains_key(&pair.0));
    }
}

fn assert_same<K, V, MT>(map: &MT, reference: &StdHashMap<K, V, ahash::RandomState>)
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
