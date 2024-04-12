use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

use crate::implementation_map::ImplementationMap;

pub(crate) struct StringMap<'a, V> {
    entries: HashMap<&'a str, V>,
}

impl<'a, V> FromIterator<(&'a str, V)> for StringMap<'a, V> {
    fn from_iter<T: IntoIterator<Item = (&'a str, V)>>(iter: T) -> Self {
        let mut entries = HashMap::new();
        entries.extend(iter);

        Self { entries }
    }
}

impl<'a, V> ImplementationMap<&str, V> for StringMap<'a, V> {
    fn get(&self, key: &&str) -> Option<&V> {
        self.entries.get(key)
    }

    fn get_key_value(&self, key: &&str) -> Option<(&&str, &V)> {
        self.entries.get_key_value(key)
    }

    fn contains_key(&self, key: &&str) -> bool {
        self.entries.contains_key(key)
    }

    fn len(&self) -> usize {
        self.entries.len()
    }
}

impl<'a, V> Debug for StringMap<'a, V>
where
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let pairs = self.entries.iter().map(|x| (x.0, x.1));
        f.debug_map().entries(pairs).finish()
    }
}
