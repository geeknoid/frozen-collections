use std::fmt::{Debug, Formatter};

use hashbrown::Equivalent;

pub(crate) struct SingletonMap<K, V>
where
    K: Equivalent<K>,
{
    key: K,
    value: V,
}

impl<K, V> SingletonMap<K, V>
where
    K: Equivalent<K>,
{
    pub fn new(key: K, value: V) -> Self {
        Self { key, value }
    }
}

impl<K, V> SingletonMap<K, V>
where
    K: Equivalent<K>,
{
    pub fn get(&self, key: &K) -> Option<&V> {
        if key.equivalent(&self.key) {
            return Some(&self.value);
        }

        None
    }

    pub fn get_key_value(&self, key: &K) -> Option<(&K, &V)> {
        if key.equivalent(&self.key) {
            return Some((&self.key, &self.value));
        }

        None
    }

    pub fn contains_key(&self, key: &K) -> bool {
        key.equivalent(&self.key)
    }

    pub fn len(&self) -> usize {
        1
    }
}

impl<K, V> Debug for SingletonMap<K, V>
where
    K: Equivalent<K> + Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries([(&self.key, &self.value)]).finish()
    }
}
