use std::{
    cmp::Ordering,
    i128, i16, i32, i64, i8, isize,
    ops::{Bound, Range, RangeBounds},
    u128, u16, u32, u64, u8, usize,
};

/// Trait to implement [Binary Search Algorithm](https://en.wikipedia.org/wiki/Binary_search_algorithm).
pub trait Bisect {
    /// The type of the array elements.
    type Item;

    /// Find a range with a comparator function.
    ///
    /// The array must be sorted by ascending order to the comparison function passed in.
    ///
    /// # Examples
    ///
    /// ```
    /// use bisect::Bisect;
    ///
    /// let v = [1, 2, 3, 4, 4, 4, 6, 7, 7, 8];
    ///
    /// assert_eq!(v.find_range_by(|x| x.cmp(&4)), 3..6);
    /// assert_eq!(v.find_range_by(|x| x.cmp(&5)), 6..6);
    /// assert_eq!(v.find_range_by(|x| x.cmp(&6)), 6..7);
    /// ```
    fn find_range_by<F>(&self, f: F) -> Range<usize>
    where
        F: FnMut(&Self::Item) -> Ordering;

    /// Find a range.
    ///
    /// # Examples
    ///
    /// ```
    /// use bisect::Bisect;
    ///
    /// let v = [1, 2, 3, 4, 4, 4, 6, 7, 7, 8];
    ///
    /// assert_eq!(v.find_range(&4), 3..6);
    /// assert_eq!(v.find_range(&5), 6..6);
    /// assert_eq!(v.find_range(&6), 6..7);
    /// ```
    fn find_range(&self, x: &Self::Item) -> Range<usize>
    where
        Self::Item: Ord,
    {
        self.find_range_by(|y| y.cmp(x))
    }

    /// Find a range with a key extraction function.
    ///
    /// # Examples
    ///
    /// ```
    /// use bisect::Bisect;
    ///
    /// let v = [(2, 0), (1, 1), (4, 1), (7, 2), (4, 3),
    ///          (8, 3), (3, 8), (6, 13), (4, 20), (7, 57)];
    ///
    /// assert_eq!(v.find_range_by_key(&3, |&(a, b)| b), 4..6);
    /// assert_eq!(v.find_range_by_key(&19, |&(a, b)| b), 8..8);
    /// assert_eq!(v.find_range_by_key(&57, |&(a, b)| b), 9..10);
    /// ```
    fn find_range_by_key<F, U>(&self, x: &U, mut f: F) -> Range<usize>
    where
        F: FnMut(&Self::Item) -> U,
        U: Ord,
    {
        self.find_range_by(|y| f(y).cmp(x))
    }

    /// Returns the index of insertion point to maintain sorted order.
    ///
    /// If the value is contained in the array, returns the previous index of all existing values.
    ///
    /// # Examples
    ///
    /// ```
    /// use bisect::Bisect;
    ///
    /// let v = [1, 2, 3, 4, 4, 4, 6, 7, 7, 8];
    ///
    /// assert_eq!(v.lower_bound(&4), 3);
    /// assert_eq!(v.lower_bound(&5), 6);
    /// assert_eq!(v.lower_bound(&6), 6);
    /// ```
    fn lower_bound(&self, x: &Self::Item) -> usize
    where
        Self::Item: Ord,
    {
        self.lower_bound_by(|y| y.cmp(x))
    }

    /// Returns the index of insertion point to maintain sorted order with a comparator function.
    ///
    /// If the value is contained in the array, returns the previous index of all existing values.
    ///
    /// # Examples
    ///
    /// ```
    /// use bisect::Bisect;
    ///
    /// let v = [1, 2, 3, 4, 4, 4, 6, 7, 7, 8];
    ///
    /// assert_eq!(v.lower_bound_by(|&x| x.cmp(&4)), 3);
    /// assert_eq!(v.lower_bound_by(|&x| x.cmp(&5)), 6);
    /// assert_eq!(v.lower_bound_by(|&x| x.cmp(&6)), 6);
    /// ```
    fn lower_bound_by<F>(&self, f: F) -> usize
    where
        F: FnMut(&Self::Item) -> Ordering,
    {
        self.find_range_by(f).start
    }

