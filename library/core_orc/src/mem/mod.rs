use core::ptr;

/// Overwrites a variable reference to T passed to dest with src
/// and returns the old T type value.
pub fn replace<T>(dest: &mut T, src: T) -> T {
    unsafe {
        let old = ptr::read(dest as *const T);
        ptr::write(dest as *mut T, src);
        old
    }
}
