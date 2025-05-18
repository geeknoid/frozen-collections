use core::fmt::Debug;
use frozen_collections_core::traits::Map;
use hashbrown::HashMap as HashbrownMap;
use hashbrown::HashSet as HashbrownSet;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::hash::Hash;
use std::ops::Index;
use std::panic;

pub fn test_map<MT, K, V>(map: &MT, reference: &HashbrownMap<K, V>, other: &HashbrownMap<K, V>)
where
    K: Hash + Eq + Clone + Debug + Default,
    V: Hash + Eq + Clone + Debug + Default,
    MT: Map<K, V> + Debug + Clone + Eq + Serialize + std::panic::RefUnwindSafe,
{
    assert_eq_map(map, reference);

    let formatted_map = format!("{map:?}");
    for key in map.keys() {
        let key_str = format!("{key:?}");
        assert!(formatted_map.contains(&key_str));
    }

    let m2 = map.clone();
    let r2 = reference.clone();
    assert_eq_map(&m2, &r2);

    let v1: HashbrownMap<K, V> = map.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
    let v2: HashbrownMap<K, V> = reference
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    assert_eq!(v1, v2);

    let v1: HashbrownMap<K, V> = map
        .clone()
        .iter_mut()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    let v2: HashbrownMap<K, V> = reference
        .clone()
        .iter_mut()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    assert_eq!(v1, v2);

    let v1: HashbrownMap<K, V> = map.clone().into_iter().collect();
    let v2: HashbrownMap<K, V> = reference.clone().into_iter().collect();
    assert_eq!(v1, v2);

    let v1: HashbrownSet<&K> = map.keys().collect();
    let v2: HashbrownSet<&K> = reference.keys().collect();
    assert_eq!(v1, v2);

    let v1: HashbrownSet<K> = map.clone().into_keys().collect();
    let v2: HashbrownSet<K> = reference.clone().into_keys().collect();
    assert_eq!(v1, v2);

    let v1: HashbrownSet<&V> = map.values().collect();
    let v2: HashbrownSet<&V> = reference.values().collect();
    assert_eq!(v1, v2);

    let v1: HashbrownSet<V> = map.clone().values_mut().map(|v| v.clone()).collect();
    let v2: HashbrownSet<V> = reference.clone().values_mut().map(|v| v.clone()).collect();
    assert_eq!(v1, v2);

    let v1: HashbrownSet<V> = map.clone().into_values().collect();
    let v2: HashbrownSet<V> = reference.clone().into_values().collect();
    assert_eq!(v1, v2);

    for pair in other {
        assert_eq!(map.contains_key(pair.0), reference.contains_key(pair.0));
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
        let mut values_from_map = cloned_map.get_disjoint_mut([keys[0], keys[1]]);

        let v0 = values_from_map[0].take().unwrap();
        let v1 = values_from_map[1].take().unwrap();

        assert_eq!(v0, &reference[keys[0]]);
        assert_eq!(v1, &reference[keys[1]]);

        let h = panic::take_hook();
        panic::set_hook(Box::new(|_info| {
            // do nothing
        }));

        let err = panic::catch_unwind(|| {
            let keys: Vec<_> = map.keys().collect();
            let mut cloned_map = map.clone();
            _ = cloned_map.get_disjoint_mut([keys[0], keys[0]]);
        })
        .unwrap_err();

        panic::set_hook(h);

        assert_eq!(
            err.downcast_ref::<&'static str>().unwrap(),
            &"duplicate keys found"
        );

        {
            let keys: Vec<_> = map.keys().collect();
            let mut cloned_map = map.clone();
            unsafe { _ = cloned_map.get_disjoint_unchecked_mut([keys[0], keys[0]]) };
        }
    }
}

pub fn test_map_default<MT, K>()
where
    K: Hash + Eq + Clone + Debug + Default,
    MT: Map<K, i32> + Debug + Clone + Default + Eq,
{
    let m = MT::default();
    let r = HashbrownMap::<_, _>::default();
    assert_eq_map(&m, &r);
    assert!(!m.contains_key(&K::default()));
    assert_eq!(0, m.len());
    assert!(m.is_empty());
}

pub fn test_map_ops<'a, MT, K, V>(map: &'a MT, reference: &'a HashbrownMap<K, V>)
where
    K: 'a + Hash + Eq,
    V: 'a + Hash + Eq + Debug,
    MT: 'a + Map<K, V> + Debug + Clone + PartialEq<HashbrownMap<K, V>> + Index<&'a K, Output = V>,
{
    assert!(map.eq(reference));

    if !map.is_empty() {
        assert!(!map.eq(&HashbrownMap::default()));

        for pair in reference {
            assert_eq!(&map[pair.0], pair.1);
        }
    }
}

pub fn test_map_iter<'a, MT, K, V>(map: &'a MT, reference: &'a HashbrownMap<K, V>)
where
    K: 'a + Hash + Eq + Clone + Debug,
    V: Eq,
    MT: 'a + Map<K, V> + Debug + Clone,
    &'a MT: IntoIterator<Item = (&'a K, &'a V)>,
{
    // operates on an &MT
    assert_eq!(map.len(), map.iter().count());
    for pair in map.iter() {
        assert!(reference.contains_key(pair.0));
    }

    // operates on an &MT
    assert_eq!(map.len(), map.into_iter().count());
    for pair in map {
        assert!(reference.contains_key(pair.0));
    }

    // operates on an MT
    assert_eq!(map.len(), map.clone().into_iter().count());
    for pair in map.clone() {
        assert!(map.contains_key(&pair.0));
    }
}

pub fn test_map_iter_mut<'a, MT, K, V>(map: &'a mut MT, reference: &'a HashbrownMap<K, V>)
where
    K: 'a + Hash + Eq + Clone + Debug,
    V: Eq,
    MT: 'a + Map<K, V> + Debug + Clone,
    &'a mut MT: IntoIterator<Item = (&'a K, &'a mut V)>,
{
    // operates on a &mut MT
    for pair in map {
        assert!(reference.contains_key(pair.0));
    }
}

pub fn test_map_empty<MT, K, V>(map: &MT)
where
    MT: Map<K, V>,
    K: Default,
{
    assert_eq!(0, map.len());
    assert!(map.is_empty());
    assert_eq!(0, map.iter().count());
    assert!(!map.contains_key(&K::default()));
}

pub fn test_map_serialization<K, V, MT, MT2>(map: &MT)
where
    K: Hash + Eq + Clone + Debug + Default,
    V: Hash + Eq + Clone + Debug + Default,
    MT: Map<K, V> + Debug + Clone + Eq + Serialize,
    MT2: Map<K, V> + Debug + Clone + Eq + DeserializeOwned,
{
    let json = serde_json::to_string(&map).unwrap();
    let map2: MT2 = serde_json::from_str(&json).unwrap();
    assert_eq_map(map, &map2);

    let map2: serde_json::Result<MT2> = serde_json::from_str("[\"123\": 2]");
    assert!(map2.is_err());
}

pub fn assert_eq_map<K, V, MT, MT2>(map: &MT, reference: &MT2)
where
    K: Hash + Eq + Clone + Debug,
    V: Clone + Eq + Debug,
    MT: Map<K, V> + Clone + IntoIterator<Item = (K, V)>,
    MT2: Map<K, V> + Clone + IntoIterator<Item = (K, V)>,
{
    assert_eq!(map.len(), reference.len());
    assert_eq!(map.is_empty(), reference.is_empty());

    for pair in reference.iter() {
        assert!(map.contains_key(pair.0));
    }

    for pair in map.iter() {
        assert!(reference.contains_key(pair.0));
    }
}
