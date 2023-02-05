use std::{
    ops::{Bound, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive},
    slice::SliceIndex,
};

use monoid::Monoid;

pub struct SegmentTree<M: Monoid> {
    tree: Vec<M::Set>,
    leaf_len: usize,
}

impl<M: Monoid> SegmentTree<M> {
    pub fn with_size(size: usize) -> Self {
        let leaf_len = size;
        let tree_len = leaf_len * 2 - 1;
        let tree = {
            let mut ret = Vec::with_capacity(tree_len);
            ret.resize_with(tree_len, M::id);
            ret
        };

        Self { tree, leaf_len }
    }

    /// Constructs segment tree from leaves slice.
    fn init(s: &[M::Set], capacity: usize) -> Self
    where
        M::Set: Clone,
    {
        let leaf_len = capacity.max(s.len());
        let tree_len = leaf_len * 2 - 1;
        let mut tree = vec![M::id(); tree_len];

        let vertex_len = tree_len - leaf_len;

        // Clone to leaf
        tree[vertex_len..vertex_len + s.len()].clone_from_slice(s);

        let mut res = Self { tree, leaf_len };
        // Resolve nodes from leaf to root
        for i in (0..vertex_len).rev() {
            res.update_vertex(i);
        }

        res
    }

    #[allow(clippy::len_without_is_empty)]
    /// Returns the number of leaves in the tree.
    pub fn len(&self) -> usize {
        self.leaf_len
    }

    // /// Returns `true` if the tree has a length of 0.
    // pub fn is_empty(&self) -> bool {
    //     self.len() == 0
    // }

    /// Returns the number of nodes in the tree.
    fn tree_len(&self) -> usize {
        self.tree.len()
    }

    /// Returns the number of vertexes in the tree.
    /// This is equivalent to `self.tree_len() - self.leaf_len()`.
    fn vertex_len(&self) -> usize {
        self.tree_len() - self.len()
    }

    /// Calculate node `i` value from its children.
    fn update_vertex(&mut self, i: usize) {
        self.tree[i] = M::operate(&self.tree[i * 2 + 1], &self.tree[i * 2 + 2]);
    }

    /// Update node `i` and its parents.
    fn resolve_node(&mut self, mut i: usize) {
        // Update nodes
        while i > 0 {
            i = (i - 1) / 2;
            self.update_vertex(i);
        }
    }

    // Update leaf `i` and its parents.
    fn resolve_leaf(&mut self, i: usize) {
        let leaf_index = i + self.vertex_len();
        self.resolve_node(leaf_index)
    }

    /// Returns a reference to an element or subslice, without doing bounds checking.
    ///
    /// For safe alternative see [`get`].
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is *[undefined behavior]*
    /// even if the resulting reference is not used.
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    pub unsafe fn get_unchecked<I>(&self, index: I) -> &I::Output
    where
        I: SliceIndex<[M::Set]>,
    {
        self.as_leaf_slice().get_unchecked(index)
    }

    /// Returns a reference to an element or subslice depending on the type of index.
    ///
    /// # Time complexity
    ///
    /// Θ(1)
    ///
    /// # Examples
    ///
    /// ```
    /// use segment_tree::SegmentTree;
    /// use monoid::types::AddAlge;
    ///
    /// let seg_tree: SegmentTree<AddAlge<_>> = vec![1, 2, 3, 4].into();
    ///
    /// assert_eq!(seg_tree.get(0), Some(&1));
    /// assert_eq!(seg_tree.get(1), Some(&2));
    /// assert_eq!(seg_tree.get(2), Some(&3));
    /// assert_eq!(seg_tree.get(3), Some(&4));
    /// assert_eq!(seg_tree.get(4), None);
    ///
    /// assert_eq!(seg_tree.get(1..3).unwrap(), &[2, 3]);
    /// assert_eq!(seg_tree.get(..2).unwrap(), &[1, 2]);
    /// assert_eq!(seg_tree.get(2..).unwrap(), &[3, 4]);
    /// assert_eq!(seg_tree.get(..).unwrap(), &[1, 2, 3, 4]);
    /// ```
    pub fn get<I>(&self, index: I) -> Option<&I::Output>
    where
        I: SliceIndex<[M::Set]>,
    {
        self.as_leaf_slice().get(index)
    }

    /// Sets a value to an element.
    ///
    /// # Time complexity
    ///
    /// Θ(log n)
    ///
    /// # Examples
    ///
    /// ```
    /// use segment_tree::SegmentTree;
    /// use monoid::types::AddAlge;
    ///
    /// let mut seg_tree: SegmentTree<AddAlge<_>> = vec![1, 2, 3, 4].into();
    ///
    /// seg_tree.set(1, 5);
    /// assert_eq!(seg_tree.fold(..), 13);
    /// ```
    pub fn set(&mut self, index: usize, value: M::Set) {
        assert!(index < self.leaf_len);

        self.as_mut_leaf_slice()[index] = value;

        self.resolve_leaf(index);
    }

    pub fn operate(&mut self, index: usize, value: M::Set) {
        assert!(index < self.leaf_len);

        let ref_leaf = unsafe { self.get_unchecked(index) };
        let new_value = M::operate(ref_leaf, &value);

        self.set(index, new_value);
    }