    /// Returns the index of insertion point to maintain sorted order with a key extraction function.
    ///
    /// If the value is contained in the array, returns the previous index of all existing values.
    ///
    /// # Examples
    ///
    /// ```
    /// use bisect::Bisect;
    ///
    /// let v = [(2, 0), (1, 1), (4, 1), (7, 2), (4, 3),
    ///          (8, 3), (3, 8), (6, 13), (4, 20), (7, 57)];
    ///
    /// assert_eq!(v.lower_bound_by_key(&3, |&(a, b)| b), 4);
    /// assert_eq!(v.lower_bound_by_key(&19, |&(a, b)| b), 8);
    /// assert_eq!(v.lower_bound_by_key(&57, |&(a, b)| b), 9);
    /// ```
    fn lower_bound_by_key<U, F>(&self, x: &U, mut f: F) -> usize
    where
        U: Ord,
        F: FnMut(&Self::Item) -> U,
    {
        self.lower_bound_by(|y| f(y).cmp(x))
    }

    /// Returns the index of insertion point to maintain sorted order.
    ///
    /// If the value is contained in the array, returns the next index of all existing values.
    ///
    /// # Examples
    ///
    /// ```
    /// use bisect::Bisect;
    ///
    /// let v = [1, 2, 3, 4, 4, 4, 6, 7, 7, 8];
    ///
    /// assert_eq!(v.upper_bound(&4), 6);
    /// assert_eq!(v.upper_bound(&5), 6);
    /// assert_eq!(v.upper_bound(&6), 7);
    /// ```
    fn upper_bound(&self, x: &Self::Item) -> usize
    where
        Self::Item: Ord,
    {
        self.upper_bound_by(|y| y.cmp(x))
    }

    /// Returns the index of insertion point to maintain sorted order with a comparator function.
    ///
    /// If the value is contained in the array, returns the next index of all existing values.
    ///
    /// # Examples
    ///
    /// ```
    /// use bisect::Bisect;
    ///
    /// let v = [1, 2, 3, 4, 4, 4, 6, 7, 7, 8];
    ///
    /// assert_eq!(v.upper_bound_by(|&x| x.cmp(&4)), 6);
    /// assert_eq!(v.upper_bound_by(|&x| x.cmp(&5)), 6);
    /// assert_eq!(v.upper_bound_by(|&x| x.cmp(&6)), 7);
    /// ```
    fn upper_bound_by<F>(&self, f: F) -> usize
    where
        F: FnMut(&Self::Item) -> Ordering,
    {
        self.find_range_by(f).end
    }

    /// Returns the index of insertion point to maintain sorted order with a key extraction function.
    ///
    /// If the value is contained in the array, returns the next index of all existing values.
    ///
    /// # Examples
    ///
    /// ```
    /// use bisect::Bisect;
    ///
    /// let v = [(2, 0), (1, 1), (4, 1), (7, 2), (4, 3),
    ///          (8, 3), (3, 8), (6, 13), (4, 20), (7, 57)];
    ///
    /// assert_eq!(v.upper_bound_by_key(&3, |&(a, b)| b), 6);
    /// assert_eq!(v.upper_bound_by_key(&19, |&(a, b)| b), 8);
    /// assert_eq!(v.upper_bound_by_key(&57, |&(a, b)| b), 10);
    /// ```
    fn upper_bound_by_key<U, F>(&self, x: &U, mut f: F) -> usize
    where
        U: Ord,
        F: FnMut(&Self::Item) -> U,
    {
        self.upper_bound_by(|y| f(y).cmp(x))
    }

    /// Returns the index of partition point according to the given predicate.
    ///
    /// This function is equivalent to [`partition_point`].
    ///
    /// [`partition_point`]: [!]::partition_point
    ///
    /// # Example
    ///
    /// ```
    /// use bisect::Bisect;
    ///
    /// let v = [1, 2, 3, 3, 5, 6, 7];
    ///
    /// // before 1.52.0
    /// // let i = v.partition_point(|&x| x < 5);
    /// // 1.52.0 or later
    /// let i = <[usize] as Bisect>::partition_point(&v, |&x| x < 5);
    ///
    /// assert_eq!(i, 4);
    /// assert!(v[..i].iter().all(|&x| x < 5));
    /// assert!(v[i..].iter().all(|&x| !(x < 5)));
    /// ```
    fn partition_point<F>(&self, mut f: F) -> usize
    where
        F: FnMut(&Self::Item) -> bool,
    {
        self.lower_bound_by(|t| {
            if f(t) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        })
    }
}

