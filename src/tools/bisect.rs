#[codesnip::entry("Bisect")]
pub use bisect_impl::{Bisect, RangeBisect};

#[codesnip::entry("Bisect")]
mod bisect_impl {
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
            (..self.len()).find_range_by(|i| f(unsafe { self.get_unchecked(i) }))
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
            let start = match self.start_bound() {
                Included(i) => *i,
                Excluded(i) => i + 1,
                Unbounded => core::usize::MIN,
            };
            let end = match self.end_bound() {
                Included(i) => i + 1,
                Excluded(i) => *i,
                Unbounded => core::usize::MAX,
            };

            let mut size = end - start;
            let mut lower = (start, end);
            let mut upper = (start, end);
            while size >= 1 {
                let mid_lower = size / 2 + lower.0;
                match f(mid_lower) {
                    Less => lower.0 = mid_lower + 1,
                    Equal | Greater => lower.1 = mid_lower,
                }

                let mid_upper = size / 2 + upper.0;
                match f(mid_upper) {
                    Less | Equal => upper.0 = mid_upper + 1,
                    Greater => upper.1 = mid_upper,
                }

                size /= 2;
            }
            lower.0..upper.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounds() {
        // 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 17, 17, 17, 17, 17, 17, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
        let v = (2..33)
            .map(|x| if (13..19).contains(&x) { 17 } else { x })
            .collect::<Vec<_>>();
        for i in 0..35 {
            // dbg!(i, v.iter().position(|&x| x == i));
            let l = v
                .iter()
                .position(|&x| x == i)
                .unwrap_or(v.binary_search(&i).err().unwrap_or_default());
            let r = v
                .iter()
                .rposition(|&x| x == i)
                .and_then(|x| Some(x + 1))
                .unwrap_or_else(|| v.binary_search(&i).err().unwrap());
            assert_eq!(l, v.lower_bound(&i), "lower_bound, i: {}", i);
            assert_eq!(r, v.upper_bound(&i), "upper_bound, i: {}", i);
            assert_eq!(l..r, v.find_range(&i), "find_range, i: {}", i);
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
