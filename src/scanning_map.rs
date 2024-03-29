use std::fmt::{Debug, Formatter};

use hashbrown::Equivalent;

use crate::implementation_map::ImplementationMap;

pub(crate) struct ScanningMap<K, V, const N: usize>
where
    K: Equivalent<K>,
{
    entries: [(K, V); N],
}

impl<K, V, const N: usize> FromIterator<(K, V)> for ScanningMap<K, V, N>
where
    K: Equivalent<K>,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let v = Vec::from_iter(iter);
        Self {
            entries: v.try_into().unwrap_or_else(|_| panic!("Huh?")),
        }
    }
}

impl<K, V, const N: usize> ImplementationMap<K, V> for ScanningMap<K, V, N>
where
    K: Equivalent<K>,
{
    fn get(&self, key: &K) -> Option<&V> {
        for entry in &self.entries {
            if key.equivalent(&entry.0) {
                return Some(&entry.1);
            }
        }

        None
    }

    fn get_key_value(&self, key: &K) -> Option<(&K, &V)> {
        for entry in &self.entries {
            if key.equivalent(&entry.0) {
                return Some((&entry.0, &entry.1));
            }
        }

        None
    }

    fn contains_key(&self, key: &K) -> bool {
        for entry in &self.entries {
            if key.equivalent(&entry.0) {
                return true;
            }
        }

        false
    }

    fn len(&self) -> usize {
        self.entries.len()
    }
}

impl<K, V, const N: usize> Debug for ScanningMap<K, V, N>
where
    K: Debug + Equivalent<K>,
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let pairs = self.entries.iter().map(|x| (&x.0, &x.1));
        f.debug_map().entries(pairs).finish()
    }
}
