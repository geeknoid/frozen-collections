macro_rules! get_many_mut_fn {
    ("Scalar") => {
        fn get_many_mut<const N: usize>(&mut self, keys: [&K; N]) -> Option<[&mut V; N]> {
            if crate::utils::has_duplicates_with_hasher(
                &keys,
                &crate::hashers::PassthroughHasher::default(),
            ) {
                crate::utils::cold();
                return None;
            }

            get_many_mut_body!(self, keys);
        }
    };

    ("Hash") => {
        #[must_use]
        fn get_many_mut<const N: usize>(&mut self, keys: [&Q; N]) -> Option<[&mut V; N]> {
            if crate::utils::has_duplicates_with_hasher(&keys, &self.hasher) {
                crate::utils::cold();
                return None;
            }

            get_many_mut_body!(self, keys);
        }
    };

    () => {
        fn get_many_mut<const N: usize>(&mut self, keys: [&Q; N]) -> Option<[&mut V; N]> {
            if crate::utils::has_duplicates_slow(&keys) {
                crate::utils::cold();
                return None;
            }

            get_many_mut_body!(self, keys);
        }
    };
}

macro_rules! get_many_mut_body {
    ($self:ident, $keys:ident) => {
        let mut result: core::mem::MaybeUninit<[&mut V; N]> = core::mem::MaybeUninit::uninit();
        let p = result.as_mut_ptr();
        let x: *mut Self = $self;

        unsafe {
            for (i, key) in $keys.iter().enumerate() {
                (*p)[i] = (*x).get_mut(*key)?;
            }

            return Some(result.assume_init());
        }
    };
}

macro_rules! index_fn {
    () => {
        type Output = V;

        #[inline]
        fn index(&self, index: &Q) -> &Self::Output {
            self.get(index).expect("index should be valid")
        }
    };
}

macro_rules! into_iter_fn {
    ($($entries:ident)+) => {
        type Item = (K, V);
        type IntoIter = IntoIter<K, V>;

        fn into_iter(self) -> Self::IntoIter {
            IntoIter::new(self$(. $entries)+.into())
        }
    };
}

macro_rules! into_iter_ref_fn {
    () => {
        type Item = (&'a K, &'a V);
        type IntoIter = Iter<'a, K, V>;

        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    };
}

macro_rules! into_iter_mut_ref_fn {
    () => {
        type Item = (&'a K, &'a mut V);
        type IntoIter = IterMut<'a, K, V>;

        fn into_iter(self) -> Self::IntoIter {
            self.iter_mut()
        }
    };
}

macro_rules! partial_eq_fn {
    () => {
        fn eq(&self, other: &MT) -> bool {
            if self.len() != other.len() {
                return false;
            }

            return self
                .iter()
                .all(|(key, value)| other.get(key).map_or(false, |v| *value == *v));
        }
    };
}

macro_rules! debug_fn {
    () => {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            let pairs = self.entries.iter().map(|x| (&x.0, &x.1));
            f.debug_map().entries(pairs).finish()
        }
    };
}

#[cfg(feature = "serde")]
macro_rules! serialize_fn {
    () => {
        fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut map = serializer.serialize_map(Some(self.len()))?;
            for (k, v) in self {
                map.serialize_entry(k, v)?;
            }
            map.end()
        }
    };
}

macro_rules! map_iteration_funcs {
    ($($entries:ident)+) => {
        type IntoKeyIterator = IntoKeys<K, V>;
        type IntoValueIterator = IntoValues<K, V>;

        fn iter(&self) -> Self::Iterator<'_> {
            Iter::new(&self$(. $entries)+)
        }

        fn keys(&self) -> Self::KeyIterator<'_> {
            Keys::new(&self$(. $entries)+)
        }

        fn values(&self) -> Self::ValueIterator<'_> {
            Values::new(&self$(. $entries)+)
        }

        fn into_keys(self) -> Self::IntoKeyIterator {
            // NOTE: this allocates and copies everything into a vector for the sake of iterating the vector.
            // This is the best I could come up with, let me know if you see a way around the need to copy.
            IntoKeys::new(Vec::from(self$(. $entries)+).into_boxed_slice())
        }

        fn into_values(self) -> Self::IntoValueIterator {
            // NOTE: this allocates and copies everything into a vector for the sake of iterating the vector.
            // This is the best I could come up with, let me know if you see a way around the need to copy.
            IntoValues::new(Vec::from(self$(. $entries)+).into_boxed_slice())
        }

        fn iter_mut(&mut self) -> Self::MutIterator<'_> {
            IterMut::new(self$(. $entries)+.as_mut())
        }

        fn values_mut(&mut self) -> Self::ValueMutIterator<'_> {
            ValuesMut::new(self$(. $entries)+.as_mut())
        }
    };
}

