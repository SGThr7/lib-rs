use super::{merge_iter::MergeIter, BTreeMultiSet, Iter};

pub struct SymmetricDifference<'a, T: 'a>(MergeIter<Iter<'a, T>>);

impl<T: Ord> SymmetricDifference<'_, T> {
    pub fn new<'a>(a: &'a BTreeMultiSet<T>, b: &'a BTreeMultiSet<T>) -> SymmetricDifference<'a, T> {
        SymmetricDifference(MergeIter::new(a.iter(), b.iter()))
    }
}

impl<'a, T: Ord> Iterator for SymmetricDifference<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (a_next, b_next) = self.0.nexts(Self::Item::cmp);
            if a_next.and(b_next).is_none() {
                return a_next.or(b_next);
            }
        }
    }
}
