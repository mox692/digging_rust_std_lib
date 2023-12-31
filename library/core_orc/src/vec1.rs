// Resources:
// https://doc.rust-lang.org/nomicon/vec/vec-alloc.html

use crate::{
    into_iterator::IntoIterator,
    iter::traits::{collector::FromIterator, iterator::Iterator},
};
use std::alloc::{self, dealloc, Layout};
use std::ptr;

#[derive(Debug)]
pub struct Vec1<T> {
    ptr: *const T,
    len: usize,
    cap: usize,
}

impl<T> Vec1<T> {
    #[inline(always)]
    fn size_of_inner() -> usize {
        std::mem::size_of::<T>()
    }

    pub fn new_with_size_and_cap(len: usize, cap: usize) -> Self {
        // For more detail, see https://doc.rust-lang.org/nomicon/vec/vec-alloc.html
        assert!(cap != 0, "cap == 0 is prohibited for now.");

        let layout = Layout::array::<T>(cap).unwrap();

        // SAFETY:
        // Here, we ensure that allocated memory is not zero-sized.
        let s = unsafe { alloc::alloc(layout) };

        Vec1 {
            ptr: s as *const T,
            len: len,
            cap: cap,
        }
    }

    pub fn new_1(x: T) -> Self {
        let layout = Layout::array::<T>(1).unwrap();

        // SAFETY:
        // Here, we ensure that allocated memory is not zero-sized.
        let m = unsafe { alloc::alloc(layout) as *mut T };

        unsafe { ptr::write(m, x) };

        Vec1 {
            ptr: m as *const T,
            len: 1,
            cap: 1,
        }
    }

    pub fn at(&self, index: usize) -> &T {
        // check whether input `n` is a valid data.
        assert!(
            index < self.len,
            "specified index `{}` is larger than the length of this Vec1, {}",
            index,
            self.len
        );

        // SAFETY:
        // this memory location is allocated by `Self::new_with_size_and_cap`,
        // and not be null, dangling, unaligned.
        unsafe { &*(self.ptr as *mut T).wrapping_add(index) }
    }

    pub fn push(&mut self, x: T) {
        if self.len + 1 <= self.cap {
            // here, we don't need to allocate additional heap memory.

            let offset = (self.ptr as *mut T).wrapping_add(self.len);
            // SAFETY:
            // dest is writable and aligned.
            unsafe { ptr::write(offset, x) };
            self.len += 1;
        } else {
            // here, we have to reallocate heap memory for this vec.

            // usually, it is common way to `double` its cap when we want to grow the vector's cap.
            let new_cap = self.cap * 2;
            let layout = Layout::array::<T>(new_cap).unwrap();
            let new_ptr = unsafe { alloc::alloc(layout) };

            let old_pointer = self.ptr as *mut u8;
            let old_cap = self.cap;

            // copy data to new vec.
            // SAFETY:
            // dest is writable and aligned.
            unsafe { ptr::copy::<T>(self.ptr, new_ptr as *mut T, self.cap) };

            // write new pushed data.
            unsafe { ptr::write((new_ptr as *mut T).wrapping_add(self.cap), x) }

            // mutate Self.
            self.ptr = new_ptr as *const T;
            self.cap = new_cap;
            self.len += 1;

            // drop old heap memory.
            unsafe { dealloc(old_pointer, Layout::array::<T>(old_cap).unwrap()) };
        }
    }
}

/// it is Vec specific type, which impl `Iterator` trait.
pub struct VecIntoIter<T> {
    ptr: *const T,
    len: usize,
    cur_pos: usize,
}

impl<T> VecIntoIter<T> {
    fn new(vec: Vec1<T>) -> Self {
        VecIntoIter {
            ptr: vec.ptr,
            len: vec.len,
            cur_pos: 0,
        }
    }
}

///
/// trait implementation
///

impl<T> IntoIterator for Vec1<T> {
    type Item = T;
    type IntoIter = VecIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        VecIntoIter::new(self)
    }
}

impl<T> Iterator for VecIntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // boundary check
        if self.cur_pos > self.len - 1 {
            None
        } else {
            let next = unsafe { Some(ptr::read(self.ptr.add(self.cur_pos))) };
            self.cur_pos += 1;
            next
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let v = self.len - self.cur_pos;
        (v, Some(v))
    }
}

impl<A> FromIterator<A> for Vec1<A> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let mut inner_iter = iter.into_iter();

        let (size, _) = inner_iter.size_hint();
        let mut new_vec: Vec1<A> = Vec1::new_with_size_and_cap(size, size);
        while let Some(v) = inner_iter.next() {
            new_vec.push(v)
        }

        new_vec
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_1() {
        let a = 42;
        let v: Vec1<i32> = Vec1::new_1(a);
        let result = v.at(0);
        assert_eq!(result, &42)
    }

    #[test]
    fn test_push() {
        let mut v = Vec1::new_1(0);
        v.push(1);
        v.push(2);
        v.push(3);

        assert_eq!(v.at(0), &0);
        assert_eq!(v.at(1), &1);
        assert_eq!(v.at(2), &2);
        assert_eq!(v.at(3), &3);
    }

    // iterator trait
    #[test]
    fn test_iterator() {
        let mut v = Vec1::new_1(0);
        v.push(1);
        v.push(2);
        v.push(3);

        let mut iterator = v.into_iter();
        assert_eq!(iterator.next(), Some(0));
        assert_eq!(iterator.next(), Some(1));
        assert_eq!(iterator.next(), Some(2));
        assert_eq!(iterator.next(), Some(3));
        assert_eq!(iterator.next(), None);
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn test_iterator_fold() {
        let mut v = Vec1::new_1(0);
        v.push(1);
        v.push(2);
        v.push(3);

        let result = v.into_iter().fold(0, |sum, cur| sum + cur);

        assert_eq!(result, 6);
    }
}
