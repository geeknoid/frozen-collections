/// Ensure key uniqueness (assumes "keys" is a relatively small array)
pub fn any_duplicate_keys<K, const N: usize>(keys: [&K; N]) -> bool
where
    K: ?Sized + Eq,
{
    for i in 0..keys.len() {
        for j in 0..i {
            if keys[j].eq(keys[i]) {
                return true;
            }
        }
    }

    false
}

macro_rules! get {
    ($self:ident, $key:ident) => {
        let range = $self.get_hash_info($key);
        let p = $self.table.entries.as_ptr();

        unsafe {
            for i in range.start..range.end {
                let entry = p.add(i);
                if $key.eq((*entry).0.borrow()) {
                    return Some(&(*entry).1);
                }
            }
        }

        return None
    };
}

macro_rules! get_no_collisions {
    ($self:ident, $key:ident) => {
        let range = $self.get_hash_info($key);
        let p = $self.table.entries.as_ptr();

        unsafe {
            let entry = p.add(range.start);
            if $key.eq((*entry).0.borrow()) {
                return Some(&(*entry).1);
            }
        }

        return None
    };
}

macro_rules! get_key_value {
    ($self:ident, $key:ident) => {
        let range = $self.get_hash_info($key);
        let p = $self.table.entries.as_ptr();

        unsafe {
            for i in range.start..range.end {
                let entry = p.add(i);
                if $key.eq((*entry).0.borrow()) {
                    return Some((&(*entry).0, &(*entry).1));
                }
            }
        }

        return None
    };
}

macro_rules! get_key_value_no_collisions {
    ($self:ident, $key:ident) => {
        let range = $self.get_hash_info($key);
        let p = $self.table.entries.as_ptr();

        unsafe {
            let entry = p.add(range.start);
            if $key.eq((*entry).0.borrow()) {
                return Some((&(*entry).0, &(*entry).1));
            }
        }

        return None
    };
}

macro_rules! get_mut {
    ($self:ident, $key:ident) => {
        let range = $self.get_hash_info($key);
        let p = $self.table.entries.as_mut_ptr();

        unsafe {
            for i in range.start..range.end {
                let entry = p.add(i);
                if $key.eq((*entry).0.borrow()) {
                    return Some(&mut (*entry).1);
                }
            }
        }

        return None
    };
}

macro_rules! get_mut_no_collisions {
    ($self:ident, $key:ident) => {
        let range = $self.get_hash_info($key);
        let p = $self.table.entries.as_mut_ptr();

        unsafe {
            let entry = p.add(range.start);
            if $key.eq((*entry).0.borrow()) {
                return Some(&mut (*entry).1);
            }
        }

        return None
    };
}

macro_rules! get_many_mut {
    ($self:ident, $keys:ident) => {
        if any_duplicate_keys($keys) {
            return None;
        }

        let mut result: std::mem::MaybeUninit<[&mut V; N]> = std::mem::MaybeUninit::uninit();
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

macro_rules! partial_eq {
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

pub(crate) use get;
pub(crate) use get_key_value;
pub(crate) use get_key_value_no_collisions;
pub(crate) use get_many_mut;
pub(crate) use get_mut;
pub(crate) use get_mut_no_collisions;
pub(crate) use get_no_collisions;
pub(crate) use partial_eq;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_duplicates() {
        let keys = [&"a", &"b", &"c"];
        assert!(!any_duplicate_keys(keys));
    }

    #[test]
    fn with_duplicates() {
        let keys = [&"a", &"b", &"a"];
        assert!(any_duplicate_keys(keys));
    }

    #[test]
    fn empty_array() {
        let keys: [&str; 0] = [];
        assert!(!any_duplicate_keys(keys));
    }

    #[test]
    fn single_element() {
        let keys = [&"a"];
        assert!(!any_duplicate_keys(keys));
    }

    #[test]
    fn all_same_elements() {
        let keys = [&"a", &"a", &"a"];
        assert!(any_duplicate_keys(keys));
    }
}
