macro_rules! get_many_mut_fn {
    ($( $generic_type_bound1:ident $(+ $generic_type_bound2:ident)*)?) => {
        #[doc = include_str!("../doc_snippets/get_many_mut_method.md")]
        #[must_use]
        pub fn get_many_mut<Q, const N: usize>(&mut self, keys: [&Q; N]) -> Option<[&mut V; N]>
        where
            K: Borrow<Q>,
            Q: ?Sized $(+ $generic_type_bound1 $(+ $generic_type_bound2)*)?,
        {
            get_many_mut_body!(self, keys);
        }
    };
}

macro_rules! get_many_mut_body {
    ($self:ident, $keys:ident) => {
        if crate::utils::slow_find_duplicate(&$keys).is_some() {
            return None;
        }

        let mut result: core::mem::MaybeUninit<[&mut V; N]> = core::mem::MaybeUninit::uninit();
        let p = result.as_mut_ptr();
        let x: *mut Self = $self;

        unsafe {
            for (i, key) in $keys.iter().enumerate() {
                (*p)[i] = (*x).get_mut(key)?;
            }

            return Some(result.assume_init());
        }
    };
}

macro_rules! contains_key_fn {
    ($( $generic_type_bound1:ident $(+ $generic_type_bound2:ident)*)?) => {
        #[doc = include_str!("../doc_snippets/contains_key_method.md")]
        #[inline]
        #[must_use]
        pub fn contains_key<Q>(&self, key: &Q) -> bool
        where
            K: Borrow<Q>,
            Q: ?Sized $(+ $generic_type_bound1 $(+ $generic_type_bound2)*)?,
        {
            self.get(key).is_some()
        }
    }
}

macro_rules! index_fn {
    () => {
        type Output = V;

        fn index(&self, index: &Q) -> &Self::Output {
            self.get(index).expect("index should be valid")
        }
    };
}

macro_rules! into_iter_fn_for_slice {
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

macro_rules! map_iterator_boilerplate_for_slice {
    ($($entries:ident)+) => {
        type IntoKeyIterator = IntoKeys<K, V>;
        type IntoValueIterator = IntoValues<K, V>;

        #[inline]
        fn iter(&self) -> Self::Iterator<'_> {
            Iter::new(&self$(. $entries)+)
        }

        #[inline]
        fn keys(&self) -> Self::KeyIterator<'_> {
            Keys::new(&self$(. $entries)+)
        }

        #[inline]
        fn values(&self) -> Self::ValueIterator<'_> {
            Values::new(&self$(. $entries)+)
        }

        #[inline]
        fn into_keys(self) -> Self::IntoKeyIterator {
            // NOTE: this allocates and copies everything into a vector for the sake of iterating the vector.
            // This is the best I could come up with, let me know if you see a way around the need to copy.
            IntoKeys::new(Vec::from(self$(. $entries)+).into_boxed_slice())
        }

        #[inline]
        fn into_values(self) -> Self::IntoValueIterator {
            // NOTE: this allocates and copies everything into a vector for the sake of iterating the vector.
            // This is the best I could come up with, let me know if you see a way around the need to copy.
            IntoValues::new(Vec::from(self$(. $entries)+).into_boxed_slice())
        }

        #[inline]
        fn iter_mut(&mut self) -> Self::MutIterator<'_> {
            IterMut::new(self$(. $entries)+.as_mut())
        }

        #[inline]
        fn values_mut(&mut self) -> Self::ValueMutIterator<'_> {
            ValuesMut::new(self$(. $entries)+.as_mut())
        }
    };
}

