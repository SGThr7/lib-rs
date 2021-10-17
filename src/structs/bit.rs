use crate::math::math_structs::monoid;

#[codesnip::entry("BinaryIndexedTree", include("Monoid"))]
pub mod binary_indexed_tree {
    use super::monoid::Monoid;
    use core::{
        fmt::Debug,
        iter::FusedIterator,
        ops::{RangeFull, RangeTo, RangeToInclusive},
        slice,
    };
    use std::vec;

    #[derive(Clone, PartialEq, Eq)]
    pub struct BinaryIndexedTree<T: Monoid> {
        tree: Vec<T::Set>,
    }

    impl<T: Monoid> BinaryIndexedTree<T> {
        pub fn new() -> Self {
            Self { tree: vec![] }
        }

        pub fn with_size(size: usize) -> Self {
            Self {
                tree: vec![T::identity(); size],
            }
        }

        pub fn len(&self) -> usize {
            self.tree.len()
        }

        pub fn capacity(&self) -> usize {
            self.tree.len()
        }

        pub fn push(&mut self, value: T::Set) {
            self.tree
                .push(T::operate(&unsafe { self.get_unchecked(..) }, &value))
        }

        pub fn increase(&mut self, additional: usize) {
            self.tree.reserve(additional);
            for _ in 0..additional {
                self.push(T::identity());
            }
        }

        pub fn get<Q: BITQuery<T>>(&self, query: Q) -> Option<Q::Output> {
            query.query(self)
        }

        pub unsafe fn get_unchecked<Q: BITQuery<T>>(&self, query: Q) -> Q::Output {
            query.query_unchecked(self)
        }

        pub fn operate(&mut self, index: usize, value: &T::Set) {
            let mut i = index;
            while i < self.len() {
                self.tree[i] = T::operate(value, &self.tree[i]);
                i += lsb(i + 1);
            }
        }

