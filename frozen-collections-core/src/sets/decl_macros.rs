macro_rules! bitand_trait_funcs {
    () => {
        type Output = hashbrown::HashSet<T>;

        fn bitand(self, rhs: &ST) -> Self::Output {
            Self::Output::from_iter(self.intersection(rhs).cloned())
        }
    };
}

macro_rules! bitor_trait_funcs {
    () => {
        type Output = hashbrown::HashSet<T>;

        fn bitor(self, rhs: &ST) -> Self::Output {
            Self::Output::from_iter(self.union(rhs).cloned())
        }
    };
}

macro_rules! bitxor_trait_funcs {
    () => {
        type Output = hashbrown::HashSet<T>;

        fn bitxor(self, rhs: &ST) -> Self::Output {
            self.symmetric_difference(rhs).cloned().collect()
        }
    };
}

macro_rules! common_primary_funcs {
    ($const_len:ident) => {
        #[doc = include_str!("../doc_snippets/iter.md")]
        #[must_use]
        pub fn iter(&self) -> Iter<'_, T> {
            Iter::new(self.map.iter())
        }

        #[must_use]
        fn into_iter(self) -> IntoIter<T> {
            IntoIter::new(self.map.into_iter())
        }

        common_primary_funcs!(@len $const_len);
    };

    (@len const_len) => {
        #[doc = include_str!("../doc_snippets/len.md")]
        #[inline]
        #[must_use]
        pub const fn len(&self) -> usize {
            self.map.len()
        }

        #[doc = include_str!("../doc_snippets/is_empty.md")]
        #[inline]
        #[must_use]
        pub const fn is_empty(&self) -> bool {
            self.len() == 0
        }
    };
}

macro_rules! debug_trait_funcs {
    () => {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_set().entries(self.iter()).finish()
        }
    };
}

macro_rules! hash_primary_funcs {
    () => {
        #[doc = include_str!("../doc_snippets/get_from_set.md")]
        #[inline]
        #[must_use]
        pub fn get<Q>(&self, value: &Q) -> Option<&T>
        where
            Q: ?Sized + Equivalent<T>,
            H: Hasher<Q>,
        {
            Some(self.map.get_key_value(value)?.0)
        }

        #[doc = include_str!("../doc_snippets/contains.md")]
        #[inline]
        #[must_use]
        pub fn contains<Q>(&self, value: &Q) -> bool
        where
            Q: ?Sized + Equivalent<T>,
            H: Hasher<Q>,
        {
            self.get(value).is_some()
        }
    };
}

macro_rules! into_iterator_trait_funcs {
    () => {
        type Item = T;
        type IntoIter = IntoIter<T>;

        fn into_iter(self) -> Self::IntoIter {
            self.into_iter()
        }
    };
}

macro_rules! into_iterator_ref_trait_funcs {
    () => {
        type Item = &'a T;
        type IntoIter = Iter<'a, T>;

        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    };
}

macro_rules! partial_eq_trait_funcs {
    () => {
        fn eq(&self, other: &ST) -> bool {
            if self.len() != other.len() {
                return false;
            }

            self.iter().all(|value| other.contains(value))
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
            let mut seq = serializer.serialize_seq(Some(self.len()))?;
            for v in self {
                seq.serialize_element(v)?;
            }
            seq.end()
        }
    };
}

macro_rules! set_extras_trait_funcs {
    () => {
        #[inline]
        fn get(&self, value: &Q) -> Option<&T> {
            self.get(value)
        }
    };
}

macro_rules! set_iteration_trait_funcs {
    () => {
        fn iter(&self) -> Self::Iterator<'_> {
            self.iter()
        }
    };
}

macro_rules! set_query_trait_funcs {
    () => {
        #[inline]
        fn contains(&self, value: &Q) -> bool {
            self.contains(value)
        }
    };
}

macro_rules! sub_trait_funcs {
    () => {
        type Output = hashbrown::HashSet<T>;

        fn sub(self, rhs: &ST) -> Self::Output {
            self.difference(rhs).cloned().collect()
        }
    };
}

pub(crate) use bitand_trait_funcs;
pub(crate) use bitor_trait_funcs;
pub(crate) use bitxor_trait_funcs;
pub(crate) use common_primary_funcs;
pub(crate) use debug_trait_funcs;
pub(crate) use hash_primary_funcs;
pub(crate) use into_iterator_ref_trait_funcs;
pub(crate) use into_iterator_trait_funcs;
pub(crate) use partial_eq_trait_funcs;
pub(crate) use set_extras_trait_funcs;
pub(crate) use set_iteration_trait_funcs;
pub(crate) use set_query_trait_funcs;
pub(crate) use sub_trait_funcs;

#[cfg(feature = "serde")]
pub(crate) use serialize_trait_funcs;
