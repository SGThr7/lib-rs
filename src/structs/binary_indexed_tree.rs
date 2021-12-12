use super::Monoid;

use std::ops::{RangeFull, RangeTo, RangeToInclusive};

/// A data structure that can efficiently operate elements and calculate prefix folds in a vec.
///
/// `BinaryIndexedTree` can be used with any algebraic structure that implements [`Monoid`].
///
/// A `BinaryIndexedTree` can be initialized from a vec.
///
/// ```
/// use lib_rust::structs::binary_indexed_tree::BinaryIndexedTree;
/// use lib_rust::math::num::alge_struct::monoid::AddMonoid;
///
/// let bit = BinaryIndexedTree::<AddMonoid<_>>::from(vec![1, 2, 3]);
/// ```
///
/// # Time complexity
///
/// | Algorithm   | Average     | Worst case  |
/// | ----------- | ----------- | ----------- |
/// | Memory      | O(*n*)      | O(*n*)      |
/// | [`fold`]    | O(log(*n*)) | O(log(*n*)) |
/// | [`operate`] | O(log(*n*)) | O(log(*n*)) |
///
/// [`fold`]: BinaryIndexedTree::fold
/// [`operate`]: BinaryIndexedTree::operate
pub struct BinaryIndexedTree<T: Monoid> {
    tree: Vec<T::Set>,
}

/// Type alias for [`BinaryIndexedTree`].
pub type BIT<T> = BinaryIndexedTree<T>;

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

impl<T: Monoid> BinaryIndexedTree<T> {
    /// Creates an initialized tree with `Monoid::id()`.
    pub fn with_size(size: usize) -> Self {
        Self {
            tree: vec![T::id(); size],
        }
    }

    /// Returns the number of elements in the tree.
    pub fn len(&self) -> usize {
        self.tree.len()
    }

    /// Returns a folded value.
    /// The `index` is allowed with [`RangeTo`], [`RangeToInclusive`] or [`RangeFull`].
    ///
    /// This operation is O(log(*n*)).
    ///
    /// # Panics
    ///
    /// May panic if the range is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use lib_rust::structs::binary_indexed_tree::BinaryIndexedTree;
    /// use lib_rust::math::num::alge_struct::monoid::AddMonoid;
    ///
    /// let bit = BinaryIndexedTree::<AddMonoid<_>>::from(vec![1, 2, 3, 4]);
    /// assert_eq!(bit.fold(..2), 3);
    /// assert_eq!(bit.fold(..=2), 6);
    /// assert_eq!(bit.fold(..), 10);
    /// ```
    pub fn fold<I: BITIndex<T>>(&self, index: I) -> T::Set {
        index.fold(self)
    }

    /// Update vec value with [`Semigroup::operate`].
    ///
    /// This operation is O(log(*n*)).
    ///
    /// [`Semigroup::operate`]: crate::math::num::Semigroup
    ///
    /// # Examples
    ///
    /// ```
    /// # use lib_rust::structs::binary_indexed_tree::BinaryIndexedTree;
    /// use lib_rust::math::num::alge_struct::monoid::AddMonoid;
    ///
    /// let mut bit = BinaryIndexedTree::<AddMonoid<_>>::from(vec![1, 2, 3, 4]);
    /// bit.operate(2, &5);
    /// assert_eq!(bit.fold(..), 15);
    /// ```
    pub fn operate(&mut self, index: usize, value: &T::Set) {
        let mut i = index;
        while i < self.len() {
            self.tree[i] = T::operate(value, &self.tree[i]);
            i += lsb(i + 1);
        }
    }
}

pub trait BITIndex<T: Monoid> {
    fn fold(self, bit: &BinaryIndexedTree<T>) -> T::Set;
}

impl<T: Monoid> BITIndex<T> for RangeTo<usize> {
    fn fold(self, bit: &BinaryIndexedTree<T>) -> T::Set {
        let mut ret = T::id();
        let mut i = self.end;
        while 0 < i {
            ret = T::operate(&bit.tree[i - 1], &ret);
            i -= lsb(i);
        }
        ret
    }
}

impl<T: Monoid> BITIndex<T> for RangeToInclusive<usize> {
    fn fold(self, bit: &BinaryIndexedTree<T>) -> T::Set {
        let to = ..self.end + 1;
        to.fold(bit)
    }
}

impl<T: Monoid> BITIndex<T> for RangeFull {
    fn fold(self, bit: &BinaryIndexedTree<T>) -> T::Set {
        let to = ..bit.len();
        to.fold(bit)
    }
}

impl<T: Monoid> From<Vec<T::Set>> for BinaryIndexedTree<T> {
    fn from(v: Vec<T::Set>) -> Self {
        let mut ret = Self::with_size(v.len());
        v.into_iter()
            .enumerate()
            .for_each(|(i, x)| ret.operate(i, &x));
        ret
    }
}
