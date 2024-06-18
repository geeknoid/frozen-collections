macro_rules! get_fn {
    ($( $generic_type_bound1:ident $(+ $generic_type_bound2:ident)*)?) => {
        #[doc = include_str!("../doc_snippets/get_from_set_method.md")]
        #[inline]
        #[must_use]
        pub fn get<Q>(&self, value: &Q) -> Option<&T>
        where
            T: Borrow<Q>,
            Q: ?Sized $(+ $generic_type_bound1 $(+ $generic_type_bound2)*)?,
        {
            Some(self.map.get_key_value(value)?.0)
        }
    };
}

macro_rules! contains_fn {
    ($( $generic_type_bound1:ident $(+ $generic_type_bound2:ident)*)?) => {
        #[doc = include_str!("../doc_snippets/contains_method.md")]
        #[inline]
        #[must_use]
        pub fn contains<Q>(&self, value: &Q) -> bool
        where
            T: Borrow<Q>,
            Q: ?Sized $(+ $generic_type_bound1 $(+ $generic_type_bound2)*)?,
        {
            self.get(value).is_some()
        }
    };
}

macro_rules! partial_eq_fn {
    () => {
        fn eq(&self, other: &ST) -> bool {
            if self.len() != other.len() {
                return false;
            }

            self.iter().all(|value| other.contains(value))
        }
    };
}

macro_rules! into_iter_fn {
    () => {
        type Item = T;
        type IntoIter = IntoIter<T>;

        fn into_iter(self) -> Self::IntoIter {
            IntoIter::new(self.map.into_iter())
        }
    };
}

macro_rules! into_iter_ref_fn {
    () => {
        type Item = &'a T;
        type IntoIter = Iter<'a, T>;

        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    };
}

macro_rules! bitor_fn {
    ($build_hasher:ident) => {
        type Output = hashbrown::HashSet<T>;

        fn bitor(self, rhs: &ST) -> Self::Output {
            Self::Output::from_iter(self.union(rhs).cloned())
        }
    };
}

macro_rules! bitand_fn {
    ($build_hasher:ident) => {
        type Output = hashbrown::HashSet<T>;

        fn bitand(self, rhs: &ST) -> Self::Output {
            Self::Output::from_iter(self.intersection(rhs).cloned())
        }
    };
}

macro_rules! bitxor_fn {
    ($build_hasher:ident) => {
        type Output = hashbrown::HashSet<T>;

        fn bitxor(self, rhs: &ST) -> Self::Output {
            self.symmetric_difference(rhs).cloned().collect()
        }
    };
}

macro_rules! sub_fn {
    ($build_hasher:ident) => {
        type Output = hashbrown::HashSet<T>;

        fn sub(self, rhs: &ST) -> Self::Output {
            self.difference(rhs).cloned().collect()
        }
    };
}

macro_rules! set_iterator_boilerplate {
    () => {
        fn iter(&self) -> Iter<'_, T> {
            Iter::new(self.map.iter())
        }
    };
}

macro_rules! set_boilerplate {
    () => {
        fn contains(&self, value: &T) -> bool {
            self.contains(value)
        }
    };
}

macro_rules! debug_fn {
    () => {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_set().entries(self.iter()).finish()
        }
    };
}

pub(crate) use bitand_fn;
pub(crate) use bitor_fn;
pub(crate) use bitxor_fn;
pub(crate) use contains_fn;
pub(crate) use debug_fn;
pub(crate) use get_fn;
pub(crate) use into_iter_fn;
pub(crate) use into_iter_ref_fn;
pub(crate) use partial_eq_fn;
pub(crate) use set_boilerplate;
pub(crate) use set_iterator_boilerplate;
pub(crate) use sub_fn;
