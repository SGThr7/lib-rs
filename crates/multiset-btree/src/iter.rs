use super::{BTreeMapIter, BTreeMultiSet, FusedIterator};

pub struct Iter<'a, T> {
    iter: BTreeMapIter<'a, T, usize>,
    cur: Option<(&'a T, usize)>,
}

impl<'a, T> Iter<'a, T> {
    pub(crate) fn new(map: &'a BTreeMultiSet<T>) -> Self {
        Self {
            iter: map.tree.iter(),
            cur: None,
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur.is_none() {
            self.cur = self.iter.next().map(|(x, count)| (x, *count));
        }

        if let Some((x, count)) = &mut self.cur {
            let ret = *x;

            // take
            *count -= 1;
            if *count == 0 {
                self.cur = None;
            }

            Some(ret)
        } else {
            None
        }
    }

    fn min(mut self) -> Option<Self::Item> {
        self.next()
    }
}

impl<T> FusedIterator for Iter<'_, T> {}
