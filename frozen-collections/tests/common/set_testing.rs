use core::fmt::Debug;
use core::hash::Hash;
use core::ops::{BitAnd, BitOr, BitXor, Sub};
use frozen_collections::{Set, SetOps};
use hashbrown::HashSet as HashbrownSet;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub fn test_set<ST, T>(set: &ST, reference: &HashbrownSet<T>, other: &HashbrownSet<T>)
where
    T: Hash + Eq + Clone + Debug + Default,
    ST: Set<T> + Debug + Clone + Serialize,
{
    assert_eq_set(set, reference);

    let s2: HashbrownSet<&T> = set.symmetric_difference(other).collect();
    let r2: HashbrownSet<&T> = reference.symmetric_difference(other).collect();
    assert_eq_set(&s2, &r2);

    let s2: HashbrownSet<&T> = set.difference(other).collect();
    let r2: HashbrownSet<&T> = reference.difference(other).collect();
    assert_eq_set(&s2, &r2);

    let s2: HashbrownSet<&T> = set.union(other).collect();
    let r2: HashbrownSet<&T> = reference.union(other).collect();
    assert_eq_set(&s2, &r2);

    let s2: HashbrownSet<&T> = set.intersection(other).collect();
    let r2: HashbrownSet<&T> = reference.intersection(other).collect();
    assert_eq_set(&s2, &r2);

    assert_eq!(set.is_disjoint(other), reference.is_disjoint(other));
    assert_eq!(set.is_subset(other), reference.is_subset(other));
    assert_eq!(set.is_superset(other), reference.is_superset(other));

    let formatted_set = format!("{set:?}");
    for value in set.iter() {
        let value_str = format!("{value:?}");
        assert!(formatted_set.contains(&value_str));
    }

    // for now, we just effectively test that this doesn't crash
    let j = serde_json::to_string(&set).unwrap();
    assert!(!j.is_empty());

    //    let s2: HashbrownSet<T> = serde_json::from_str(&j).unwrap();
    //    assert_same(set, &s2);

    let s2 = set.clone();
    let r2 = reference.clone();
    assert_eq_set(&s2, &r2);

    let s2 = set.clone();
    let mut r2 = reference.clone();
    for value in s2 {
        r2.remove(&value);
    }

    assert!(r2.is_empty());
}

pub fn test_set_default<ST, T>()
where
    T: Hash + Eq + Clone + Debug + Default,
    ST: Set<T> + Debug + Clone + Default,
{
    let s = ST::default();
    let r = HashbrownSet::<_>::default();
    assert_eq_set(&s, &r);
    assert!(!s.contains(&T::default()));
    assert_eq!(0, s.len());
    assert!(s.is_empty());
}

pub fn test_set_ops<'a, ST, T>(
    set: &'a ST,
    reference: &'a HashbrownSet<T>,
    other: &'a HashbrownSet<T>,
) where
    T: 'a + Hash + Eq + Clone + Debug,
    ST: 'a + Set<T> + Debug + Clone + PartialEq<HashbrownSet<T>>,
    &'a ST: BitOr<&'a HashbrownSet<T>, Output = HashbrownSet<T>>
        + BitAnd<&'a HashbrownSet<T>, Output = HashbrownSet<T>>
        + BitXor<&'a HashbrownSet<T>, Output = HashbrownSet<T>>
        + Sub<&'a HashbrownSet<T>, Output = HashbrownSet<T>>,
{
    assert!(set.eq(reference));

    if !set.is_empty() {
        assert!(!set.eq(&HashbrownSet::default()));
    }

    let s2 = set.bitor(other);
    let r2 = reference.bitor(other);
    assert_eq!(&s2, &r2);
    assert!(s2.eq(&r2));

    let s2 = set.bitand(other);
    let r2 = reference.bitand(other);
    assert_eq!(s2, r2);
    assert!(s2.eq(&r2));

    let s2 = set.bitxor(other);
    let r2 = reference.bitxor(other);
    assert_eq!(s2, r2);
    assert!(s2.eq(&r2));

    let s2 = set.sub(other);
    let r2 = reference.sub(other);
    assert_eq!(s2, r2);
    assert!(s2.eq(&r2));
}

pub fn test_set_iter<'a, ST, T>(set: &'a ST, reference: &'a HashbrownSet<T>)
where
    T: 'a + Hash + Eq + Clone + Debug,
    ST: 'a + Set<T> + Debug + Clone,
    &'a ST: IntoIterator<Item = &'a T>,
{
    // operates on an &ST
    assert_eq!(set.len(), set.iter().count());
    for v in set.iter() {
        assert!(reference.contains(v));
    }

    // operates on an &ST
    assert_eq!(set.len(), set.into_iter().count());
    for v in set {
        assert!(reference.contains(v));
    }

    // operates on an ST
    assert_eq!(set.len(), set.clone().into_iter().count());
    for v in set.clone() {
        assert!(set.contains(&v));
    }
}

pub fn test_set_empty<ST, T>(set: &ST)
where
    ST: Set<T>,
    T: Default,
{
    assert_eq!(0, set.len());
    assert!(set.is_empty());
    assert_eq!(0, set.iter().count());
    assert!(!set.contains(&T::default()));
}

pub fn test_set_serialization<T, ST, ST2>(set: &ST)
where
    T: Hash + Eq + Clone + Debug + Default,
    ST: Set<T> + Debug + Clone + Eq + Serialize,
    ST2: Set<T> + Debug + Clone + Eq + DeserializeOwned,
{
    let json = serde_json::to_string(&set).unwrap();
    let set2: ST2 = serde_json::from_str(&json).unwrap();
    assert_eq_set(set, &set2);

    let set2: serde_json::Result<ST2> = serde_json::from_str("{XXX: XXX,}");
    assert!(set2.is_err());
}

pub fn assert_eq_set<T, ST, ST2>(set: &ST, reference: &ST2)
where
    T: Hash + Eq + Clone,
    ST: Set<T> + Clone,
    ST2: Set<T> + Clone,
{
    assert_eq!(set.len(), reference.len());
    assert_eq!(set.is_empty(), reference.is_empty());

    for value in reference.iter() {
        assert!(set.contains(value));
    }

    for value in set.iter() {
        assert!(reference.contains(value));
    }
}
