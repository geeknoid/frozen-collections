macro_rules! binary_search_primary_funcs {
    () => {
        #[doc = include_str!("../doc_snippets/get.md")]
        #[inline]
        pub fn get<Q>(&self, key: &Q) -> Option<&V>
        where
            Q: ?Sized + Comparable<K>,
        {
            self.entries
                .binary_search_by(|entry| key.compare(&entry.0).reverse())
                .map(|index| {
                    // SAFETY: We are guaranteed that the index is valid because binary_search_by returns an in-range index
                    let entry = unsafe { self.entries.get_unchecked(index) };
                    &entry.1
                })
                .ok()
        }

        #[doc = include_str!("../doc_snippets/get_mut.md")]
        #[inline]
        pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
        where
            Q: ?Sized + Comparable<K>,
        {
            self.entries
                .binary_search_by(|entry| key.compare(&entry.0).reverse())
                .map(|index| {
                    // SAFETY: We are guaranteed that the index is valid because binary_search_by returns an in-range index
                    let entry = unsafe { self.entries.get_unchecked_mut(index) };
                    &mut entry.1
                })
                .ok()
        }

        #[doc = include_str!("../doc_snippets/get_key_value.md")]
        #[inline]
        pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
        where
            Q: ?Sized + Comparable<K>,
        {
            self.entries
                .binary_search_by(|entry| key.compare(&entry.0).reverse())
                .map(|index| {
                    // SAFETY: We are guaranteed that the index is valid because binary_search_by returns an in-range index
                    let entry = unsafe { self.entries.get_unchecked(index) };
                    (&entry.0, &entry.1)
                })
                .ok()
        }

        #[doc = include_str!("../doc_snippets/contains_key.md")]
        #[inline]
        #[must_use]
        pub fn contains_key<Q>(&self, key: &Q) -> bool
        where
            Q: ?Sized + Comparable<K>,
        {
            self.get(key).is_some()
        }

        get_disjoint_mut_funcs!("Ord");
    };
}

macro_rules! common_primary_funcs {
    ($const_len:ident, $($entries:ident)+) => {
        #[doc = include_str!("../doc_snippets/iter.md")]
        #[must_use]
        pub fn iter(&self) -> Iter<'_, K, V> {
            Iter::new(&self$(. $entries)+)
        }

        #[doc = include_str!("../doc_snippets/iter_mut.md")]
        #[must_use]
        pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
            IterMut::new(self$(. $entries)+.as_mut())
        }

        #[must_use]
        fn into_iter(self) -> IntoIter<K, V> {
            IntoIter::new(self$(. $entries)+.into())
        }

        #[doc = include_str!("../doc_snippets/keys.md")]
        #[must_use]
        pub fn keys(&self) -> Keys<'_, K, V> {
            Keys::new(&self$(. $entries)+)
        }

        #[doc = include_str!("../doc_snippets/into_keys.md")]
        #[must_use]
        pub fn into_keys(self) -> IntoKeys<K, V> {
            // NOTE: this allocates and copies everything into a vector for the sake of iterating the vector.
            // This is the best I could come up with, let me know if you see a way around the need to copy.
            IntoKeys::new(alloc::vec::Vec::from(self$(. $entries)+).into_boxed_slice())
        }

        #[doc = include_str!("../doc_snippets/values.md")]
        #[must_use]
        pub fn values(&self) -> Values<'_, K, V> {
            Values::new(&self$(. $entries)+)
        }

        #[doc = include_str!("../doc_snippets/values_mut.md")]
        #[must_use]
        pub fn values_mut(&mut self) -> ValuesMut<'_, K, V> {
            ValuesMut::new(self$(. $entries)+.as_mut())
        }

        #[doc = include_str!("../doc_snippets/into_values.md")]
        #[must_use]
        pub fn into_values(self) -> IntoValues<K, V> {
            // NOTE: this allocates and copies everything into a vector for the sake of iterating the vector.
            // This is the best I could come up with, let me know if you see a way around the need to copy.
            IntoValues::new(alloc::vec::Vec::from(self$(. $entries)+).into_boxed_slice())
        }

        common_primary_funcs!(@len $const_len);
    };

    (@len const_len) => {
        #[doc = include_str!("../doc_snippets/len.md")]
        #[inline]
        #[must_use]
        pub const fn len(&self) -> usize {
            self.entries.len()
        }

        #[doc = include_str!("../doc_snippets/is_empty.md")]
        #[inline]
        #[must_use]
        pub const fn is_empty(&self) -> bool {
            self.len() == 0
        }
    };

    (@len non_const_len) => {
        #[doc = include_str!("../doc_snippets/len.md")]
        #[inline]
        #[must_use]
        pub fn len(&self) -> usize {
            self.entries.len()
        }

        #[doc = include_str!("../doc_snippets/is_empty.md")]
        #[inline]
        #[must_use]
        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }
    };
}

