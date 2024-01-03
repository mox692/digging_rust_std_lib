use core_orc::cell::Cell;
use std::ptr::{self, NonNull};

// TODO: introduce weak ref to handle circular referencing.

struct RcInner<T> {
    value: T,
    strong_ref_count: Cell<usize>,
}

impl<T> RcInner<T> {
    fn strong_ref_count(&self) -> usize {
        self.strong_ref_count.get()
    }
    fn decr_ref_connt(&self) {
        self.strong_ref_count.set(self.strong_ref_count() - 1);
    }
    fn incr_ref_connt(&self) {
        self.strong_ref_count.set(self.strong_ref_count() + 1)
    }
}

pub struct Rc<T> {
    /// RcInner is handled only internally, not by the user, so it never moves,
    /// i.e., there is no need to consider Pinning.
    inner: NonNull<RcInner<T>>,
}

impl<T> Rc<T> {
    pub fn new(v: T) -> Self {
        let inner = Box::new(RcInner {
            value: v,
            strong_ref_count: Cell::new(1),
        });

        unsafe {
            // We know `inner` is not null.
            Rc {
                inner: NonNull::new_unchecked(Box::into_raw(inner)),
            }
        }
    }

    pub unsafe fn get_mut_unchecked(&mut self) -> &mut T {
        &mut self.inner.as_mut().value
    }

    pub unsafe fn decrement_strong_count(ptr: *const T) {
        todo!()
    }

    pub fn strong_count(this: &Rc<T>) -> usize {
        unsafe { this.inner.as_ref().strong_ref_count() }
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        unsafe {
            self.inner.as_ref().incr_ref_connt();
        }
        Rc { inner: self.inner }
    }
}

impl<T> AsRef<T> for Rc<T> {
    fn as_ref(&self) -> &T {
        unsafe { &self.inner.as_ref().value }
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        // decrease ref count
        unsafe { self.inner.as_ref().decr_ref_connt() }
        if Rc::strong_count(self) == 0 {
            // strong_ref_count will be zero by this drop, so we drop `RcInner` as well.
            unsafe { ptr::drop_in_place(self.inner.as_mut()) }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn inner_strong_count_is_updated_collectly() {
        let rc = Rc::new(42);
        assert!(rc.as_ref() == &42);
        assert_eq!(Rc::strong_count(&rc), 1);

        {
            let rc2 = rc.clone();
            assert_eq!(Rc::strong_count(&rc2), 2);
            assert_eq!(Rc::strong_count(&rc), 2);
        }

        assert_eq!(Rc::strong_count(&rc), 1)
    }

    // TODO: ensure that memory leak doesn't happen.
}
