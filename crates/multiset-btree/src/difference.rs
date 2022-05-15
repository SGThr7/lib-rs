use crate::{merge_iter::MergeIter, BTreeMultiSet, Iter};

pub struct Difference<'a, T: 'a>(MergeIter<Iter<'a, T>>);

impl<T> Difference<'_, T> {
    pub(crate) fn new<'a>(a: &'a BTreeMultiSet<T>, b: &'a BTreeMultiSet<T>) -> Difference<'a, T> {
        Difference(MergeIter::new(a.iter(), b.iter()))
    }
}

impl<'a, T: Ord> Iterator for Difference<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (a_next, b_next) = self.0.nexts(Self::Item::cmp);
            if let (_, None) = (a_next, b_next) {
                return a_next;
            }
        }
    }
}