macro_rules! debug_trait_funcs {
    () => {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            let pairs = self.iter().map(|x| (x.0, x.1));
            f.debug_map().entries(pairs).finish()
        }
    };
}

macro_rules! dense_scalar_lookup_primary_funcs {
    () => {
        #[doc = include_str!("../doc_snippets/get.md")]
        #[inline]
        pub fn get<Q>(&self, key: &Q) -> Option<&V>
        where
            Q: Comparable<K> + Scalar,
        {
            let index = key.index();
            (index >= self.min && index <= self.max).then(|| {
                // SAFETY: We are guaranteed that the index is valid because we checked it against min and max
                let entry = unsafe { self.entries.get_unchecked(index - self.min) };
                &entry.1
            })
        }

        #[doc = include_str!("../doc_snippets/get_mut.md")]
        #[inline]
        pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
        where
            Q: Comparable<K> + Scalar,
        {
            let index = key.index();
            (index >= self.min && index <= self.max).then(|| {
                // SAFETY: We are guaranteed that the index is valid because we checked it against min and max
                let entry = unsafe { self.entries.get_unchecked_mut(index - self.min) };
                &mut entry.1
            })
        }

        #[doc = include_str!("../doc_snippets/get_key_value.md")]
        #[inline]
        pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
        where
            Q: Comparable<K> + Scalar,
        {
            let index = key.index();
            (index >= self.min && index <= self.max).then(|| {
                // SAFETY: We are guaranteed that the index is valid because we checked it against min and max
                let entry = unsafe { self.entries.get_unchecked(index - self.min) };
                (&entry.0, &entry.1)
            })
        }

        #[doc = include_str!("../doc_snippets/contains_key.md")]
        #[inline]
        #[must_use]
        pub fn contains_key<Q>(&self, key: &Q) -> bool
        where
            Q: Comparable<K> + Scalar,
        {
            self.get(key).is_some()
        }

        get_disjoint_mut_funcs!("Scalar");
    };
}

