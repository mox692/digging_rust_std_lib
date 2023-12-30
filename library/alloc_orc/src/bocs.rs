use std::alloc::{alloc, dealloc, Layout};
use std::ops::{Deref, DerefMut};
use std::ptr;

pub struct Bocs<T> {
    // TODO: consider aliasing.
    inner: *const T,
}

impl<T> Bocs<T> {
    pub fn new(x: T) -> Self {
        let layout = Layout::new::<T>();
        let ptr = unsafe { alloc(layout) as *const T };

        unsafe { ptr::write(ptr as *mut T, x) };

        Bocs { inner: ptr }
    }
}

impl<T: Clone> Clone for Bocs<T> {
    fn clone(&self) -> Self {
        // SAFETY:
        // `self.inner` must not be null.
        let inner_clone = unsafe { (*(self.inner)).clone() };

        Bocs::new(inner_clone)
    }
}

impl<T> Deref for Bocs<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.inner }
    }
}

impl<T> DerefMut for Bocs<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self.inner as *mut T) }
    }
}

impl<T> Drop for Bocs<T> {
    // TODO: is it ok to drop inner even when Bocs has the data which is refered from elsewhere?
    fn drop(&mut self) {
        unsafe { dealloc(self.inner as *mut u8, Layout::new::<T>()) }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_deref_trait() {
        let b = Bocs::new(1);

        assert_eq!((&b).deref(), &1);
        assert_eq!(*(&b).deref(), 1);
        assert_eq!(*b, 1);
    }

    #[test]
    fn test_deref_mut_trait() {
        let mut b = Bocs::new(1);

        assert_eq!((&mut b).deref_mut(), &mut 1);
        assert_eq!(*(&mut b).deref_mut(), 1);

        *b.deref_mut() = 42;

        assert_eq!(*b, 42);
    }
}
