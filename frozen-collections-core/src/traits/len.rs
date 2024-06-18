use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use std::ffi::{CStr, CString, OsStr, OsString};
use std::rc::Rc;
use std::sync::Arc;

/// Describes the length of a collection.
pub trait Len {
    /// Returns the length of a collection.
    fn len(&self) -> usize;

    /// Returns whether a collection is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T, S> Len for HashSet<T, S> {
    fn len(&self) -> usize {
        self.len()
    }
}

impl<K, V, S> Len for HashMap<K, V, S> {
    fn len(&self) -> usize {
        self.len()
    }
}

impl Len for String {
    fn len(&self) -> usize {
        self.len()
    }
}

impl Len for str {
    fn len(&self) -> usize {
        self.len()
    }
}

impl Len for CStr {
    fn len(&self) -> usize {
        self.to_bytes().len()
    }
}

impl Len for CString {
    fn len(&self) -> usize {
        self.as_bytes().len()
    }
}

impl<T> Len for [T] {
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T: ?Sized + Len> Len for Box<T> {
    fn len(&self) -> usize {
        T::len(self)
    }
}

impl<T: ?Sized + Len> Len for Rc<T> {
    fn len(&self) -> usize {
        T::len(self)
    }
}

impl<T: ?Sized + Len> Len for Arc<T> {
    fn len(&self) -> usize {
        T::len(self)
    }
}

impl<K, V> Len for BTreeMap<K, V> {
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T> Len for BTreeSet<T> {
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T> Len for BinaryHeap<T> {
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T> Len for LinkedList<T> {
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T> Len for Vec<T> {
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T> Len for VecDeque<T> {
    fn len(&self) -> usize {
        self.len()
    }
}

impl Len for OsStr {
    fn len(&self) -> usize {
        self.len()
    }
}

impl Len for OsString {
    fn len(&self) -> usize {
        self.as_os_str().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hashset_len_and_is_empty() {
        let mut set = HashSet::new();
        assert_eq!(set.len(), 0);
        assert!(set.is_empty());

        set.insert(1);
        assert_eq!(set.len(), 1);
        assert!(!set.is_empty());
    }

    #[test]
    fn hashmap_len_and_is_empty() {
        let mut map = HashMap::new();
        assert_eq!(map.len(), 0);
        assert!(map.is_empty());

        map.insert("key", "value");
        assert_eq!(map.len(), 1);
        assert!(!map.is_empty());
    }
}
