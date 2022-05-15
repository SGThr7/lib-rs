use super::{merge_iter::MergeIter, BTreeMultiSet, Iter};

pub struct Union<'a, T: 'a> {
    inner: MergeIter<Iter<'a, T>>,
    dup: Option<&'a T>,
}

impl<T: Ord> Union<'_, T> {
    pub fn new<'a>(a: &'a BTreeMultiSet<T>, b: &'a BTreeMultiSet<T>) -> Union<'a, T> {
        Union {
            inner: MergeIter::new(a.iter(), b.iter()),
            dup: None,
        }
    }
}

impl<'a, T: Ord> Iterator for Union<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.dup.is_some() {
            self.dup.take()
        } else {
            let (a_next, b_next) = self.inner.nexts(Self::Item::cmp);
            match (a_next, b_next) {
                (Some(_), Some(_)) => {
                    self.dup = b_next;
                    a_next
                }
                _ => (a_next.or(b_next)),
            }
        }
    }
}