macro_rules! eytzinger_search_primary_funcs {
    () => {
        #[doc = include_str!("../doc_snippets/get.md")]
        #[inline]
        pub fn get<Q>(&self, key: &Q) -> Option<&V>
        where
            Q: ?Sized + Comparable<K>,
        {
            if let Some(index) = eytzinger_search_by(&self.entries, |entry| key.compare(&entry.0).reverse()) {
                // SAFETY: We are guaranteed that the index is valid because eytzinger_search_by returns an in-range index
                let entry = unsafe { self.entries.get_unchecked(index) };
                Some(&entry.1)
            } else {
                None
            }
        }

        #[doc = include_str!("../doc_snippets/get_mut.md")]
        #[inline]
        pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
        where
            Q: ?Sized + Comparable<K>,
        {
            if let Some(index) = eytzinger_search_by(&self.entries, |entry| key.compare(&entry.0).reverse()) {
                // SAFETY: We are guaranteed that the index is valid because eytzinger_search_by returns an in-range index
                let entry = unsafe { self.entries.get_unchecked_mut(index) };
                Some(&mut entry.1)
            } else {
                None
            }
        }

        #[doc = include_str!("../doc_snippets/get_key_value.md")]
        #[inline]
        pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
        where
            Q: ?Sized + Comparable<K>,
        {
            if let Some(index) = eytzinger_search_by(&self.entries, |entry| key.compare(&entry.0).reverse()) {
                // SAFETY: We are guaranteed that the index is valid because eytzinger_search_by returns an in-range index
                let entry = unsafe { self.entries.get_unchecked(index) };
                Some((&entry.0, &entry.1))
            } else {
                None
            }
        }

        #[doc = include_str!("../doc_snippets/contains_key.md")]
        #[inline]
        #[must_use]
        pub fn contains_key<Q>(&self, key: &Q) -> bool
        where
            Q: ?Sized + Comparable<K>,
        {
            self.get(key).is_some()
        }

        get_disjoint_mut_funcs!("Ord");
    };
}

macro_rules! get_disjoint_mut_funcs {
    ("Ord") => {
        #[doc = include_str!("../doc_snippets/get_disjoint_mut.md")]
        pub fn get_disjoint_mut<Q, const N: usize>(&mut self, keys: [&Q; N]) -> [Option<&mut V>; N]
        where
            Q: ?Sized + Eq + Comparable<K>,
        {
            get_disjoint_mut_funcs!(@safe_body, self, keys);
        }

        #[doc = include_str!("../doc_snippets/get_disjoint_unchecked_mut.md")]
        pub unsafe fn get_disjoint_unchecked_mut<Q, const N: usize>(&mut self, keys: [&Q; N]) -> [Option<&mut V>; N]
        where
            Q: ?Sized + Comparable<K>,
        {
            get_disjoint_mut_funcs!(@unsafe_body, self, keys);
        }
    };

    ("Scalar") => {
        #[doc = include_str!("../doc_snippets/get_disjoint_mut.md")]
        pub fn get_disjoint_mut<Q, const N: usize>(&mut self, keys: [&Q; N]) -> [Option<&mut V>; N]
        where
            Q: Eq + Comparable<K> + Scalar,
        {
            get_disjoint_mut_funcs!(@safe_body, self, keys);
        }

        #[doc = include_str!("../doc_snippets/get_disjoint_unchecked_mut.md")]
        pub unsafe fn get_disjoint_unchecked_mut<Q, const N: usize>(&mut self, keys: [&Q; N]) -> [Option<&mut V>; N]
        where
            Q: Comparable<K> + Scalar,
        {
            get_disjoint_mut_funcs!(@unsafe_body, self, keys);
        }
    };

    (@safe_body, $self:ident, $keys:ident) => {
        if crate::utils::has_duplicates(&$keys) {
            crate::utils::cold();
            panic!("duplicate keys found");
        }

        // SAFETY: We've validated that the caller isn't asking for duplicate keys
        return unsafe { $self.get_disjoint_unchecked_mut($keys) };
    };

    (@unsafe_body, $self:ident, $keys:ident) => {
        let ptrs: [Option<::core::ptr::NonNull<V>>; N] = ::core::array::from_fn(|i| {
            $self.get_mut($keys[i]).map(|value| {
                let v = value as *mut V;
                ::core::ptr::NonNull::new(v).unwrap()
            })
        });

        return ptrs.map(|ptr| ptr.map(|mut ptr|
            // SAFETY: pointers all safely acquired above
            unsafe { ptr.as_mut() }
        ));
    };
}

