use super::{merge_iter::MergeIter, BTreeMultiSet, Iter};

pub struct Intersection<'a, T: 'a>(MergeIter<Iter<'a, T>>);

impl<T> Intersection<'_, T> {
    pub fn new<'a>(a: &'a BTreeMultiSet<T>, b: &'a BTreeMultiSet<T>) -> Intersection<'a, T> {
        Intersection(MergeIter::new(a.iter(), b.iter()))
    }
}

impl<'a, T: Ord> Iterator for Intersection<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (a_next, b_next) = self.0.nexts(Self::Item::cmp);
            match (a_next, b_next) {
                // `Some(_), Some(_)`: Found `a_next == b_next`
                // `None, None`: Reached the end
                (Some(_), Some(_)) | (None, None) => return a_next,
                // Others, find the elements that `a_next == b_next`
                _ => (),
            }
        }
    }
}
