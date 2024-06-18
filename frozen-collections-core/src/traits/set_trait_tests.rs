use crate::traits::Set;
use core::fmt::Debug;
use core::hash::Hash;
use hashbrown::HashSet as HashbrownSet;

pub(crate) fn test_set_trait_impl<T, ST>(
    set: &ST,
    reference: &HashbrownSet<T>,
    other: &HashbrownSet<T>,
) where
    T: Hash + Eq + Clone + Debug,
    ST: Set<T> + Debug + Clone + Default + Eq,
{
    assert_same(set, reference);

    let s2: HashbrownSet<T> = set.symmetric_difference(other).cloned().collect();
    let r2: HashbrownSet<T> = reference.symmetric_difference(other).cloned().collect();
    assert_same(&s2, &r2);

    let s2: HashbrownSet<T> = set.difference(other).cloned().collect();
    let r2: HashbrownSet<T> = reference.difference(other).cloned().collect();
    assert_same(&s2, &r2);

    let s2: HashbrownSet<T> = set.union(other).cloned().collect();
    let r2: HashbrownSet<T> = reference.union(other).cloned().collect();
    assert_same(&s2, &r2);

    let s2: HashbrownSet<T> = set.intersection(other).cloned().collect();
    let r2: HashbrownSet<T> = reference.intersection(other).cloned().collect();
    assert_same(&s2, &r2);

    assert_eq!(set.is_disjoint(other), reference.is_disjoint(other));
    assert_eq!(set.is_subset(other), reference.is_subset(other));
    assert_eq!(set.is_superset(other), reference.is_superset(other));

    let formatted_set = format!("{set:?}");
    for value in set.iter() {
        let value_str = format!("{value:?}");
        assert!(
            formatted_set.contains(&value_str),
            "Formatted string does not contain value: {value:?}"
        );
    }

    let s2 = set.clone();
    let r2 = reference.clone();
    assert_same(&s2, &r2);

    let s2 = ST::default();
    let r2 = HashbrownSet::default();
    assert_same(&s2, &r2);
}

fn assert_same<T, ST>(set: &ST, reference: &HashbrownSet<T>)
where
    T: Hash + Eq + Clone,
    ST: Set<T> + Clone,
{
    assert_eq!(set.len(), reference.len());
    assert_eq!(set.is_empty(), reference.is_empty());

    for value in reference {
        assert!(set.contains(value));
    }

    for value in set.iter() {
        assert!(reference.contains(value));
    }
}

macro_rules! test_misc_trait_impl {
    ($set_type:ty, $key_type:ty) => {
        fn test_misc_trait_impl(
            set: &$set_type,
            reference: &HashbrownSet<$key_type>,
            other: &HashbrownSet<$key_type>,
        ) {
            let s2 = set | other;
            let r2 = reference | other;
            assert_eq!(s2, r2);

            let s2 = set & other;
            let r2 = reference & other;
            assert_eq!(s2, r2);

            let s2 = set ^ other;
            let r2 = reference ^ other;
            assert_eq!(s2, r2);

            let s2 = set - other;
            let r2 = reference - other;
            assert_eq!(s2, r2);

            assert_eq!(set, reference);

            let s2 = set.clone();
            let mut r2 = reference.clone();
            for value in s2 {
                r2.remove(&value);
            }

            assert!(r2.is_empty());

            let s2 = set.clone();
            let mut r2 = reference.clone();
            for value in &s2 {
                r2.remove(value);
            }

            assert!(r2.is_empty());
        }
    };
}

pub(crate) use test_misc_trait_impl;
