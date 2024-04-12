use hashbrown::Equivalent;
use std::any::TypeId;
use std::hash::Hash;
use std::mem::{transmute, ManuallyDrop};
use std::ptr;

use crate::empty_map::EmptyMap;
use crate::fallback_map::FallbackMap;
use crate::implementation_map::ImplementationMap;
use crate::integer_map::IntegerMap;
use crate::scanning_map::ScanningMap;
use crate::singleton_map::SingletonMap;
use crate::string_map::StringMap;

enum ImplementationTypes<K, V>
where
    K: Eq + Hash,
{
    Empty(EmptyMap<K, V>),
    Singleton(SingletonMap<K, V>),
    Scanning2(ScanningMap<K, V, 2>),
    Scanning3(ScanningMap<K, V, 3>),
    Scanning4(ScanningMap<K, V, 4>),
    Fallback(FallbackMap<K, V>),
    Integer32(IntegerMap<i32, V>),
    String(StringMap<V>),
}

pub struct FrozenMap<K, V>
where
    K: Eq + Hash,
{
    implementation: ImplementationTypes<K, V>,
}

impl<K, V> FrozenMap<K, V>
where
    K: Eq + Hash,
{
    fn get(&self, key: &K) -> Option<&V> {
        match &self.implementation {
            ImplementationTypes::Empty(m) => m.get(key),
            ImplementationTypes::Singleton(m) => m.get(key),
            ImplementationTypes::Scanning2(m) => m.get(key),
            ImplementationTypes::Scanning3(m) => m.get(key),
            ImplementationTypes::Scanning4(m) => m.get(key),
            ImplementationTypes::Fallback(m) => m.get(key),
            ImplementationTypes::Integer32(m) => m.get(unsafe { transmute(key) }),
            ImplementationTypes::String(m) => m.get(unsafe { transmute(key) }),
        }
    }

    fn get_key_value(&self, key: &K) -> Option<(&K, &V)> {
        match &self.implementation {
            ImplementationTypes::Empty(m) => m.get_key_value(key),
            ImplementationTypes::Singleton(m) => m.get_key_value(key),
            ImplementationTypes::Scanning2(m) => m.get_key_value(key),
            ImplementationTypes::Scanning3(m) => m.get_key_value(key),
            ImplementationTypes::Scanning4(m) => m.get_key_value(key),
            ImplementationTypes::Fallback(m) => m.get_key_value(key),
            ImplementationTypes::Integer32(m) => unsafe {
                transmute(m.get_key_value(transmute(key)))
            },
            ImplementationTypes::String(m) => unsafe { transmute(m.get_key_value(transmute(key))) },
        }
    }

    fn contains_key(&self, key: &K) -> bool {
        match &self.implementation {
            ImplementationTypes::Empty(m) => m.contains_key(key),
            ImplementationTypes::Singleton(m) => m.contains_key(key),
            ImplementationTypes::Scanning2(m) => m.contains_key(key),
            ImplementationTypes::Scanning3(m) => m.contains_key(key),
            ImplementationTypes::Scanning4(m) => m.contains_key(key),
            ImplementationTypes::Fallback(m) => m.contains_key(key),
            ImplementationTypes::Integer32(m) => m.contains_key(unsafe { transmute(key) }),
            ImplementationTypes::String(m) => m.contains_key(unsafe { transmute(key) }),
        }
    }

    fn len(&self) -> usize {
        match &self.implementation {
            ImplementationTypes::Empty(m) => m.len(),
            ImplementationTypes::Singleton(m) => m.len(),
            ImplementationTypes::Scanning2(m) => m.len(),
            ImplementationTypes::Scanning3(m) => m.len(),
            ImplementationTypes::Scanning4(m) => m.len(),
            ImplementationTypes::Fallback(m) => m.len(),
            ImplementationTypes::Integer32(m) => m.len(),
            ImplementationTypes::String(m) => m.len(),
        }
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
        } else if N == 1 {
            let iter = payload.into_iter();
            let entry = iter.last().unwrap();
            return Self {
                implementation: ImplementationTypes::Singleton(SingletonMap::<K, V>::new(
                    entry.0, entry.1,
                )),
            };
        } else if N == 2 {
            return Self {
                implementation: ImplementationTypes::Scanning2(ScanningMap::<K, V, 2>::from_iter(
                    payload,
                )),
            };
        } else if N == 3 {
            return Self {
                implementation: ImplementationTypes::Scanning3(ScanningMap::<K, V, 3>::from_iter(
                    payload,
                )),
            };
        } else if N == 4 {
            return Self {
                implementation: ImplementationTypes::Scanning4(ScanningMap::<K, V, 4>::from_iter(
                    payload,
                )),
            };
        } else if TypeId::of::<K>() == TypeId::of::<i32>() {
            // We're going to move out of the old payload, so mark it as
            // manually dropped, so we don't double-free
            let payload = ManuallyDrop::new(payload);

            // SAFETY: We know `K` is `i32` so this cast is okay
            let payload: &[(i32, V); N] = unsafe { transmute(&payload) };

            // SAFETY: We know we're reading the right type, and we're reading
            // from a ManuallyDrop, so we don't have to worry about
            // double-dropping.
            let payload = unsafe { ptr::read(payload) };

            return Self {
                implementation: ImplementationTypes::Integer32(IntegerMap::from_iter(payload)),
            };
        } else if TypeId::of::<K>() == TypeId::of::<&str>() {
            // We're going to move out of the old payload, so mark it as
            // manually dropped, so we don't double-free
            let payload = ManuallyDrop::new(payload);

            // SAFETY: We know `K` is `&str` so this cast is okay
            let payload: &[(&str, V); N] = unsafe { transmute(&payload) };

            // SAFETY: We know we're reading the right type, and we're reading
            // from a ManuallyDrop, so we don't have to worry about
            // double-dropping.
            let payload = unsafe { ptr::read(payload) };

            return Self {
                implementation: ImplementationTypes::String(StringMap::from_iter(payload)),
            };
        } else {
            Self {
                implementation: ImplementationTypes::Fallback(FallbackMap::from_iter(payload)),
            }
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
    K: Eq + Hash,
{
    fn default() -> Self {
        Self {
            implementation: ImplementationTypes::Empty(EmptyMap::<K, V>::default()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::FrozenMap;

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
}
