use std::{
    mem::replace,
    ops::{RangeFull, RangeTo, RangeToInclusive},
};

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
    /// use monoid::types::AddMonoid;
    ///
    /// let mut bit: FenwickTree<AddMonoid<_>> = vec![1, 2, 3, 4].into();
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
    /// use monoid::types::AddMonoid;
    ///
    /// let bit: FenwickTree<AddMonoid<_>> = vec![1, 2, 3, 4].into();
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
