use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

pub(crate) struct StringMap<V> {
    entries: HashMap<&str, V>,
}

impl<V> FromIterator<(&str, V)> for StringMap<V> {
    fn from_iter<T: IntoIterator<Item = (&str, V)>>(iter: T) -> Self {
        let mut entries = HashMap::new();
        entries.extend(iter);

        Self { entries }
    }
}

impl<V> StringMap<V> {
    pub fn get(&self, key: &&str) -> Option<&V> {
        self.entries.get(key)
    }

    pub fn get_key_value(&self, key: &&str) -> Option<(&&str, &V)> {
        self.entries.get_key_value(key)
    }

    pub fn contains_key(&self, key: &&str) -> bool {
        self.entries.contains_key(key)
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

impl<V> Debug for StringMap<V>
where
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let pairs = self.entries.iter().map(|x| (x.0, x.1));
        f.debug_map().entries(pairs).finish()
    }
}