        pub fn iter<'a>(&'a self) -> Iter<'a, T> {
            Iter::new(self)
        }

        pub fn lower_bound(&self, x: T::Set) -> usize
        where
            T::Set: PartialOrd,
        {
            let mut len = self.len().next_power_of_two();
            let mut i = 0;
            let mut total = T::identity();
            while 0 < len {
                if i + len - 1 < self.len() && T::operate(&total, &self.tree[i + len - 1]) < x {
                    total = T::operate(&total, &self.tree[i + len - 1]);
                    i += len;
                }
                len >>= 1;
            }
            i
        }
    }

    impl<T> Debug for BinaryIndexedTree<T>
    where
        T: Monoid,
        T::Set: Debug,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_list().entries(self.iter()).finish()
        }
    }

    impl<T: Monoid> Default for BinaryIndexedTree<T> {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<T> From<Vec<T::Set>> for BinaryIndexedTree<T>
    where
        T: Monoid,
    {
        fn from(v: Vec<T::Set>) -> Self {
            let mut res = Self::with_size(v.len());
            for (i, x) in v.into_iter().enumerate() {
                res.operate(i, &x.clone().into());
            }
            res
        }
    }

    /// Least Significant Bit
    fn lsb(i: usize) -> usize {
        i & i.wrapping_neg()
    }

    pub fn query<T: Monoid>(bit: &BinaryIndexedTree<T>, exclusive_bound: usize) -> Option<T::Set> {
        if exclusive_bound <= bit.len() {
            Some(unsafe { query_unchecked(bit, exclusive_bound) })
        } else {
            None
        }
    }

    pub unsafe fn query_unchecked<T: Monoid>(
        bit: &BinaryIndexedTree<T>,
        exclusive_bound: usize,
    ) -> T::Set {
        let mut ret = T::identity();
        let mut i = exclusive_bound;
        while 0 < i {
            ret = T::operate(&bit.tree[i - 1], &ret);
            i -= lsb(i);
        }
        ret
    }

    pub trait BITQuery<T: Monoid> {
        type Output: Clone;
        fn query(self, bit: &BinaryIndexedTree<T>) -> Option<Self::Output>;
        unsafe fn query_unchecked(self, bit: &BinaryIndexedTree<T>) -> Self::Output;
    }

    impl<T: Monoid> BITQuery<T> for RangeTo<usize> {
        type Output = T::Set;

        fn query(self, bit: &BinaryIndexedTree<T>) -> Option<Self::Output> {
            query(bit, self.end)
        }
        unsafe fn query_unchecked(self, bit: &BinaryIndexedTree<T>) -> Self::Output {
            query_unchecked(bit, self.end)
        }
    }

    impl<T: Monoid> BITQuery<T> for RangeToInclusive<usize> {
        type Output = T::Set;

        fn query(self, bit: &BinaryIndexedTree<T>) -> Option<Self::Output> {
            query(bit, self.end + 1)
        }
        unsafe fn query_unchecked(self, bit: &BinaryIndexedTree<T>) -> Self::Output {
            query_unchecked(bit, self.end + 1)
        }
    }

    impl<T: Monoid> BITQuery<T> for RangeFull {
        type Output = T::Set;

        fn query(self, bit: &BinaryIndexedTree<T>) -> Option<Self::Output> {
            (..bit.len()).query(bit)
        }
        unsafe fn query_unchecked(self, bit: &BinaryIndexedTree<T>) -> Self::Output {
            (..bit.len()).query_unchecked(bit)
        }
    }

    pub struct IntoIter<T: Monoid> {
        iter: vec::IntoIter<T::Set>,
        index: usize,
        memo: Vec<T::Set>,
    }

    impl<T: Monoid> Iterator for IntoIter<T> {
        type Item = T::Set;
        fn next(&mut self) -> Option<Self::Item> {
            if let Some(value) = self.iter.next() {
                let i = self.index + 1;
                if i.is_power_of_two() {
                    // 2^nの時はlsb(index)がmemo.len()より大きいからpushする
                    self.memo.push(value);
                } else {
                    // 古いのは使わないので上書き
                    self.memo[lsb(i).trailing_zeros() as usize] = value;
                }
                self.index += 1;

                let mut i = i;
                let mut ret = T::identity();
                while 0 < i {
                    ret = T::operate(&self.memo[i.trailing_zeros() as usize], &ret);
                    i -= lsb(i);
                }
                Some(ret)
            } else {
                None
            }
        }
    }

    impl<T: Monoid> FusedIterator for IntoIter<T> {}

    impl<T: Monoid> IntoIterator for BinaryIndexedTree<T> {
        type Item = T::Set;
        type IntoIter = IntoIter<T>;
        fn into_iter(self) -> Self::IntoIter {
            IntoIter {
                iter: self.tree.into_iter(),
                index: 0,
                memo: vec![],
            }
        }
    }

    pub struct Iter<'a, T: Monoid> {
        iter: slice::Iter<'a, T::Set>,
        index: usize,
        memo: Vec<&'a T::Set>,
    }

    impl<'a, T: Monoid> Iter<'a, T> {
        pub fn new(bit: &'a BinaryIndexedTree<T>) -> Self {
            Self {
                iter: bit.tree.iter(),
                index: 0,
                memo: vec![],
            }
        }
    }

    impl<'a, T: Monoid> Iterator for Iter<'a, T> {
        type Item = T::Set;

        fn next(&mut self) -> Option<Self::Item> {
            if let Some(value) = self.iter.next() {
                let i = self.index + 1;
                if i.is_power_of_two() {
                    // 2^nの時はlsb(index)がmemo.len()より大きいからpushする
                    self.memo.push(value);
                } else {
                    // 古いのは使わないので上書き
                    self.memo[lsb(i).trailing_zeros() as usize] = value;
                }
                self.index += 1;

                let mut i = i;
                let mut ret = T::identity();
                while 0 < i {
                    ret = T::operate(self.memo[i.trailing_zeros() as usize], &ret);
                    i -= lsb(i);
                }
                Some(ret)
            } else {
                None
            }
        }
    }

    impl<'a, T: Monoid> FusedIterator for Iter<'a, T> {}

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::math::math_structs::monoid::AddMonoid;
        type BITree<T> = BinaryIndexedTree<AddMonoid<T>>;

        #[test]
        fn debug() {
            let bit = BITree::from(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
            // [1, 3, 6, 10, 15, 21, 28, 36, 45]
            println!("{:?}", bit);
        }

        #[test]
        fn lsb() {
            for i in 1..(1e7 as usize) {
                let t = super::lsb(i);
                assert_eq!(i.trailing_zeros(), t.trailing_zeros());
                assert_eq!(1, t.count_ones());
            }
        }

        #[test]
        fn from() {
            let bit = BITree::from(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
            assert_eq!(vec![1, 3, 3, 10, 5, 11, 7, 36, 9], bit.tree);
        }

        #[test]
        fn get() {
            let bit = BITree::from(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
            assert_eq!(
                vec![0, 1, 3, 6, 10, 15, 21, 28, 36, 45],
                (0..=bit.len())
                    .map(|i| bit.get(..i).unwrap())
                    .collect::<Vec<_>>()
            );
        }

        #[test]
        fn into_iter() {
            let bit = BITree::from(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
            assert_eq!(
                vec![1, 3, 6, 10, 15, 21, 28, 36, 45],
                bit.into_iter().collect::<Vec<_>>()
            );
        }

        #[test]
        fn iter() {
            let bit = BITree::from(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
            assert_eq!(
                vec![1, 3, 6, 10, 15, 21, 28, 36, 45],
                bit.iter().collect::<Vec<_>>()
            );
        }

        #[test]
        fn lower_bound() {
            let t = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
            let ans = {
                vec![0]
                    .into_iter()
                    .chain(t.iter().enumerate().map(|(i, &x)| vec![i; x]).flatten())
                    .chain(vec![t.len(); 2])
                    .collect::<Vec<_>>()
            };
            let bit = BITree::from(t);
            for (i, ans) in ans.into_iter().enumerate() {
                assert_eq!(ans, bit.lower_bound(i), "i={}", i);
            }
        }
    }
}
