use core_orc::cell::Cell;
use std::ptr::NonNull;

struct RcInner<T> {
    value: T,
    ref_count: Cell<usize>,
}

pub struct Rc<T> {
    /// RcInner is handled only internally, not by the user, so it never moves,
    /// i.e., there is no need to consider Pinning.
    inner: NonNull<RcInner<T>>,
}

impl<T> Rc<T> {
    fn new(v: T) -> Self {
        let inner = Box::new(RcInner {
            value: v,
            ref_count: Cell::new(0),
        });

        unsafe {
            // We know `inner` is not null.
            Rc {
                inner: NonNull::new_unchecked(Box::into_raw(inner)),
            }
        }
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        unsafe {
            let cur_count = { self.inner.as_ref().ref_count.get() };
            self.inner.as_ref().ref_count.set(cur_count + 1);
        }
        Rc { inner: self.inner }
    }
}