impl<T> Bisect for [T] {
    type Item = T;
    fn find_range_by<F>(&self, mut f: F) -> Range<usize>
    where
        F: FnMut(&T) -> Ordering,
    {
        // SAFETY: 0 <= i < length
        (0..self.len()).find_range_by(|i| f(unsafe { self.get_unchecked(i) }))
    }
}

/// The trait to implement [Binary Search Algorithm](https://en.wikipedia.org/wiki/Binary_search_algorithm) for [`RangeBounds`].
pub trait RangeBisect<Idx> {
    /// Returns the range of values for which the passed function returned [`Ordering::Equal`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bisect::RangeBisect;
    /// let i = (7_usize..13).find_range_by(|i| (i / 10).cmp(&1));
    ///
    /// assert_eq!(i, 10..13);
    /// ```
    fn find_range_by<F: FnMut(Idx) -> Ordering>(&self, f: F) -> Range<Idx>;

    /// Returns the first index of which the passed function returned [`Ordering::Equal`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bisect::RangeBisect;
    /// let i = (7_usize..13).lower_bound_by(|i| (i / 10).cmp(&1));
    ///
    /// assert_eq!(i, 10);
    /// ```
    fn lower_bound_by<F: FnMut(Idx) -> Ordering>(&self, f: F) -> Idx {
        self.find_range_by(f).start
    }

    /// Returns the next index after all indices for which the passed function returned [`Ordering::Equal`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bisect::RangeBisect;
    /// let i = (7_usize..13).upper_bound_by(|i| (i / 10).cmp(&1));
    ///
    /// assert_eq!(i, 13);
    /// ```
    fn upper_bound_by<F: FnMut(Idx) -> Ordering>(&self, f: F) -> Idx {
        self.find_range_by(f).end
    }

    /// Returns the index of partition point according to the given predicate.
    ///
    /// This function is equivalent to [`partition_point`] in std library.
    ///
    /// The given predicate is assumed to be able to separate the range.
    /// This means that all values after which the given predicate returns false are returned false by the given predicate.
    /// For example, `7..15` can be partitioned under the predicate `x < 13`.
    ///
    /// This function is useful to compute the maximum value that satisfies a given predicate in a given function.
    ///
    /// [`partition_point`]: [!]::partition_point
    ///
    /// # Examples
    ///
    /// ```
    /// use bisect::RangeBisect;
    ///
    /// let target = 1700;
    /// let max = 1e7 as usize;
    ///
    /// let f = |x: usize| (x + 1) * x / 2;
    /// let i = (..max).partition_point(|x| f(x) < target);
    ///
    /// assert!((0..i).all(|x| f(x) < target));
    /// assert!((i..max).all(|x| !(f(x) < target)));
    /// assert_eq!(i, 57 + 1);
    /// ```
    fn partition_point<F: FnMut(Idx) -> bool>(&self, mut f: F) -> Idx {
        self.lower_bound_by(|i| {
            if f(i) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        })
    }
}

