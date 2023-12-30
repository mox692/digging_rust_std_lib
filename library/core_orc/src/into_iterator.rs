use crate::iter::traits::iterator::Iterator;

/// consume `self`, and convert to `Self::Item`, which should behave `Iterator`.
/// This is considered to be the reverse behavior of FromIterator.
pub trait IntoIterator {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter;
}

impl<I: Iterator> IntoIterator for I {
    type Item = I::Item;
    type IntoIter = I;

    // Since `I` is Iterator, we just return `self`
    #[inline]
    fn into_iter(self) -> I {
        self
    }
}
