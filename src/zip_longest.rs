use std::iter::{ExactSizeIterator, Iterator};

pub(crate) enum EitherOrBoth<A,B> {
    Left(A),
    Right(B),
    Both(A,B),
}

pub(crate) struct Zip<A, B> {
    a: A,
    b: B,
}
impl<A, B> Iterator for Zip<A, B>
where
    A: Iterator,
    B: Iterator,
{
    type Item = EitherOrBoth<A::Item, B::Item>;

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (left_min, left_max) = self.a.size_hint();
        let (right_min, right_max) = self.b.size_hint();
        let lr_max = left_max.zip(right_max).map(|(l, r)| l.max(r));
        (left_min.max(right_min), lr_max.or(left_max).or(right_max))
    }
    fn next(&mut self) -> Option<EitherOrBoth<A::Item, B::Item>> {
        match (self.a.next(), self.b.next()) {
            (None, None) => None,
            (Some(a), None) => Some(EitherOrBoth::Left(a)),
            (Some(a), Some(b)) => Some(EitherOrBoth::Both(a,b)),
            (None, Some(b)) => Some(EitherOrBoth::Right(b)),
        }
    }
}
impl<A, B> ExactSizeIterator for Zip<A, B>
where
    A: ExactSizeIterator,
    B: ExactSizeIterator,
{
    fn len(&self) -> usize {
        self.a.len().min(self.b.len())
    }
}

pub(crate) fn zip_longest<A, B>(a: A, b: B) -> Zip<A, B>
where
    A: Iterator,
    B: Iterator,
{
    Zip { a, b }
}