macro_rules! hash_primary_funcs {
    () => {
        #[doc = include_str!("../doc_snippets/get.md")]
        #[inline]
        pub fn get<Q>(&self, key: &Q) -> Option<&V>
        where
            Q: ?Sized + Equivalent<K>,
            H: Hasher<Q>,
        {
            self.entries
                .find(self.hasher.hash_one(key), |entry| key.equivalent(&entry.0))
                .map(|(_, v)| v)
        }

        #[doc = include_str!("../doc_snippets/get_mut.md")]
        #[inline]
        pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
        where
            Q: ?Sized + Equivalent<K>,
            H: Hasher<Q>,
        {
            self.entries
                .find_mut(self.hasher.hash_one(key), |entry| key.equivalent(&entry.0))
                .map(|(_, v)| v)
        }

        #[doc = include_str!("../doc_snippets/get_key_value.md")]
        #[inline]
        pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
        where
            Q: ?Sized + Equivalent<K>,
            H: Hasher<Q>,
        {
            self.entries
                .find(self.hasher.hash_one(key), |entry| key.equivalent(&entry.0))
                .map(|(k, v)| (k, v))
        }

        #[doc = include_str!("../doc_snippets/contains_key.md")]
        #[inline]
        #[must_use]
        pub fn contains_key<Q>(&self, key: &Q) -> bool
        where
            Q: ?Sized + Equivalent<K>,
            H: Hasher<Q>,
        {
            self.get(key).is_some()
        }

        #[doc = include_str!("../doc_snippets/get_disjoint_mut.md")]
        pub fn get_disjoint_mut<Q, const N: usize>(&mut self, keys: [&Q; N]) -> [Option<&mut V>; N]
        where
            Q: ?Sized + Eq + Equivalent<K>,
            H: Hasher<Q>,
        {
            get_disjoint_mut_funcs!(@safe_body, self, keys);
        }

        #[doc = include_str!("../doc_snippets/get_disjoint_unchecked_mut.md")]
        pub unsafe fn get_disjoint_unchecked_mut<Q, const N: usize>(&mut self, keys: [&Q; N]) -> [Option<&mut V>; N]
        where
            Q: ?Sized + Equivalent<K>,
            H: Hasher<Q>,
        {
            get_disjoint_mut_funcs!(@unsafe_body, self, keys);
        }
    };
}

macro_rules! index_trait_funcs {
    () => {
        type Output = V;

        #[inline]
        fn index(&self, index: &Q) -> &Self::Output {
            self.get(index).expect("index should be valid")
        }
    };
}

macro_rules! into_iterator_trait_funcs {
    () => {
        type Item = (K, V);
        type IntoIter = IntoIter<K, V>;

        fn into_iter(self) -> Self::IntoIter {
            self.into_iter()
        }
    };
}

macro_rules! into_iterator_trait_mut_ref_funcs {
    () => {
        type Item = (&'a K, &'a mut V);
        type IntoIter = IterMut<'a, K, V>;

        fn into_iter(self) -> Self::IntoIter {
            self.iter_mut()
        }
    };
}

macro_rules! into_iterator_trait_ref_funcs {
    () => {
        type Item = (&'a K, &'a V);
        type IntoIter = Iter<'a, K, V>;

        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    };
}

macro_rules! len_trait_funcs {
    () => {
        #[inline]
        fn len(&self) -> usize {
            self.len()
        }

        #[inline]
        fn is_empty(&self) -> bool {
            self.is_empty()
        }
    };
}

macro_rules! map_extras_trait_funcs {
    () => {
        fn get_key_value(&self, key: &Q) -> Option<(&K, &V)> {
            self.get_key_value(key)
        }

        fn get_disjoint_mut<const N: usize>(&mut self, keys: [&Q; N]) -> [Option<&mut V>; N]
        where
            Q: Eq,
        {
            self.get_disjoint_mut(keys)
        }

        unsafe fn get_disjoint_unchecked_mut<const N: usize>(&mut self, keys: [&Q; N]) -> [Option<&mut V>; N] {
            // SAFETY: The caller is responsible for ensuring that the keys are valid and unique
            unsafe { self.get_disjoint_unchecked_mut(keys) }
        }
    };
}

