use std::{
    cmp::Ordering,
    mem::replace,
    ops::{Range, RangeFull, RangeTo, RangeToInclusive},
};

use bisect::Bisect;
use monoid::Monoid;

/// A struct that can update elements and calculate prefix sums fast.
///
/// # Time complexity
///
/// | Operation   | Complexity |
/// | ----------- | ---------- |
/// | Space       | Θ(n)       |
/// | [`operate`] | Θ(log n)   |
/// | [`fold`]    | Θ(log n)   |
///
/// [`operate`]: FenwickTree::operate
/// [`fold`]: FenwickTree::fold
pub struct FenwickTree<T: Monoid> {
    tree: Vec<T::Set>,
}

pub type BIT<T> = FenwickTree<T>;

pub type BinaryIndexedTree<T> = FenwickTree<T>;

impl<T: Monoid> FenwickTree<T> {
    /// Creates an initialized Fenwick tree that size of `size` with [`<T as Monoid>::id()`].
    ///
    /// [`<T as Monoid>::id()`]: Monoid::id
    pub fn with_size(size: usize) -> Self {
        Self {
            tree: vec![T::id(); size],
        }
    }

    /// Returns the number of elements in the tree.
    pub fn len(&self) -> usize {
        self.tree.len()
    }

    /// Returns `true` if the tree has a length of 0.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Update a tree value with [`Semigroup::operate`].
    ///
    /// This operation is just Θ(log n).
    ///
    /// [`Semigroup::operate`]: monoid::Semigroup::operate
    ///
    /// # Examples
    ///
    /// ```
    /// use fenwick_tree::FenwickTree;
    /// use monoid::types::AddAlge;
    ///
    /// let mut bit: FenwickTree<AddAlge<_>> = vec![1, 2, 3, 4].into();
    /// assert_eq!(bit.fold(..), 10);
    /// bit.operate(2, 5);
    /// assert_eq!(bit.fold(..2), 3);
    /// assert_eq!(bit.fold(..), 15);
    /// ```
    pub fn operate(&mut self, index: usize, value: T::Set) {
        let mut i = index;
        while i < self.len() {
            let val = replace(&mut self.tree[i], T::id());
            self.tree[i] = T::operate(value.clone(), val);
            i += lsb(i + 1);
        }
    }

    /// Returns a folded value.
    /// The `index` can be passed [`RangeTo`], [`RangeToInclusive`] or [`RangeFull`].
    ///
    /// This operation is just Θ(log n).
    ///
    /// # Panics
    ///
    /// May panic if the range is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use fenwick_tree::FenwickTree;
    /// use monoid::types::AddAlge;
    ///
    /// let bit: FenwickTree<AddAlge<_>> = vec![1, 2, 3, 4].into();
    /// assert_eq!(bit.fold(..2), 3);
    /// assert_eq!(bit.fold(..=2), 6);
    /// assert_eq!(bit.fold(..), 10);
    /// ```
    pub fn fold<I: Index<T>>(&self, index: I) -> T::Set {
        index.fold(self)
    }
}

impl<T: Monoid> From<Vec<T::Set>> for BinaryIndexedTree<T> {
    fn from(v: Vec<T::Set>) -> Self {
        let mut ret = Self::with_size(v.len());
        v.into_iter()
            .enumerate()
            .for_each(|(i, x)| ret.operate(i, x));
        ret
    }
}

/// Returns the least significant bit by `i`.
fn lsb(i: usize) -> usize {
    i & i.wrapping_neg()
}

#[cfg(test)]
#[test]
fn lsb_test() {
    for i in 1..=(1e7 as usize) {
        let t = lsb(i);
        assert_eq!(t.trailing_zeros(), i.trailing_zeros());
        assert_eq!(t.count_ones(), 1);
    }
}

pub trait Index<T: Monoid> {
    fn fold(self, tree: &FenwickTree<T>) -> T::Set;
}

impl<T: Monoid> Index<T> for RangeTo<usize> {
    fn fold(self, tree: &FenwickTree<T>) -> T::Set {
        let mut ret = T::id();
        let mut i = self.end;
        while i > 0 {
            ret = T::operate(tree.tree[i - 1].clone(), ret);
            i -= lsb(i);
        }
        ret
    }
}

impl<T: Monoid> Index<T> for RangeToInclusive<usize> {
    fn fold(self, tree: &FenwickTree<T>) -> <T>::Set {
        (..self.end + 1).fold(tree)
    }
}