    /// Returns a folded value.
    ///
    /// # Time complexity
    ///
    /// Θ(log n)
    ///
    /// # Panics
    ///
    /// May panic if the range is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use segment_tree::SegmentTree;
    /// use monoid::types::AddAlge;
    /// use std::ops::Bound;
    ///
    /// let seg: SegmentTree<AddAlge<_>> = vec![1, 2, 3, 4].into();
    ///
    /// assert_eq!(seg.fold(..2), 3);
    /// assert_eq!(seg.fold(..=2), 6);
    /// assert_eq!(seg.fold(2..), 7);
    /// assert_eq!(seg.fold(..), 10);
    /// assert_eq!(seg.fold((Bound::Excluded(2), Bound::Unbounded)), 4);
    /// ```
    pub fn fold<I: Index<M>>(&self, index: I) -> M::Set {
        index.fold(self)
    }

    /// Returns a leaves slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use segment_tree::SegmentTree;
    /// use monoid::types::AddAlge;
    ///
    /// let v = vec![1, 2, 3, 4];
    /// let seg: SegmentTree<AddAlge<_>> = v.into();
    ///
    /// assert_eq!(seg.as_leaf_slice(), &v);
    /// ```
    pub fn as_leaf_slice(&self) -> &[M::Set] {
        &self.tree[self.vertex_len()..]
    }

    /// Returns a mutable leaves slice.
    fn as_mut_leaf_slice(&mut self) -> &mut [M::Set] {
        let vertex_len = self.vertex_len();
        &mut self.tree[vertex_len..]
    }
}

impl<M: Monoid> From<&[M::Set]> for SegmentTree<M>
where
    M::Set: Clone,
{
    fn from(s: &[M::Set]) -> Self {
        Self::init(s, s.len())
    }
}

impl<M: Monoid> From<Vec<M::Set>> for SegmentTree<M>
where
    M::Set: Clone,
{
    fn from(v: Vec<M::Set>) -> Self {
        v.as_slice().into()
    }
}

pub trait Index<M: Monoid> {
    fn fold(self, tree: &SegmentTree<M>) -> M::Set;
}

impl<M: Monoid> Index<M> for Range<usize> {
    fn fold(self, tree: &SegmentTree<M>) -> M::Set {
        // Leaf index
        let mut l = self.start + tree.vertex_len();
        let mut r = self.end + tree.vertex_len();

        let mut val_l = M::id();
        let mut val_r = M::id();

        while l < r {
            // Example of fold [1, 5)
            // |                [0, 8)                 |
            // |      [0, 4)       |      [4, 8)       |
            //           |=========|
            // |  [0, 2) |  [2, 4) |  [4, 6) |  [6, 8) |
            //      |====|         |====|
            // |  0 |  1 |  2 |  3 |  4 |  5 |  6 |  7 |
            //      |-------[1, 5)------|
            //
            // Collect [8, 4, 11]
            // |                   0                   | 1
            // |         1         |         2         | 3
            //           |=========|
            // |    3    |    4    |    5    |    6    | 7
            //      |====|         |====|
            // |  7 |  8 |  9 | 10 | 11 | 12 | 13 | 14 | 15
            //

            if l % 2 == 0 {
                // is even
                val_l = M::operate(&val_l, &tree.tree[l]);
            }
            if r % 2 == 0 {
                // is even
                // right-closed range, so collect left (r - 1) one.
                r -= 1;
                val_r = M::operate(&tree.tree[r], &val_r);
            }

            // Climb up tree node
            l /= 2;
            r /= 2;
        }

        M::operate(&val_l, &val_r)
    }
}

impl<M: Monoid> Index<M> for RangeInclusive<usize> {
    fn fold(self, tree: &SegmentTree<M>) -> M::Set {
        Index::fold(*self.start()..self.end() + 1, tree)
    }
}

impl<M: Monoid> Index<M> for RangeFrom<usize> {
    fn fold(self, tree: &SegmentTree<M>) -> M::Set {
        Index::fold(self.start..tree.len(), tree)
    }
}

impl<M: Monoid> Index<M> for RangeTo<usize> {
    fn fold(self, tree: &SegmentTree<M>) -> M::Set {
        Index::fold(0..self.end, tree)
    }
}

impl<M: Monoid> Index<M> for RangeToInclusive<usize> {
    fn fold(self, tree: &SegmentTree<M>) -> M::Set {
        Index::fold(..self.end + 1, tree)
    }
}

impl<M: Monoid> Index<M> for RangeFull {
    fn fold(self, tree: &SegmentTree<M>) -> M::Set {
        Index::fold(0..tree.len(), tree)
    }
}

impl<M: Monoid> Index<M> for usize {
    fn fold(self, tree: &SegmentTree<M>) -> M::Set {
        Index::fold(self..=self, tree)
    }
}

impl<M: Monoid> Index<M> for (Bound<usize>, Bound<usize>) {
    fn fold(self, tree: &SegmentTree<M>) -> M::Set {
        let start = match self.0 {
            Bound::Included(i) => i,
            Bound::Excluded(i) => i + 1,
            Bound::Unbounded => 0,
        };
        let end = match self.1 {
            Bound::Included(i) => i + 1,
            Bound::Excluded(i) => i,
            Bound::Unbounded => tree.len(),
        };

        Index::fold(start..end, tree)
    }
}
