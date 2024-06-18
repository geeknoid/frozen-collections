use alloc::boxed::Box;
use alloc::collections::VecDeque;
use alloc::rc::Rc;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;

/// Describes the length of a collection.
pub trait Len {
    /// Returns the length of a collection.
    fn len(&self) -> usize;

    /// Returns whether a collection is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(feature = "std")]
impl<T, CM> Len for std::collections::HashSet<T, CM> {
    fn len(&self) -> usize {
        self.len()
    }
}

#[cfg(feature = "std")]
impl<K, V, CM> Len for std::collections::HashMap<K, V, CM> {
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

impl Len for &str {
    fn len(&self) -> usize {
        str::len(self)
    }
}

impl Len for core::ffi::CStr {
    fn len(&self) -> usize {
        self.to_bytes().len()
    }
}

#[cfg(feature = "std")]
impl Len for std::ffi::CString {
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

#[cfg(feature = "std")]
impl<K, V> Len for std::collections::BTreeMap<K, V> {
    fn len(&self) -> usize {
        self.len()
    }
}

#[cfg(feature = "std")]
impl<T> Len for std::collections::BTreeSet<T> {
    fn len(&self) -> usize {
        self.len()
    }
}

#[cfg(feature = "std")]
impl<T> Len for std::collections::BinaryHeap<T> {
    fn len(&self) -> usize {
        self.len()
    }
}

#[cfg(feature = "std")]
impl<T> Len for std::collections::LinkedList<T> {
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

#[cfg(feature = "std")]
impl Len for std::ffi::OsStr {
    fn len(&self) -> usize {
        self.len()
    }
}

#[cfg(feature = "std")]
impl Len for std::ffi::OsString {
    fn len(&self) -> usize {
        self.as_os_str().len()
    }
}

impl<K, V, BH> Len for hashbrown::HashMap<K, V, BH> {
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T, BH> Len for hashbrown::HashSet<T, BH> {
    fn len(&self) -> usize {
        self.len()
    }
}

#[cfg(test)]
mod tests {
    use hashbrown::{HashMap, HashSet};

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
