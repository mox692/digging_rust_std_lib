use super::super::traits::iterator::Iterator;
pub struct Map<A, F> {
    inner: A,
    f: F,
}

// impl<A: Iterator, F> Iterator for Map<A, F> {
//     fn size_hint(&self) -> (usize, Option<usize>) {
//         (self.inner.size_hint(), Some(self.inner.size_hint()))
//     }
// }

impl<A: Iterator, B, F: FnMut(A::Item) -> B> Map<A, F> {
    pub fn new(inner: A, f: F) -> Self {
        Map { inner: inner, f: f }
    }
}

impl<A: Iterator, B, F: FnMut(A::Item) -> B> Iterator for Map<A, F> {
    type Item = B;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some(n) => Some((self.f)(n)),
            None => None,
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

#[cfg(test)]
mod test {
    use crate::{into_iterator::IntoIterator, iter::traits::iterator::Iterator};

    use super::super::super::super::vec1::Vec1;

    #[test]
    fn test_iterator_next() {
        let mut v = Vec1::new_1(0);
        v.push(1);
        v.push(2);
        v.push(3);

        let v = v.into_iter().map(|x| x * 2).collect::<Vec1<_>>();

        assert_eq!(v.at(0), &0);
        assert_eq!(v.at(1), &2);
        assert_eq!(v.at(2), &4);
        assert_eq!(v.at(3), &6);
    }
}
