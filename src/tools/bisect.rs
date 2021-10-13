#[codesnip::entry("Bisect")]
pub trait Bisect<T> {
    fn lower_bound(&self, x: &T) -> usize
    where
        T: core::cmp::Ord,
    {
        self.lower_bound_by(|y| y.cmp(x))
    }

    fn lower_bound_by<'a, F>(&'a self, f: F) -> usize
    where
        T: 'a,
        F: FnMut(&'a T) -> core::cmp::Ordering;

    fn upper_bound(&self, x: &T) -> usize
    where
        T: core::cmp::Ord,
    {
        self.upper_bound_by(|y| y.cmp(x))
    }

    fn upper_bound_by<'a, F>(&'a self, f: F) -> usize
    where
        T: 'a,
        F: FnMut(&'a T) -> core::cmp::Ordering;

    fn find_range(&self, x: &T) -> core::ops::Range<usize>
    where
        T: core::cmp::Ord,
    {
        self.find_range_by(|y| y.cmp(x))
    }

    fn find_range_by<'a, F>(&'a self, f: F) -> core::ops::Range<usize>
    where
        T: 'a,
        F: FnMut(&'a T) -> core::cmp::Ordering;

    /// see slice::partition_point
    fn partition_point<'a, F>(&'a self, mut f: F) -> usize
    where
        T: 'a,
        F: FnMut(&'a T) -> bool,
    {
        self.lower_bound_by(|t| {
            if f(t) {
                core::cmp::Ordering::Less
            } else {
                core::cmp::Ordering::Greater
            }
        })
    }
}

#[codesnip::entry("Bisect")]
impl<T> Bisect<T> for [T] {
    fn lower_bound_by<'a, F>(&'a self, mut f: F) -> usize
    where
        T: 'a,
        F: FnMut(&'a T) -> core::cmp::Ordering,
    {
        let mut l = 0;
        let mut r = self.len();
        use core::cmp::Ordering::{Equal, Greater, Less};
        while l < r {
            let mid = (r - l) / 2 + l;
            let cmp = f(unsafe { self.get_unchecked(mid) });
            match cmp {
                Less => l = mid + 1,
                Equal | Greater => r = mid,
            }
        }
        l
    }

    fn upper_bound_by<'a, F>(&'a self, mut f: F) -> usize
    where
        T: 'a,
        F: FnMut(&'a T) -> core::cmp::Ordering,
    {
        let mut l = 0;
        let mut r = self.len();
        use core::cmp::Ordering::{Equal, Greater, Less};
        while l < r {
            let mid = (r - l) / 2 + l;
            let cmp = f(unsafe { self.get_unchecked(mid) });
            match cmp {
                Less | Equal => l = mid + 1,
                Greater => r = mid,
            }
        }
        r
    }

    fn find_range_by<'a, F>(&'a self, mut f: F) -> core::ops::Range<usize>
    where
        T: 'a,
        F: FnMut(&'a T) -> core::cmp::Ordering,
    {
        let mut size = self.len();
        let mut lower = (0, self.len());
        let mut upper = (0, self.len());
        use core::cmp::Ordering::{Equal, Greater, Less};
        while size >= 1 {
            let mid_lower = size / 2 + lower.0;
            let cmp_lower = f(unsafe { self.get_unchecked(mid_lower) });
            match cmp_lower {
                Less => lower.0 = mid_lower + 1,
                Equal | Greater => lower.1 = mid_lower,
            }

            let mid_upper = size / 2 + upper.0;
            let cmp_upper = f(unsafe { self.get_unchecked(mid_upper) });
            match cmp_upper {
                Less | Equal => upper.0 = mid_upper + 1,
                Greater => upper.1 = mid_upper,
            }

            size /= 2;
        }
        lower.0..upper.0
    }
}

#[cfg(test)]
mod test_bisect {
    use crate::tools::bisect::Bisect;

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
            let r = {
                let t = v.iter().rposition(|&x| x == i);
                if let Some(t) = t {
                    t + 1
                } else {
                    v.binary_search(&i).err().unwrap()
                }
            };
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
}