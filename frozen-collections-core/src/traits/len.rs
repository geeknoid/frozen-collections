use alloc::collections::VecDeque;
use alloc::rc::Rc;
use alloc::sync::Arc;

#[cfg(not(feature = "std"))]
use {alloc::boxed::Box, alloc::string::String, alloc::vec::Vec};

/// Types that can return a length.
pub trait Len {
    #[doc = include_str!("../doc_snippets/len.md")]
    fn len(&self) -> usize;

    #[doc = include_str!("../doc_snippets/is_empty.md")]
    fn is_empty(&self) -> bool {
        self.len() == 0
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

#[cfg(feature = "std")]
impl Len for alloc::ffi::CString {
    fn len(&self) -> usize {
        self.as_bytes().len()
    }
}

#[cfg(feature = "std")]
impl<K, V> Len for alloc::collections::BTreeMap<K, V> {
    fn len(&self) -> usize {
        self.len()
    }
}

#[cfg(feature = "std")]
impl<T> Len for alloc::collections::BTreeSet<T> {
    fn len(&self) -> usize {
        self.len()
    }
}

#[cfg(feature = "std")]
impl<T> Len for alloc::collections::BinaryHeap<T> {
    fn len(&self) -> usize {
        self.len()
    }
}

#[cfg(feature = "std")]
impl<T> Len for alloc::collections::LinkedList<T> {
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

#[cfg(test)]
mod tests {
    use crate::traits::Len;
    use alloc::collections::VecDeque;
    use alloc::rc::Rc;
    use alloc::sync::Arc;
    use alloc::vec;

    fn get_len<T: Len + ?Sized>(value: &T) -> usize {
        value.len()
    }

    #[test]
    #[cfg(feature = "std")]
    fn hashset_len_and_is_empty() {
        let mut set = std::collections::HashSet::new();
        assert_eq!(get_len(&set), 0);
        assert!(set.is_empty());

        _ = set.insert(1);
        assert_eq!(get_len(&set), 1);
        assert!(!set.is_empty());
    }

    #[test]
    #[cfg(feature = "std")]
    fn hashmap_len_and_is_empty() {
        let mut map = std::collections::HashMap::new();
        assert_eq!(get_len(&map), 0);
        assert!(map.is_empty());

        _ = map.insert("key", "value");
        assert_eq!(get_len(&map), 1);
        assert!(!map.is_empty());
    }

    #[test]
    fn hashbrown_hashset_len_and_is_empty() {
        let mut set = hashbrown::HashSet::new();
        assert_eq!(get_len(&set), 0);
        assert!(set.is_empty());

        _ = set.insert(1);
        assert_eq!(get_len(&set), 1);
        assert!(!set.is_empty());
    }

    #[test]
    fn hashbrown_hashmap_len_and_is_empty() {
        let mut map = hashbrown::HashMap::new();
        assert_eq!(get_len(&map), 0);
        assert!(map.is_empty());

        _ = map.insert("key", "value");
        assert_eq!(get_len(&map), 1);
        assert!(!map.is_empty());
    }

    #[test]
    fn string_len_and_is_empty() {
        let s = String::new();
        assert_eq!(get_len(&s), 0);
        assert!(s.is_empty());

        let s = String::from("hello");
        assert_eq!(get_len(&s), 5);
        assert!(!s.is_empty());
    }

    #[test]
    fn str_len_and_is_empty() {
        let s = "";
        assert_eq!(get_len(&s), 0);
        assert!(s.is_empty());

        let s = "hello";
        assert_eq!(get_len(&s), 5);
        assert!(!s.is_empty());
    }

    #[test]
    #[cfg(feature = "std")]
    fn cstring_len_and_is_empty() {
        use alloc::ffi::CString;
        let s = CString::new("").unwrap();
        assert_eq!(get_len(&s), 0);
        assert!(s.is_empty());

        let s = CString::new("hello").unwrap();
        assert_eq!(get_len(&s), 5);
        assert!(!s.is_empty());
    }

    #[test]
    #[cfg(feature = "std")]
    fn cstr_len_and_is_empty() {
        let s = c"";
        assert_eq!(get_len(s), 0);
        assert!(s.is_empty());

        let s = c"hello";
        assert_eq!(get_len(s), 5);
        assert!(!s.is_empty());
    }

    #[test]
    fn vec_len_and_is_empty() {
        let v: Vec<i32> = Vec::new();
        assert_eq!(get_len(&v), 0);
        assert!(v.is_empty());

        let v = vec![1, 2, 3];
        assert_eq!(get_len(&v), 3);
        assert!(!v.is_empty());
    }

    #[test]
    fn vecdeque_len_and_is_empty() {
        let v: VecDeque<i32> = VecDeque::new();
        assert_eq!(get_len(&v), 0);
        assert!(v.is_empty());

        let v: VecDeque<i32> = alloc::vec![1, 2, 3].into();
        assert_eq!(get_len(&v), 3);
        assert!(!v.is_empty());
    }

    #[test]
    fn box_len_and_is_empty() {
        let b: Box<str> = Box::from("");
        assert_eq!(get_len(&b), 0);
        assert!(b.is_empty());

        let b: Box<str> = Box::from("hello");
        assert_eq!(get_len(&b), 5);
        assert!(!b.is_empty());
    }

    #[test]
    fn rc_len_and_is_empty() {
        let r: Rc<str> = Rc::from("");
        assert_eq!(get_len(&r), 0);
        assert!(r.is_empty());

        let r: Rc<str> = Rc::from("hello");
        assert_eq!(get_len(&r), 5);
        assert!(!r.is_empty());
    }

    #[test]
    fn arc_len_and_is_empty() {
        let a: Arc<str> = Arc::from("");
        assert_eq!(get_len(&a), 0);
        assert!(a.is_empty());

        let a: Arc<str> = Arc::from("hello");
        assert_eq!(get_len(&a), 5);
        assert!(!a.is_empty());
    }

    #[test]
    #[cfg(feature = "std")]
    fn btreemap_len_and_is_empty() {
        use alloc::collections::BTreeMap;
        let mut map = BTreeMap::new();
        assert_eq!(get_len(&map), 0);
        assert!(map.is_empty());

        _ = map.insert("key", "value");
        assert_eq!(get_len(&map), 1);
        assert!(!map.is_empty());
    }

    #[test]
    #[cfg(feature = "std")]
    fn btreeset_len_and_is_empty() {
        use alloc::collections::BTreeSet;
        let mut set = BTreeSet::new();
        assert_eq!(get_len(&set), 0);
        assert!(set.is_empty());

        _ = set.insert(1);
        assert_eq!(get_len(&set), 1);
        assert!(!set.is_empty());
    }

    #[test]
    #[cfg(feature = "std")]
    fn binaryheap_len_and_is_empty() {
        use alloc::collections::BinaryHeap;
        let mut heap = BinaryHeap::new();
        assert_eq!(get_len(&heap), 0);
        assert!(heap.is_empty());

        heap.push(1);
        assert_eq!(get_len(&heap), 1);
        assert!(!heap.is_empty());
    }

    #[test]
    #[cfg(feature = "std")]
    fn linkedlist_len_and_is_empty() {
        use alloc::collections::LinkedList;
        let mut list = LinkedList::new();
        assert_eq!(get_len(&list), 0);
        assert!(list.is_empty());

        list.push_back(1);
        assert_eq!(get_len(&list), 1);
        assert!(!list.is_empty());
    }

    #[test]
    #[cfg(feature = "std")]
    fn osstr_len_and_is_empty() {
        use std::ffi::OsStr;
        let s = OsStr::new("");
        assert_eq!(get_len(s), 0);
        assert!(s.is_empty());

        let s = OsStr::new("hello");
        assert_eq!(get_len(s), 5);
        assert!(!s.is_empty());
    }

    #[test]
    #[cfg(feature = "std")]
    fn osstring_len_and_is_empty() {
        use std::ffi::OsString;
        let s = OsString::new();
        assert_eq!(get_len(&s), 0);
        assert!(s.is_empty());

        let s = OsString::from("hello");
        assert_eq!(get_len(&s), 5);
        assert!(!s.is_empty());
    }

    #[test]
    fn slice_len_and_is_empty() {
        let s: &[u8] = [].as_slice();
        assert_eq!(get_len(s), 0);
        assert!(s.is_empty());

        let s = [0, 1, 2, 3, 4].as_slice();
        assert_eq!(get_len(s), 5);
        assert!(!s.is_empty());
    }
}
