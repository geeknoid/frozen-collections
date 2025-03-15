/// Indicate that a code path should be treated as cold code
#[inline]
#[cold]
pub const fn cold() {}

/*
/// Prefetch the cache line where the given data lives.
pub fn prefetch_for_read<T>(s: &[T], index: usize) {
    let ptr = unsafe { s.as_ptr().add(index) as *const u64 };
    prefetch_for_read_ptr(ptr);
}

/// Prefetch the cache line where the given data lives.
pub fn prefetch_for_read_ptr<T>(ptr: *const T) {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::x86_64::_mm_prefetch::<{core::arch::x86_64::_MM_HINT_T0}>(ptr as *const i8);
    }
    #[cfg(target_arch = "x86")]
    unsafe {
        core::arch::x86::_mm_prefetch::<{core::arch::x86::_MM_HINT_T0}>(ptr as *const i8);
    }
    #[cfg(target_arch = "aarch64")]
    unsafe {
        core::arch::aarch64::_prefetch::<{core::arch::aarch64::_PREFETCH_READ}, {core::arch::aarch64::_PREFETCH_LOCALITY3}>(ptr as *const i8);
    }
    #[cfg(not(any(target_arch = "x86_64", target_arch = "x86", target_arch = "aarch64")))]
    {
        // Do nothing.
    }
}
*/