macro_rules! map_iteration_trait_funcs {
    () => {
        type IntoKeyIterator = IntoKeys<K, V>;
        type IntoValueIterator = IntoValues<K, V>;

        fn iter(&self) -> Self::Iterator<'_> {
            self.iter()
        }

        fn iter_mut(&mut self) -> Self::MutIterator<'_> {
            self.iter_mut()
        }

        fn keys(&self) -> Self::KeyIterator<'_> {
            self.keys()
        }

        fn into_keys(self) -> Self::IntoKeyIterator {
            self.into_keys()
        }

        fn values(&self) -> Self::ValueIterator<'_> {
            self.values()
        }

        fn values_mut(&mut self) -> Self::ValueMutIterator<'_> {
            self.values_mut()
        }

        fn into_values(self) -> Self::IntoValueIterator {
            self.into_values()
        }
    };
}

macro_rules! map_query_trait_funcs {
    () => {
        #[inline]
        fn get(&self, key: &Q) -> Option<&V> {
            self.get(key)
        }

        #[inline]
        fn get_mut(&mut self, key: &Q) -> Option<&mut V> {
            self.get_mut(key)
        }

        #[inline]
        fn contains_key(&self, key: &Q) -> bool {
            self.contains_key(key)
        }
    };
}

macro_rules! partial_eq_trait_funcs {
    () => {
        fn eq(&self, other: &MT) -> bool {
            if self.len() != other.len() {
                return false;
            }

            self.iter().all(|(key, value)| other.get(key).map_or(false, |v| *value == *v))
        }
    };
}

macro_rules! scan_primary_funcs {
    () => {
        #[doc = include_str!("../doc_snippets/get.md")]
        #[inline]
        pub fn get<Q>(&self, key: &Q) -> Option<&V>
        where
            Q: ?Sized + Equivalent<K>,
        {
            let mut result = None;
            for entry in &self.entries {
                if key.equivalent(&entry.0) {
                    result = Some(&entry.1);
                }
            }

            result
        }

        #[doc = include_str!("../doc_snippets/get_mut.md")]
        #[inline]
        pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
        where
            Q: ?Sized + Equivalent<K>,
        {
            let mut result = None;
            for entry in &mut self.entries {
                if key.equivalent(&entry.0) {
                    result = Some(&mut entry.1);
                }
            }

            result
        }

        #[doc = include_str!("../doc_snippets/get_key_value.md")]
        #[inline]
        pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
        where
            Q: ?Sized + Equivalent<K>,
        {
            let mut result = None;
            for entry in &self.entries {
                if key.equivalent(&entry.0) {
                    result = Some((&entry.0, &entry.1));
                }
            }

            result
        }

        #[doc = include_str!("../doc_snippets/contains_key.md")]
        #[inline]
        #[must_use]
        pub fn contains_key<Q>(&self, key: &Q) -> bool
        where
            Q: ?Sized + Equivalent<K>,
        {
            self.get(key).is_some()
        }

        #[doc = include_str!("../doc_snippets/get_disjoint_mut.md")]
        pub fn get_disjoint_mut<Q, const N: usize>(&mut self, keys: [&Q; N]) -> [Option<&mut V>; N]
        where
            Q: ?Sized + Eq + Equivalent<K>,
        {
            get_disjoint_mut_funcs!(@safe_body, self, keys);
        }

        #[doc = include_str!("../doc_snippets/get_disjoint_unchecked_mut.md")]
        pub unsafe fn get_disjoint_unchecked_mut<Q, const N: usize>(&mut self, keys: [&Q; N]) -> [Option<&mut V>; N]
        where
            Q: ?Sized + Equivalent<K>,
        {
            get_disjoint_mut_funcs!(@unsafe_body, self, keys);
        }
    };
}