impl<T: Monoid> Index<T> for RangeFull {
    fn fold(self, tree: &FenwickTree<T>) -> <T>::Set {
        (..tree.len()).fold(tree)
    }
}

impl<T: Monoid> Bisect for FenwickTree<T> {
    type Item = T::Set;

    fn find_range_by<F>(&self, mut f: F) -> Range<usize>
    where
        F: FnMut(&Self::Item) -> Ordering,
    {
        let mut len = self.len().next_power_of_two();
        let mut i = 0;
        let mut total = T::id();

        while len > 0 {
            if i + len - 1 < self.len() {
                let tmp = T::operate(total.clone(), self.tree[i + len - 1].clone());
                let cmp = f(&tmp);
                match cmp {
                    Ordering::Less => {
                        i += len;
                        total = tmp;
                    }
                    Ordering::Greater => (),
                    Ordering::Equal => break,
                }
            }

            len /= 2;
        }

        let mut lower_i = i;
        let mut lower_total = total.clone();
        let mut upper_i = i;
        let mut upper_total = total;
        while len > 0 {
            let lower_tmp = T::operate(lower_total.clone(), self.tree[i + len - 1].clone());
            let lower_cmp = f(&lower_tmp);
            match lower_cmp {
                Ordering::Less => {
                    lower_i += len;
                    lower_total = lower_tmp;
                }
                Ordering::Equal | Ordering::Greater => (),
            }

            let upper_tmp = T::operate(upper_total.clone(), self.tree[i + len - 1].clone());
            let upper_cmp = f(&upper_tmp);
            match upper_cmp {
                Ordering::Equal | Ordering::Less => {
                    upper_i += len;
                    upper_total = upper_tmp;
                }
                Ordering::Greater => (),
            }

            len /= 2;
        }

        lower_i..upper_i
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use monoid::types::AddAlge;

    #[test]
    fn find_range_by() {
        let bit: FenwickTree<AddAlge<usize>> = vec![1, 2, 3, 4, 5, 6, 6, 8, 9, 10].into();

        assert_eq!(bit.find_range_by(|x| x.cmp(&1)), 0..1);
        assert_eq!(bit.find_range_by(|x| x.cmp(&2)), 1..1);
        assert_eq!(bit.find_range_by(|x| x.cmp(&3)), 1..2);
        (4..=5).for_each(|i| assert_eq!(bit.find_range_by(|x| x.cmp(&i)), 2..2));
        assert_eq!(bit.find_range_by(|x| x.cmp(&6)), 2..3);
        (7..=9).for_each(|i| assert_eq!(bit.find_range_by(|x| x.cmp(&i)), 3..3));
        assert_eq!(bit.find_range_by(|x| x.cmp(&10)), 3..4);
        (11..=14).for_each(|i| assert_eq!(bit.find_range_by(|x| x.cmp(&i)), 4..4));
        assert_eq!(bit.find_range_by(|x| x.cmp(&15)), 4..5);
        (16..=20).for_each(|i| assert_eq!(bit.find_range_by(|x| x.cmp(&i)), 5..5));
        assert_eq!(bit.find_range_by(|x| x.cmp(&21)), 5..6);
        (22..=26).for_each(|i| assert_eq!(bit.find_range_by(|x| x.cmp(&i)), 6..6));
        assert_eq!(bit.find_range_by(|x| x.cmp(&27)), 6..7);
        (28..=34).for_each(|i| assert_eq!(bit.find_range_by(|x| x.cmp(&i)), 7..7));
        assert_eq!(bit.find_range_by(|x| x.cmp(&35)), 7..8);
        (36..=43).for_each(|i| assert_eq!(bit.find_range_by(|x| x.cmp(&i)), 8..8));
        assert_eq!(bit.find_range_by(|x| x.cmp(&44)), 8..9);
        (45..=53).for_each(|i| assert_eq!(bit.find_range_by(|x| x.cmp(&i)), 9..9));
        assert_eq!(bit.find_range_by(|x| x.cmp(&54)), 9..10);
        (55..100).for_each(|i| assert_eq!(bit.find_range_by(|x| x.cmp(&i)), 10..10));
    }

    #[test]
    fn partition_point() {
        let bit: FenwickTree<AddAlge<usize>> = vec![1, 2, 3, 4, 5, 6, 6, 8, 9, 10].into();

        assert_eq!(bit.partition_point(|&x| x <= 20), 5);
        assert_eq!(bit.partition_point(|&x| x <= 21), 6);
    }
}
