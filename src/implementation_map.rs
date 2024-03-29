pub(crate) trait ImplementationMap<K, V> {
    fn get(&self, key: &K) -> Option<&V>;
    fn get_key_value(&self, key: &K) -> Option<(&K, &V)>;
    fn contains_key(&self, key: &K) -> bool;
    fn len(&self) -> usize;
}
