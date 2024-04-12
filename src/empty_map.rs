use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;

pub(crate) struct EmptyMap<K, V> {
    key: PhantomData<K>,
    value: PhantomData<V>,
}

impl<K, V> EmptyMap<K, V> {
    pub fn get(&self, _key: &K) -> Option<&V> {
        None
    }

    pub fn get_key_value(&self, _key: &K) -> Option<(&K, &V)> {
        None
    }

    pub fn contains_key(&self, _key: &K) -> bool {
        false
    }

    pub fn len(&self) -> usize {
        0
    }
}

impl<K, V> Default for EmptyMap<K, V> {
    fn default() -> Self {
        Self {
            key: PhantomData,
            value: PhantomData,
        }
    }
}

impl<K, V> Debug for EmptyMap<K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("{}")
    }
}
