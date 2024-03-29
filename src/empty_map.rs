use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;

use crate::implementation_map::ImplementationMap;

pub(crate) struct EmptyMap<K, V> {
    key: PhantomData<K>,
    value: PhantomData<V>,
}

impl<K, V> ImplementationMap<K, V> for EmptyMap<K, V> {
    fn get(&self, _key: &K) -> Option<&V> {
        None
    }

    fn get_key_value(&self, _key: &K) -> Option<(&K, &V)> {
        None
    }

    fn contains_key(&self, _key: &K) -> bool {
        false
    }

    fn len(&self) -> usize {
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
