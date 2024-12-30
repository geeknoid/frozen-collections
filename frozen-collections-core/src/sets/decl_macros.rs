macro_rules! get_fn {
    ("Scalar") => {
        #[inline]
        fn get(&self, value: &T) -> Option<&T> {
            Some(self.map.get_key_value(value)?.0)
        }
    };

    () => {
        #[inline]
        fn get(&self, value: &Q) -> Option<&T> {
            Some(self.map.get_key_value(value)?.0)
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
    () => {
        type Output = hashbrown::HashSet<T>;

        fn bitor(self, rhs: &ST) -> Self::Output {
            Self::Output::from_iter(self.union(rhs).cloned())
        }
    };
}

macro_rules! bitand_fn {
    () => {
        type Output = hashbrown::HashSet<T>;

        fn bitand(self, rhs: &ST) -> Self::Output {
            Self::Output::from_iter(self.intersection(rhs).cloned())
        }
    };
}

macro_rules! bitxor_fn {
    () => {
        type Output = hashbrown::HashSet<T>;

        fn bitxor(self, rhs: &ST) -> Self::Output {
            self.symmetric_difference(rhs).cloned().collect()
        }
    };
}

macro_rules! sub_fn {
    () => {
        type Output = hashbrown::HashSet<T>;

        fn sub(self, rhs: &ST) -> Self::Output {
            self.difference(rhs).cloned().collect()
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

#[cfg(feature = "serde")]
macro_rules! serialize_fn {
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

macro_rules! set_iteration_funcs {
    () => {
        fn iter(&self) -> Iter<'_, T> {
            Iter::new(self.map.iter())
        }
    };
}

pub(crate) use bitand_fn;
pub(crate) use bitor_fn;
pub(crate) use bitxor_fn;
pub(crate) use debug_fn;
pub(crate) use get_fn;
pub(crate) use into_iter_fn;
pub(crate) use into_iter_ref_fn;
pub(crate) use partial_eq_fn;
pub(crate) use set_iteration_funcs;
pub(crate) use sub_fn;

#[cfg(feature = "serde")]
pub(crate) use serialize_fn;
