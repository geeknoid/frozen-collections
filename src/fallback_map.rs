use crate::implementation_map::ImplementationMap;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::hash::Hash;

pub(crate) struct FallbackMap<K, V>
where
    K: Eq + Hash,
{
    entries: HashMap<K, V>,
}

impl<K, V> FromIterator<(K, V)> for FallbackMap<K, V>
where
    K: Eq + Hash,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut entries = HashMap::new();
        entries.extend(iter);

        Self { entries }
    }
}

impl<K, V> ImplementationMap<K, V> for FallbackMap<K, V>
where
    K: Eq + Hash,
{
    fn get(&self, key: &K) -> Option<&V> {
        self.entries.get(key)
    }

    fn get_key_value(&self, key: &K) -> Option<(&K, &V)> {
        self.entries.get_key_value(key)
    }

    fn contains_key(&self, key: &K) -> bool {
        self.entries.contains_key(key)
    }

    fn len(&self) -> usize {
        self.entries.len()
    }
}

impl<K, V> Debug for FallbackMap<K, V>
where
    K: Debug + Eq + Hash,
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let pairs = self.entries.iter().map(|x| (x.0, x.1));
        f.debug_map().entries(pairs).finish()
    }
}
