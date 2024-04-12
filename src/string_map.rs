use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

pub(crate) struct StringMap<K, V> {
    entries: HashMap<K, V>,
}

impl<K, V> FromIterator<(K, V)> for StringMap<K, V>
where
    K: Eq + std::hash::Hash,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut entries = HashMap::new();
        entries.extend(iter);

        Self { entries }
    }
}

impl<K, V> StringMap<K, V>
where
    K: Eq + std::hash::Hash,
{
    pub fn get(&self, key: &K) -> Option<&V> {
        self.entries.get(key)
    }

    pub fn get_key_value(&self, key: &K) -> Option<(&K, &V)> {
        self.entries.get_key_value(key)
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.entries.contains_key(key)
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

impl<'a, K, V> Debug for StringMap<K, V>
where
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let pairs = self.entries.iter().map(|x| (x.0, x.1));
        f.debug_map().entries(pairs).finish()
    }
}
