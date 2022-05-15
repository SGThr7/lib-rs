use super::{BTreeMapIntoIter, BTreeMultiSet, FusedIterator};

pub struct IntoIter<T> {
    iter: BTreeMapIntoIter<T, usize>,
    cur: Option<(T, usize)>,
}

impl<T> IntoIter<T> {
    pub(crate) fn new(map: BTreeMultiSet<T>) -> Self {
        let mut iter = map.tree.into_iter();
        let cur = iter.next();
        Self { iter, cur }
    }
}

impl<T: Clone> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur.is_none() {
            self.cur = self.iter.next().map(|(x, count)| (x, count));
        }

        if let Some((x, count)) = &mut self.cur {
            let ret = x.clone();

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

impl<T: Clone> FusedIterator for IntoIter<T> {}