macro_rules! dense_scalar_lookup_query_funcs {
    () => {
        #[inline]
        fn get(&self, key: &K) -> Option<&V> {
            let index = key.index();
            if index >= self.min && index <= self.max {
                let entry = unsafe { self.entries.get_unchecked(index - self.min) };
                Some(&entry.1)
            } else {
                None
            }
        }

        #[inline]
        fn get_key_value(&self, key: &K) -> Option<(&K, &V)> {
            let index = key.index();
            if index >= self.min && index <= self.max {
                let entry = unsafe { self.entries.get_unchecked(index - self.min) };
                Some((&entry.0, &entry.1))
            } else {
                None
            }
        }

        #[inline]
        fn get_mut(&mut self, key: &K) -> Option<&mut V> {
            let index = key.index();
            if index >= self.min && index <= self.max {
                let entry = unsafe { self.entries.get_unchecked_mut(index - self.min) };
                Some(&mut entry.1)
            } else {
                None
            }
        }
    };
}

macro_rules! binary_search_query_funcs {
    () => {
        #[inline]
        fn get(&self, key: &Q) -> Option<&V> {
            self.entries
                .binary_search_by(|entry| key.compare(&entry.0).reverse())
                .map(|index| {
                    let entry = unsafe { self.entries.get_unchecked(index) };
                    &entry.1
                })
                .ok()
        }

        #[inline]
        fn get_mut(&mut self, key: &Q) -> Option<&mut V> {
            self.entries
                .binary_search_by(|entry| key.compare(&entry.0).reverse())
                .map(|index| {
                    let entry = unsafe { self.entries.get_unchecked_mut(index) };
                    &mut entry.1
                })
                .ok()
        }

        #[inline]
        fn get_key_value(&self, key: &Q) -> Option<(&K, &V)> {
            self.entries
                .binary_search_by(|entry| key.compare(&entry.0).reverse())
                .map(|index| {
                    let entry = unsafe { self.entries.get_unchecked(index) };
                    (&entry.0, &entry.1)
                })
                .ok()
        }
    };
}

macro_rules! eytzinger_search_query_funcs {
    () => {
        #[inline]
        fn get(&self, key: &Q) -> Option<&V> {
            if let Some(index) =
                eytzinger_search_by(&self.entries, |entry| key.compare(&entry.0).reverse())
            {
                let entry = unsafe { self.entries.get_unchecked(index) };
                Some(&entry.1)
            } else {
                None
            }
        }

        #[inline]
        fn get_mut(&mut self, key: &Q) -> Option<&mut V> {
            if let Some(index) =
                eytzinger_search_by(&self.entries, |entry| key.compare(&entry.0).reverse())
            {
                let entry = unsafe { self.entries.get_unchecked_mut(index) };
                Some(&mut entry.1)
            } else {
                None
            }
        }

        #[inline]
        fn get_key_value(&self, key: &Q) -> Option<(&K, &V)> {
            if let Some(index) =
                eytzinger_search_by(&self.entries, |entry| key.compare(&entry.0).reverse())
            {
                let entry = unsafe { self.entries.get_unchecked(index) };
                Some((&entry.0, &entry.1))
            } else {
                None
            }
        }
    };
}