macro_rules! map_boilerplate_for_slice {
    ($($entries:ident)+) => {
        #[inline]
        fn contains_key(&self, key: &K) -> bool {
            self.contains_key(key)
        }

        #[inline]
        fn get(&self, key: &K) -> Option<&V> {
            Self::get(self, key)
        }

        #[inline]
        fn get_key_value(&self, key: &K) -> Option<(&K, &V)> {
            Self::get_key_value(self, key)
        }

        #[inline]
        fn get_mut(&mut self, key: &K) -> Option<&mut V> {
            Self::get_mut(self, key)
        }

        #[inline]
        fn get_many_mut<const N: usize>(&mut self, keys: [&K; N]) -> Option<[&mut V; N]> {
            Self::get_many_mut(self, keys)
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

macro_rules! dense_scalar_lookup_core {
    () => {
        #[doc = include_str!("../doc_snippets/get_method.md")]
        #[inline]
        #[must_use]
        pub fn get<Q>(&self, key: &Q) -> Option<&V>
        where
            K: Borrow<Q>,
            Q: Scalar,
        {
            let index = key.index();
            if index >= self.min && index <= self.max {
                let entry = unsafe { self.entries.get_unchecked(index - self.min) };
                Some(&entry.1)
            } else {
                None
            }
        }

        #[doc = include_str!("../doc_snippets/get_key_value_method.md")]
        #[inline]
        #[must_use]
        pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
        where
            K: Borrow<Q>,
            Q: Scalar,
        {
            let index = key.index();
            if index >= self.min && index <= self.max {
                let entry = unsafe { self.entries.get_unchecked(index - self.min) };
                Some((&entry.0, &entry.1))
            } else {
                None
            }
        }

        #[doc = include_str!("../doc_snippets/get_mut_method.md")]
        #[inline]
        #[must_use]
        pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
        where
            K: Borrow<Q>,
            Q: Scalar,
        {
            let index = key.index();
            if index >= self.min && index <= self.max {
                let entry = unsafe { self.entries.get_unchecked_mut(index - self.min) };
                Some(&mut entry.1)
            } else {
                None
            }
        }

        get_many_mut_fn!(Scalar);
        contains_key_fn!(Scalar);
    };
}

macro_rules! binary_search_core {
    () => {
        #[doc = include_str!("../doc_snippets/get_method.md")]
        #[inline]
        #[must_use]
        pub fn get<Q>(&self, key: &Q) -> Option<&V>
        where
            K: Borrow<Q>,
            Q: ?Sized + Ord,
        {
            self.entries
                .binary_search_by_key(&key, |entry| entry.0.borrow())
                .map(|index| &self.entries[index].1)
                .ok()
        }

        #[doc = include_str!("../doc_snippets/get_mut_method.md")]
        #[inline]
        #[must_use]
        pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
        where
            K: Borrow<Q>,
            Q: ?Sized + Ord,
        {
            self.entries
                .binary_search_by_key(&key, |entry| entry.0.borrow())
                .map(|index| &mut self.entries[index].1)
                .ok()
        }

        #[doc = include_str!("../doc_snippets/get_key_value_method.md")]
        #[inline]
        #[must_use]
        pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
        where
            K: Borrow<Q>,
            Q: ?Sized + Ord,
        {
            self.entries
                .binary_search_by_key(&key, |entry| entry.0.borrow())
                .map(|index| (&self.entries[index].0, &self.entries[index].1))
                .ok()
        }

        get_many_mut_fn!(Ord);
        contains_key_fn!(Ord);
    };
}

macro_rules! sparse_scalar_lookup_core {
    () => {
        #[doc = include_str!("../doc_snippets/get_method.md")]
        #[inline]
        #[must_use]
        pub fn get<Q>(&self, key: &Q) -> Option<&V>
        where
            K: Borrow<Q>,
            Q: Scalar,
        {
            let index = key.index();
            if index >= self.min && index <= self.max {
                let index_in_lookup = index - self.min;
                let index_in_entries =
                    unsafe { (*self.lookup.get_unchecked(index_in_lookup)).into() };
                if index_in_entries > 0 {
                    let entry = unsafe { self.entries.get_unchecked(index_in_entries - 1) };
                    return Some(&entry.1);
                }
            }

            None
        }

        #[doc = include_str!("../doc_snippets/get_key_value_method.md")]
        #[inline]
        #[must_use]
        pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
        where
            K: Borrow<Q>,
            Q: Scalar,
        {
            let index = key.index();
            if index >= self.min && index <= self.max {
                let index_in_lookup = index - self.min;
                let index_in_entries =
                    unsafe { (*self.lookup.get_unchecked(index_in_lookup)).into() };
                if index_in_entries > 0 {
                    let entry = unsafe { self.entries.get_unchecked(index_in_entries - 1) };
                    return Some((&entry.0, &entry.1));
                }
            }

            None
        }

        #[doc = include_str!("../doc_snippets/get_mut_method.md")]
        #[inline]
        #[must_use]
        pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
        where
            K: Borrow<Q>,
            Q: Scalar,
        {
            let index = key.index();
            if index >= self.min && index <= self.max {
                let index_in_lookup = index - self.min;
                let index_in_entries =
                    unsafe { (*self.lookup.get_unchecked(index_in_lookup)).into() };
                if index_in_entries > 0 {
                    let entry = unsafe { self.entries.get_unchecked_mut(index_in_entries - 1) };
                    return Some(&mut entry.1);
                }
            }

            None
        }

        get_many_mut_fn!(Scalar);
        contains_key_fn!(Scalar);
    };
}

macro_rules! scan_core {
    () => {
        #[doc = include_str!("../doc_snippets/get_method.md")]
        #[inline]
        #[must_use]
        pub fn get<Q>(&self, key: &Q) -> Option<&V>
        where
            K: Borrow<Q>,
            Q: ?Sized + Eq,
        {
            for entry in &self.entries {
                if key.eq(entry.0.borrow()) {
                    return Some(&entry.1);
                }
            }

            None
        }

        #[doc = include_str!("../doc_snippets/get_mut_method.md")]
        #[inline]
        #[must_use]
        pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
        where
            K: Borrow<Q>,
            Q: ?Sized + Eq,
        {
            for entry in &mut self.entries {
                if key.eq(entry.0.borrow()) {
                    return Some(&mut entry.1);
                }
            }

            None
        }

        #[doc = include_str!("../doc_snippets/get_key_value_method.md")]
        #[inline]
        #[must_use]
        pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
        where
            K: Borrow<Q>,
            Q: ?Sized + Eq,
        {
            for entry in &self.entries {
                if key.eq(entry.0.borrow()) {
                    return Some((&entry.0, &entry.1));
                }
            }

            None
        }

        get_many_mut_fn!(Eq);
        contains_key_fn!(Eq);
    };
}

macro_rules! ordered_scan_core {
    () => {
        #[doc = include_str!("../doc_snippets/get_method.md")]
        #[inline]
        #[must_use]
        pub fn get<Q>(&self, key: &Q) -> Option<&V>
        where
            K: Borrow<Q>,
            Q: ?Sized + Ord,
        {
            for entry in &self.entries {
                let ord = key.cmp(entry.0.borrow());
                if ord == Ordering::Equal {
                    return Some(&entry.1);
                } else if ord == Ordering::Less {
                    break;
                }
            }

            None
        }

        #[doc = include_str!("../doc_snippets/get_mut_method.md")]
        #[inline]
        #[must_use]
        pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
        where
            K: Borrow<Q>,
            Q: ?Sized + Ord,
        {
            for entry in &mut self.entries {
                let ord = key.cmp(entry.0.borrow());
                if ord == Ordering::Equal {
                    return Some(&mut entry.1);
                } else if ord == Ordering::Less {
                    break;
                }
            }

            None
        }

        #[doc = include_str!("../doc_snippets/get_key_value_method.md")]
        #[inline]
        #[must_use]
        pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
        where
            K: Borrow<Q>,
            Q: ?Sized + Ord,
        {
            for entry in &self.entries {
                let ord = key.cmp(entry.0.borrow());
                if ord == Ordering::Equal {
                    return Some((&entry.0, &entry.1));
                } else if ord == Ordering::Less {
                    break;
                }
            }

            None
        }

        get_many_mut_fn!(Ord);
        contains_key_fn!(Ord);
    };
}

macro_rules! hash_core {
    () => {
        #[doc = include_str!("../doc_snippets/get_method.md")]
        #[inline]
        #[must_use]
        pub fn get<Q>(&self, key: &Q) -> Option<&V>
        where
            K: Borrow<Q>,
            H: Hasher<Q>,
            Q: ?Sized + Eq,
        {
            self.table
                .find(self.hasher.hash(key), |entry| key.eq(entry.0.borrow()))
                .map(|(_, v)| v)
        }

        #[doc = include_str!("../doc_snippets/get_key_value_method.md")]
        #[inline]
        #[must_use]
        pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
        where
            K: Borrow<Q>,
            H: Hasher<Q>,
            Q: ?Sized + Eq,
        {
            self.table
                .find(self.hasher.hash(key), |entry| key.eq(entry.0.borrow()))
                .map(|(k, v)| (k, v))
        }

        #[doc = include_str!("../doc_snippets/get_mut_method.md")]
        #[inline]
        #[must_use]
        pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
        where
            K: Borrow<Q>,
            H: Hasher<Q>,
            Q: ?Sized + Eq,
        {
            self.table
                .find_mut(self.hasher.hash(key), |entry| key.eq(entry.0.borrow()))
                .map(|(_, v)| v)
        }

        #[doc = include_str!("../doc_snippets/get_many_mut_method.md")]
        #[must_use]
        pub fn get_many_mut<Q, const N: usize>(&mut self, keys: [&Q; N]) -> Option<[&mut V; N]>
        where
            K: Borrow<Q>,
            H: Hasher<Q>,
            Q: ?Sized + Eq,
        {
            let mut hashes = [0_u64; N];
            for i in 0..N {
                hashes[i] = self.hasher.hash(keys[i]);
            }

            self.table
                .get_many_mut(hashes, |i, (k, _)| keys[i].eq(k.borrow()))
                .map(|res| res.map(|(_, v)| v))
        }

        #[doc = include_str!("../doc_snippets/contains_key_method.md")]
        #[inline]
        #[must_use]
        pub fn contains_key<Q>(&self, key: &Q) -> bool
        where
            K: Borrow<Q>,
            H: Hasher<Q>,
            Q: ?Sized + Eq,
        {
            self.get(key).is_some()
        }
    };
}

pub(crate) use binary_search_core;
pub(crate) use contains_key_fn;
pub(crate) use debug_fn;
pub(crate) use dense_scalar_lookup_core;
pub(crate) use get_many_mut_body;
pub(crate) use get_many_mut_fn;
pub(crate) use hash_core;
pub(crate) use index_fn;
pub(crate) use into_iter_fn_for_slice;
pub(crate) use into_iter_mut_ref_fn;
pub(crate) use into_iter_ref_fn;
pub(crate) use map_boilerplate_for_slice;
pub(crate) use map_iterator_boilerplate_for_slice;
pub(crate) use ordered_scan_core;
pub(crate) use partial_eq_fn;
pub(crate) use scan_core;
pub(crate) use sparse_scalar_lookup_core;
