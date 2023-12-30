use super::super::adaptors::map::Map;
use super::collector::FromIterator;

pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    fn fold<B, F>(mut self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        let mut cur_sum = init;
        while let Some(v) = Self::next(&mut self) {
            cur_sum = f(cur_sum, v);
        }
        cur_sum
    }

    // TODO: impl
    fn map<B, F>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> B,
    {
        Map::new(self, f)
    }

    fn collect<B: FromIterator<Self::Item>>(self) -> B
    where
        Self: Sized,
    {
        FromIterator::from_iter(self)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}