macro_rules! sparse_scalar_lookup_query_funcs {
    () => {
        #[inline]
        fn get(&self, key: &K) -> Option<&V> {
            let index = key.index();
            if index >= self.min && index <= self.max {
                let index_in_lookup = index - self.min;
                let index_in_entries: usize =
                    unsafe { (*self.lookup.get_unchecked(index_in_lookup)).into() };
                if index_in_entries > 0 {
                    let entry = unsafe { self.entries.get_unchecked(index_in_entries - 1) };
                    return Some(&entry.1);
                }
            }

            None
        }

        #[inline]
        fn get_key_value(&self, key: &K) -> Option<(&K, &V)> {
            let index = key.index();
            if index >= self.min && index <= self.max {
                let index_in_lookup = index - self.min;
                let index_in_entries: usize =
                    unsafe { (*self.lookup.get_unchecked(index_in_lookup)).into() };
                if index_in_entries > 0 {
                    let entry = unsafe { self.entries.get_unchecked(index_in_entries - 1) };
                    return Some((&entry.0, &entry.1));
                }
            }

            None
        }

        #[inline]
        fn get_mut(&mut self, key: &K) -> Option<&mut V> {
            let index = key.index();
            if index >= self.min && index <= self.max {
                let index_in_lookup = index - self.min;
                let index_in_entries: usize =
                    unsafe { (*self.lookup.get_unchecked(index_in_lookup)).into() };
                if index_in_entries > 0 {
                    let entry = unsafe { self.entries.get_unchecked_mut(index_in_entries - 1) };
                    return Some(&mut entry.1);
                }
            }

            None
        }
    };
}

macro_rules! scan_query_funcs {
    () => {
        #[inline]
        fn get(&self, key: &Q) -> Option<&V> {
            for entry in &self.entries {
                if key.equivalent(&entry.0) {
                    return Some(&entry.1);
                }
            }

            None
        }

        #[inline]
        fn get_mut(&mut self, key: &Q) -> Option<&mut V> {
            for entry in &mut self.entries {
                if key.equivalent(&entry.0) {
                    return Some(&mut entry.1);
                }
            }

            None
        }

        #[inline]
        fn get_key_value(&self, key: &Q) -> Option<(&K, &V)> {
            for entry in &self.entries {
                if key.equivalent(&entry.0) {
                    return Some((&entry.0, &entry.1));
                }
            }

            None
        }
    };
}

macro_rules! ordered_scan_query_funcs {
    () => {
        #[inline]
        fn get(&self, key: &Q) -> Option<&V> {
            for entry in &self.entries {
                let ord = key.compare(&entry.0);
                if ord == Ordering::Equal {
                    return Some(&entry.1);
                } else if ord == Ordering::Less {
                    break;
                }
            }

            None
        }

        #[inline]
        fn get_mut(&mut self, key: &Q) -> Option<&mut V> {
            for entry in &mut self.entries {
                let ord = key.compare(&entry.0);
                if ord == Ordering::Equal {
                    return Some(&mut entry.1);
                } else if ord == Ordering::Less {
                    break;
                }
            }

            None
        }

        #[inline]
        fn get_key_value(&self, key: &Q) -> Option<(&K, &V)> {
            for entry in &self.entries {
                let ord = key.compare(&entry.0);
                if ord == Ordering::Equal {
                    return Some((&entry.0, &entry.1));
                } else if ord == Ordering::Less {
                    break;
                }
            }

            None
        }
    };
}

macro_rules! hash_query_funcs {
    () => {
        #[inline]
        fn get(&self, key: &Q) -> Option<&V> {
            self.table
                .find(self.hasher.hash(key), |entry| key.equivalent(&entry.0))
                .map(|(_, v)| v)
        }

        #[inline]
        fn get_key_value(&self, key: &Q) -> Option<(&K, &V)> {
            self.table
                .find(self.hasher.hash(key), |entry| key.equivalent(&entry.0))
                .map(|(k, v)| (k, v))
        }

        #[inline]
        fn get_mut(&mut self, key: &Q) -> Option<&mut V> {
            self.table
                .find_mut(self.hasher.hash(key), |entry| key.equivalent(&entry.0))
                .map(|(_, v)| v)
        }
    };
}

pub(crate) use binary_search_query_funcs;
pub(crate) use debug_fn;
pub(crate) use dense_scalar_lookup_query_funcs;
pub(crate) use eytzinger_search_query_funcs;
pub(crate) use get_many_mut_body;
pub(crate) use get_many_mut_fn;
pub(crate) use hash_query_funcs;
pub(crate) use index_fn;
pub(crate) use into_iter_fn;
pub(crate) use into_iter_mut_ref_fn;
pub(crate) use into_iter_ref_fn;
pub(crate) use map_iteration_funcs;
pub(crate) use ordered_scan_query_funcs;
pub(crate) use partial_eq_fn;
pub(crate) use scan_query_funcs;
pub(crate) use sparse_scalar_lookup_query_funcs;

#[cfg(feature = "serde")]
pub(crate) use serialize_fn;
