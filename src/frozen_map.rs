use std::any::TypeId;
use std::hash::Hash;
use std::mem::{transmute, ManuallyDrop};
use std::ptr;

use hashbrown::Equivalent;

use crate::empty_map::EmptyMap;
use crate::fallback_map::FallbackMap;
use crate::implementation_map::{self, ImplementationMap};
use crate::integer_map::IntegerMap;
use crate::scanning_map::ScanningMap;
use crate::singleton_map::SingletonMap;

pub struct FrozenMap<K, V> {
    implementation: Box<dyn ImplementationMap<K, V>>,
}

impl<K, V> FrozenMap<K, V> {
    fn get(&self, key: &K) -> Option<&V> {
        self.implementation.get(key)
    }

    fn get_key_value(&self, key: &K) -> Option<(&K, &V)> {
        self.implementation.get_key_value(key)
    }

    fn contains_key(&self, key: &K) -> bool {
        self.implementation.contains_key(key)
    }

    fn len(&self) -> usize {
        self.implementation.len()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn capacity(&self) -> usize {
        self.len()
    }
}

impl<K, V: 'static, const N: usize> From<[(K, V); N]> for FrozenMap<K, V>
where
    K: Eq + Hash + Equivalent<K> + 'static,
{
    fn from(payload: [(K, V); N]) -> FrozenMap<K, V> {
        if N == 0 {
            return FrozenMap::default();
        }

        if N == 1 {
            let iter = payload.into_iter();
            let entry = iter.last().unwrap();
            return Self {
                implementation: Box::new(SingletonMap::<K, V>::new(entry.0, entry.1)),
            };
        }

        if N < 4 {
            return Self {
                implementation: Box::new(ScanningMap::<K, V, N>::from_iter(payload)),
            };
        }

        if TypeId::of::<K>() == TypeId::of::<i32>() {
            // We're going to move out of the old payload, so mark it as
            // manually drop so we don't double-free
            let payload = ManuallyDrop::new(payload);
            // SAFETY: We know `K` is `i32` so this cast is okay
            let payload: &[(i32, V); N] = unsafe { transmute(&payload) };
            // SAFETY: We know we're reading the right type, and we're reading
            // from a ManuallyDrop so we don't have to worry about
            // double-dropping.
            let payload = unsafe { ptr::read(payload) };
            let implementation =
                Box::new(IntegerMap::from_iter(payload)) as Box<dyn ImplementationMap<i32, _>>;
            // SAFETY: We know K == i32, so we're undoing the first transmute.
            let implementation = unsafe { transmute(implementation) };

            return Self { implementation };
        }

        Self {
            implementation: Box::new(FallbackMap::from_iter(payload)),
        }
    }
}

impl<K, V> FromIterator<(K, V)> for FrozenMap<K, V>
where
    K: Eq + Hash + Equivalent<K>,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        todo!()
    }
}

impl<K, V: 'static> Default for FrozenMap<K, V>
where
    K: Hash + Equivalent<K> + 'static,
{
    fn default() -> Self {
        Self {
            implementation: Box::<EmptyMap<K, V>>::default(),
        }
    }
}

#[test]
fn test_empty_map() {
    type FM = FrozenMap<i32, i32>;

    let m = FM::default();
    assert_eq!(m.len(), 0);
}

#[test]
fn test_i32_map() {
    let m = FrozenMap::<i32, i32>::from([(1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6)]);
    assert_eq!(m.get(&6), Some(&6));
}

/*
#[test]
fn test_debug() {
    type HM = HashMap<i32, i32>;
    type FM = FrozenMap<i32, i32>;

    let fm = FM::from([]);
    let fs = format!("{:?}", fm);

    let hm = HM::from([]);
    let hs = format!("{:?}", hm);

    println!("{}", fs);
    format!("{:?}", fm);

    println!("{}", hs);
    format!("{:?}", hm);
}

#[test]
fn test_small_inline_map() {
    type FM = FrozenMap<i32, i32>;

    let m = FM::from([(1, 2), (3, 4), (5, 6)]);
    assert_eq!(m.len(), 3);

    let v = m.get(&3);
    assert_eq!(v.unwrap(), &4);
}

#[test]
fn test_small_dynamic_map() {
    type FM = FrozenMap<i32, i32>;

    let m = FM::from_iter([(1, 2), (3, 4), (5, 6)]);
    assert_eq!(m.len(), 3);

    let v = m.get(&3);
    assert_eq!(v.unwrap(), &4);
}
*/