#[cfg(feature = "serde")]
macro_rules! serialize_trait_funcs {
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

macro_rules! sparse_scalar_lookup_primary_funcs {
    () => {
        #[doc = include_str!("../doc_snippets/get.md")]
        #[inline]
        pub fn get<Q>(&self, key: &Q) -> Option<&V>
        where
            Q: Comparable<K> + Scalar,
        {
            let index = key.index();
            if index >= self.min && index <= self.max {
                let index_in_lookup = index - self.min;

                // SAFETY: We are guaranteed that the index is valid because we checked it against min and max
                let index_in_entries: usize = unsafe { (*self.lookup.get_unchecked(index_in_lookup)).into() };
                (index_in_entries > 0).then(|| {
                    // SAFETY: We are guaranteed that the index is valid because we checked it against min and max
                    let entry = unsafe { self.entries.get_unchecked(index_in_entries - 1) };
                    &entry.1
                })
            } else {
                None
            }
        }

        #[doc = include_str!("../doc_snippets/get_mut.md")]
        #[inline]
        fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
        where
            Q: Comparable<K> + Scalar,
        {
            let index = key.index();
            if index >= self.min && index <= self.max {
                let index_in_lookup = index - self.min;

                // SAFETY: We are guaranteed that the index is valid because we checked it against min and max
                let index_in_entries: usize = unsafe { (*self.lookup.get_unchecked(index_in_lookup)).into() };
                (index_in_entries > 0).then(|| {
                    // SAFETY: We are guaranteed that the index is valid because we checked it against min and max
                    let entry = unsafe { self.entries.get_unchecked_mut(index_in_entries - 1) };
                    &mut entry.1
                })
            } else {
                None
            }
        }

        #[doc = include_str!("../doc_snippets/get_key_value.md")]
        #[inline]
        pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
        where
            Q: Comparable<K> + Scalar,
        {
            let index = key.index();
            if index >= self.min && index <= self.max {
                let index_in_lookup = index - self.min;

                // SAFETY: We are guaranteed that the index is valid because we checked it against min and max
                let index_in_entries: usize = unsafe { (*self.lookup.get_unchecked(index_in_lookup)).into() };
                (index_in_entries > 0).then(|| {
                    // SAFETY: We are guaranteed that the index is valid because we checked it against min and max
                    let entry = unsafe { self.entries.get_unchecked(index_in_entries - 1) };
                    (&entry.0, &entry.1)
                })
            } else {
                None
            }
        }

        #[doc = include_str!("../doc_snippets/contains_key.md")]
        #[inline]
        #[must_use]
        pub fn contains_key<Q>(&self, key: &Q) -> bool
        where
            Q: Comparable<K> + Scalar,
        {
            self.get(key).is_some()
        }

        get_disjoint_mut_funcs!("Scalar");
    };
}

pub(crate) use binary_search_primary_funcs;
pub(crate) use common_primary_funcs;
pub(crate) use debug_trait_funcs;
pub(crate) use dense_scalar_lookup_primary_funcs;
pub(crate) use eytzinger_search_primary_funcs;
pub(crate) use get_disjoint_mut_funcs;
pub(crate) use hash_primary_funcs;
pub(crate) use index_trait_funcs;
pub(crate) use into_iterator_trait_funcs;
pub(crate) use into_iterator_trait_mut_ref_funcs;
pub(crate) use into_iterator_trait_ref_funcs;
pub(crate) use len_trait_funcs;
pub(crate) use map_extras_trait_funcs;
pub(crate) use map_iteration_trait_funcs;
pub(crate) use map_query_trait_funcs;
pub(crate) use partial_eq_trait_funcs;
pub(crate) use scan_primary_funcs;
pub(crate) use sparse_scalar_lookup_primary_funcs;

#[cfg(feature = "serde")]
pub(crate) use serialize_trait_funcs;
