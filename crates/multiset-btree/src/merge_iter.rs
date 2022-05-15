use std::{
    cmp::Ordering,
    fmt::{self, Debug},
    iter::FusedIterator,
};

pub struct MergeIter<I: Iterator> {
    a: I,
    b: I,
    peeked: Option<Peeked<I>>,
}

#[derive(Debug, Clone)]
enum Peeked<I: Iterator> {
    A(I::Item),
    B(I::Item),
}

impl<I: Iterator> Clone for MergeIter<I>
where
    I: Clone,
    I::Item: Clone,
{
    fn clone(&self) -> Self {
        Self {
            a: self.a.clone(),
            b: self.b.clone(),
            peeked: self.peeked.clone(),
        }
    }
}

impl<I: Iterator> Debug for MergeIter<I>
where
    I: Debug,
    I::Item: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("MergeIter")
            .field(&self.a)
            .field(&self.b)
            .field(&self.peeked)
            .finish()
    }
}

impl<I: Iterator> MergeIter<I> {
    /// Creates a new merged iterator from two sources.
    pub fn new(a: I, b: I) -> Self {
        MergeIter { a, b, peeked: None }
    }

    pub fn nexts<Cmp: Fn(&I::Item, &I::Item) -> Ordering>(
        &mut self,
        cmp: Cmp,
    ) -> (Option<I::Item>, Option<I::Item>)
    where
        I: FusedIterator,
    {
        let (mut a_next, mut b_next) = match self.peeked.take() {
            Some(Peeked::A(next)) => (Some(next), self.b.next()),
            Some(Peeked::B(next)) => (self.a.next(), Some(next)),
            None => (self.a.next(), self.b.next()),
        };
        if let (Some(ref a), Some(ref b)) = (&a_next, &b_next) {
            match cmp(a, b) {
                Ordering::Less => self.peeked = b_next.take().map(Peeked::B),
                Ordering::Greater => self.peeked = a_next.take().map(Peeked::A),
                Ordering::Equal => (),
            }
        }
        (a_next, b_next)
    }
}
