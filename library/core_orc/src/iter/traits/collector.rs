use crate::into_iterator::IntoIterator;

/// The type implementing this trait can be constructed from `Iterator`.
/// This is considered to be the reverse behavior of IntoIterator.
pub trait FromIterator<A>: Sized {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self;
}
