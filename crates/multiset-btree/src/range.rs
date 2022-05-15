use super::{BTreeMapRange, BTreeMultiSet, Borrow, FusedIterator, RangeBounds};

#[derive(Clone)]
pub struct Range<'a, T: 'a> {
    iter: BTreeMapRange<'a, T, usize>,
    cur: Option<(&'a T, usize)>,
}

impl<T> Range<'_, T> {
    pub(crate) fn new<I, R>(map: &BTreeMultiSet<T>, range: R) -> Range<'_, T>
    where
        T: Ord + Borrow<I>,
        I: Ord + ?Sized,
        R: RangeBounds<I>,
    {
        let mut iter = map.tree.range(range);
        let cur = iter.next().map(|(x, &i)| (x, i));
        Range { cur, iter }
    }
}

impl<'a, T> Iterator for Range<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((x, count)) = &mut self.cur {
            let ret = *x;
            *count -= 1;

            if count == &0 {
                self.cur = self.iter.next().map(|(x, &i)| (x, i));
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

impl<T> FusedIterator for Range<'_, T> {}