macro_rules! range_bisect {
    ($($t:tt)*) => {$(
        impl<R: RangeBounds<$t>> RangeBisect<$t> for R {
            fn find_range_by<F: FnMut($t) -> Ordering>(&self, mut f: F) -> Range<$t> {
                let mut start = match self.start_bound() {
                    Bound::Included(i) => *i,
                    Bound::Excluded(i) => i + 1,
                    Bound::Unbounded => $t::MIN,
                };
                let mut end = match self.end_bound() {
                    Bound::Included(i) => i + 1,
                    Bound::Excluded(i) => *i,
                    Bound::Unbounded => $t::MAX,
                };

                // Normal binary search
                let mut mid = start + (end - start) / 2;
                let mut cmp = f(mid);
                while end - start >= 1 && cmp != Ordering::Equal {
                    match cmp {
                        Ordering::Less => start = mid + 1,
                        Ordering::Greater => end = mid,
                        Ordering::Equal => unreachable!(),
                    }
                    mid = start + (end - start) / 2;
                    cmp = f(mid);
                }

                // Search leftest position
                let mut lower = (start, mid);
                while lower.1 - lower.0 >= 1 {
                    let mid = lower.0 + (lower.1 - lower.0) / 2;
                    match f(mid) {
                        Ordering::Less => lower.0 = mid + 1,
                        Ordering::Equal | Ordering::Greater => lower.1 = mid,
                    }
                }

                // Search rightest position
                let mut upper = (mid, end);
                while upper.1 - upper.0 >= 1 {
                    let mid = upper.0 + (upper.1 - upper.0) / 2;
                    match f(mid) {
                        Ordering::Less | Ordering::Equal => upper.0 = mid + 1,
                        Ordering::Greater => upper.1 = mid,
                    }
                }

                lower.0..upper.0
            }
        }
    )*};
}

range_bisect! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_bisect() {
        let v = [1, 2, 3, 4, 4, 4, 6, 7, 7, 8];

        assert_eq!(v.find_range(&0), 0..0);
        assert_eq!(v.find_range(&1), 0..1);
        assert_eq!(v.find_range(&2), 1..2);
        assert_eq!(v.find_range(&3), 2..3);
        assert_eq!(v.find_range(&4), 3..6);
        assert_eq!(v.find_range(&5), 6..6);
        assert_eq!(v.find_range(&6), 6..7);
        assert_eq!(v.find_range(&7), 7..9);
        assert_eq!(v.find_range(&8), 9..10);
        assert_eq!(v.find_range(&9), 10..10);
    }

    #[test]
    /// [C - MAD TEAM](https://atcoder.jp/contests/zone2021/tasks/zone2021_c)
    fn zone2021_c() {
        use itertools::Itertools;
        use std::collections::BTreeSet;

        fn solver(ar: Vec<Vec<usize>>) -> usize {
            const STATUS_LEN: usize = 5;
            const LIM: usize = 1e9 as usize;

            let ng = (0..=LIM).partition_point(|x| {
                let status_set = ar
                    .iter()
                    .map(|status| {
                        status
                            .iter()
                            .copied()
                            .enumerate()
                            .filter_map(|(i, v)| if v >= x { Some(i) } else { None })
                            .collect::<BTreeSet<_>>()
                    })
                    .collect::<BTreeSet<_>>();
                if status_set.len() < 3 {
                    status_set
                        .into_iter()
                        .fold(BTreeSet::new(), |acc, set| &acc | &set)
                        .len()
                        == STATUS_LEN
                } else {
                    status_set
                        .iter()
                        .tuple_combinations()
                        .any(|(a, b, c)| (&(a | b) | c).len() == STATUS_LEN)
                }
            });
            // collect range (closed): 0..ng
            ng - 1
        }

        let ar = vec![
            vec![3, 9, 6, 4, 6],
            vec![6, 9, 3, 1, 1],
            vec![8, 8, 9, 3, 7],
        ];
        assert_eq!(solver(ar), 4);

        let ar = vec![
            vec![6, 13, 6, 19, 11],
            vec![4, 4, 12, 11, 18],
            vec![20, 7, 19, 2, 5],
            vec![15, 5, 12, 20, 7],
            vec![8, 7, 6, 18, 5],
        ];
        assert_eq!(solver(ar), 13);

        let ar = vec![
            vec![6, 7, 5, 18, 2],
            vec![3, 8, 1, 6, 3],
            vec![7, 2, 8, 7, 7],
            vec![6, 3, 3, 4, 7],
            vec![12, 8, 9, 15, 9],
            vec![9, 8, 6, 1, 10],
            vec![12, 9, 7, 8, 2],
            vec![10, 3, 17, 4, 10],
            vec![3, 1, 3, 19, 3],
            vec![3, 14, 7, 13, 1],
        ];
        assert_eq!(solver(ar), 10);
    }
}
