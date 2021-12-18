use core::{
    cmp::{
        Ord,
        Ordering::{self, Equal, Greater, Less},
    },
    ops::{
        Bound::{Excluded, Included, Unbounded},
        Range, RangeBounds,
    },
};

pub trait Bisect<T> {
    fn lower_bound(&self, x: &T) -> usize
    where
        T: Ord,
    {
        self.lower_bound_by(|y| y.cmp(x))
    }

    fn lower_bound_by_key<'a, B, F>(&'a self, b: &B, mut f: F) -> usize
    where
        T: 'a,
        F: FnMut(&'a T) -> B,
        B: Ord,
    {
        self.lower_bound_by(|k| f(k).cmp(b))
    }

    fn lower_bound_by<'a, F>(&'a self, f: F) -> usize
    where
        T: 'a,
        F: FnMut(&'a T) -> Ordering,
    {
        self.find_range_by(f).start
    }

    fn upper_bound(&self, x: &T) -> usize
    where
        T: Ord,
    {
        self.upper_bound_by(|y| y.cmp(x))
    }

    fn upper_bound_by_key<'a, B, F>(&'a self, b: &B, mut f: F) -> usize
    where
        T: 'a,
        F: FnMut(&'a T) -> B,
        B: Ord,
    {
        self.upper_bound_by(|k| f(k).cmp(b))
    }

    fn upper_bound_by<'a, F>(&'a self, f: F) -> usize
    where
        T: 'a,
        F: FnMut(&'a T) -> Ordering,
    {
        self.find_range_by(f).end
    }

    fn find_range(&self, x: &T) -> Range<usize>
    where
        T: Ord,
    {
        self.find_range_by(|y| y.cmp(x))
    }

    fn find_range_by_key<'a, B, F>(&'a self, b: &B, mut f: F) -> Range<usize>
    where
        T: 'a,
        F: FnMut(&'a T) -> B,
        B: Ord,
    {
        self.find_range_by(|k| f(k).cmp(b))
    }

    fn find_range_by<'a, F>(&'a self, f: F) -> Range<usize>
    where
        T: 'a,
        F: FnMut(&'a T) -> Ordering;

    /// See [slice::partition_point](https://doc.rust-lang.org/std/primitive.slice.html#method.partition_point).
    /// `slice::partition_point` requires version 1.52 or upper.
    fn partition_point<'a, F>(&'a self, mut f: F) -> usize
    where
        T: 'a,
        F: FnMut(&'a T) -> bool,
    {
        self.lower_bound_by(|t| if f(t) { Less } else { Greater })
    }
}

impl<T> Bisect<T> for [T] {
    fn find_range_by<'a, F>(&'a self, mut f: F) -> Range<usize>
    where
        T: 'a,
        F: FnMut(&'a T) -> Ordering,
    {
        (0..self.len()).find_range_by(|i| f(unsafe { self.get_unchecked(i) }))
    }
}

pub trait RangeBisect<Idx> {
    fn find_range_by<F: FnMut(Idx) -> Ordering>(&self, f: F) -> Range<Idx>;
    fn lower_bound_by<F: FnMut(Idx) -> Ordering>(&self, f: F) -> Idx {
        self.find_range_by(f).start
    }
    fn upper_bound_by<F: FnMut(Idx) -> Ordering>(&self, f: F) -> Idx {
        self.find_range_by(f).end
    }
    fn partition_point<F: FnMut(Idx) -> bool>(&self, mut f: F) -> Idx {
        self.lower_bound_by(|i| if f(i) { Less } else { Greater })
    }
}

impl<R: RangeBounds<usize>> RangeBisect<usize> for R {
    fn find_range_by<F: FnMut(usize) -> Ordering>(&self, mut f: F) -> Range<usize> {
        let mut start = match self.start_bound() {
            Included(i) => *i,
            Excluded(i) => i + 1,
            Unbounded => core::usize::MIN,
        };
        let mut end = match self.end_bound() {
            Included(i) => i + 1,
            Excluded(i) => *i,
            Unbounded => core::usize::MAX,
        };

        let mut mid = start + (end - start) / 2;
        let mut cmp = f(mid);
        while end - start >= 1 && cmp != Equal {
            match cmp {
                Less => start = mid + 1,
                Greater => end = mid,
                Equal => unreachable!(),
            }
            mid = start + (end - start) / 2;
            cmp = f(mid);
        }

        let mut lower = (start, mid);
        while lower.1 - lower.0 >= 1 {
            let mid = lower.0 + (lower.1 - lower.0) / 2;
            match f(mid) {
                Less => lower.0 = mid + 1,
                Equal | Greater => lower.1 = mid,
            }
        }

        let mut upper = (mid, end);
        while upper.1 - upper.0 >= 1 {
            let mid = upper.0 + (upper.1 - upper.0) / 2;
            match f(mid) {
                Less | Equal => upper.0 = mid + 1,
                Greater => upper.1 = mid,
            }
        }

        lower.0..upper.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Reverse;

    #[test]
    fn find_range() {
        let mut v = vec![2, 1, 4, 7, 4, 8, 3, 6, 4, 7];
        v.sort();
        let v = v;

        for i in 0..10 {
            let lans = v
                .iter()
                .position(|&x| x == i)
                .unwrap_or_else(|| v.binary_search(&i).err().unwrap());
            let rans = v
                .iter()
                .rposition(|&x| x == i)
                .and_then(|x| Some(x + 1))
                .unwrap_or_else(|| v.binary_search(&i).err().unwrap());
            assert_eq!(v.find_range(&i), lans..rans, "range, i: {}", i);
        }
    }

    #[test]
    fn bounds_by_key() {
        let mut v = vec![2, 1, 4, 7, 4, 8, 3, 6, 4, 7];
        let map = |x: &usize| Reverse(*x);
        v.sort_by_key(map);
        let v = v;

        for i in 0..10 {
            let lans = v
                .iter()
                .position(|&x| x == i)
                .unwrap_or_else(|| v.binary_search_by_key(&Reverse(i), map).err().unwrap());
            let rans = v
                .iter()
                .rposition(|&x| x == i)
                .and_then(|x| Some(x + 1))
                .unwrap_or_else(|| {
                    v.binary_search_by_key(&Reverse(i), |&x| Reverse(x))
                        .err()
                        .unwrap()
                });
            assert_eq!(
                v.find_range_by_key(&Reverse(i), |&x| Reverse(x)),
                lans..rans,
                "range, i: {}",
                i
            );
        }
    }

    #[test]
    fn partition_point() {
        let v = vec![8, 4, 6, 3, 9, 5, 7, 11, 13];
        assert_eq!(3, v.partition_point(|&x| x % 2 == 0));

        let v = [1, 2, 3, 3, 5, 6, 7];
        let i = v.partition_point(|&x| x < 5);

        assert_eq!(i, 4);
        assert!(v[..i].iter().all(|&x| x < 5));
        assert!(v[i..].iter().all(|&x| !(x < 5)));
    }

    #[test]
    fn range_bisect() {
        assert_eq!((..).partition_point(|i| i * 2 < 13), 7)
    }
}
