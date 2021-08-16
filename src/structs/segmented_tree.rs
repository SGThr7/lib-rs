use crate::math::monoid::{AddMonoid, MaxMonoid, MinMonoid, Monoid, MulMonoid, XorMonoid};

#[codesnip::entry(include("Monoid"))]
pub struct SegmentedTree<T: Monoid + Copy> {
    tree: Vec<T>,
}

#[codesnip::entry("SegmentedTree", include("Monoid"))]
impl<T: Monoid + Copy> SegmentedTree<T> {
    /// O(n)
    pub fn new(n: usize) -> Self {
        Self::init(n, Vec::<T>::new())
    }

    /// O(n)
    fn init(n: usize, v: Vec<impl Into<T>>) -> Self {
        let n = n.next_power_of_two();
        let size = n * 2 - 1;
        let mut tree = vec![T::e(); size];
        v.into_iter()
            .enumerate()
            .for_each(|(i, x)| tree[i + n - 1] = x.into());
        let mut res = Self { tree };
        res.update_nodes();
        res
    }

    /// O(N)
    /// N: count of leaves
    fn update_nodes(&mut self) {
        let n = self.len();
        let mut len = n / 2;
        let mut i0 = n - 1 - len;
        while len > 0 {
            for i in i0..i0 + len {
                let a = self.tree[i * 2 + 1];
                let b = self.tree[i * 2 + 2];
                self.tree[i] = a.op(b);
            }
            len /= 2;
            i0 -= len;
        }
    }

    /// O(1)
    pub fn len(&self) -> usize {
        (self.tree.len() + 1) / 2
    }

    pub fn get_children(&self, index: usize) -> (T, T) {
        (self.tree[index * 2 + 1], self.tree[index * 2 + 2])
    }

    /// O(log(N))
    /// N: count of leaves
    pub fn set(&mut self, index: usize, value: T) {
        assert!(index < self.len());
        let mut i = index + self.len() - 1;
        self.tree[i] = value;
        while i > 0 {
            i = (i - 1) / 2;
            let (a, b) = self.get_children(i);
            self.tree[i] = a.op(b);
        }
    }

    /// O(log(N))
    /// N: count of leaves
    pub fn operate(&mut self, index: usize, rhs: T) {
        assert!(index < self.len());
        let mut i = index + self.len() - 1;
        self.tree[i] = self.tree[i].op(rhs);
        while i > 0 {
            i = (i - 1) / 2;
            let (a, b) = self.get_children(i);
            self.tree[i] = a.op(b);
        }
    }

    /// O(log(N))
    /// N: count of leaves
    pub fn set_range<R>(&mut self, range: R, value: T)
    where
        R: core::ops::RangeBounds<usize>,
    {
        use core::ops::Bound::{Excluded, Included, Unbounded};
        let start = match range.start_bound() {
            Included(&l) => l,
            Excluded(&l) => l + 1,
            Unbounded => 0,
        };
        let end = match range.end_bound() {
            Included(&r) => r + 1,
            Excluded(&r) => r,
            Unbounded => self.len(),
        };
        assert!(end <= self.len());

        let pad = self.len() - 1;
        let mut parentq = std::collections::VecDeque::with_capacity(self.len());
        for i in start + pad..end + pad {
            self.tree[i] = value;
            // parent
            let p = i / 2;
            if parentq.back() != Some(&p) {
                parentq.push_back(p);
            }
        }
        // update nodes
        while let Some(p) = parentq.pop_front() {
            let (a, b) = self.get_children(p);
            self.tree[p] = a.op(b);
            if p == 0 {
                break;
            }
            // grand parent
            let gp = p / 2;
            if parentq.back() != Some(&gp) {
                parentq.push_back(gp);
            }
        }
    }

    /// O(log(N))
    /// N: count of leaves
    pub fn query<R>(&self, range: R) -> Option<T>
    where
        R: core::ops::RangeBounds<usize>,
    {
        let n = self.len();
        let start = match range.start_bound() {
            core::ops::Bound::Included(&l) => l,
            core::ops::Bound::Excluded(&l) => l + 1,
            core::ops::Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            core::ops::Bound::Included(&r) => r + 1,
            core::ops::Bound::Excluded(&r) => r,
            core::ops::Bound::Unbounded => n,
        };
        if start > end || end > n {
            None
        } else {
            self._query(start, end, 0, 0, n)
        }
    }

    /// O(log(N))
    /// N: count of leaves
    fn _query(&self, s: usize, e: usize, i: usize, l: usize, r: usize) -> Option<T> {
        if r <= s || e <= l {
            None
        } else if s <= l && r <= e {
            Some(self.tree[i])
        } else {
            let a = self._query(s, e, i * 2 + 1, l, (l + r) / 2);
            let b = self._query(s, e, i * 2 + 2, (l + r) / 2, r);
            match (a, b) {
                (None, None) => None,
                (None, Some(b)) => Some(b),
                (Some(a), None) => Some(a),
                (Some(a), Some(b)) => Some(a.op(b)),
            }
        }
    }
}

#[codesnip::entry("SegmentedTree")]
impl<T: Monoid + Copy> core::ops::Index<usize> for SegmentedTree<T> {
    type Output = T;
    /// O(1)
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.len());
        let index = index + self.len() - 1;
        &self.tree[index]
    }
}

#[codesnip::entry("SegmentedTree")]
impl<T: Monoid + Copy, F: Into<T>> From<Vec<F>> for SegmentedTree<T> {
    /// O(N)
    /// N: v length
    fn from(v: Vec<F>) -> Self {
        Self::init(v.len(), v)
    }
}

#[codesnip::entry("SegmentedTree")]
impl<T: Monoid + Copy, F: Into<T>> core::iter::FromIterator<F> for SegmentedTree<T> {
    /// O(N)
    /// N: iter length
    fn from_iter<S>(iter: S) -> Self
    where
        S: IntoIterator<Item = F>,
    {
        Self::from(iter.into_iter().collect::<Vec<_>>())
    }
}

#[codesnip::entry(include("SegmentedTree", "AddMonoid"))]
pub type AddSegTree<T> = SegmentedTree<AddMonoid<T>>;
#[codesnip::entry(include("SegmentedTree", "MulMonoid"))]
pub type MulSegTree<T> = SegmentedTree<MulMonoid<T>>;
#[codesnip::entry(include("SegmentedTree", "MaxMonoid"))]
pub type MaxSegTree<T> = SegmentedTree<MaxMonoid<T>>;
#[codesnip::entry(include("SegmentedTree", "MinMonoid"))]
pub type MinSegTree<T> = SegmentedTree<MinMonoid<T>>;
#[codesnip::entry(include("SegmentedTree", "XorMonoid"))]
pub type XorSegTree<T> = SegmentedTree<XorMonoid<T>>;

#[test]
fn add_seg_tree() {
    let st = AddSegTree::from(vec![1, 2, 3, 4, 5, 6, 7]);
    assert_eq!(st.query(..), Some(28.into()));
    assert_eq!(st.query(2..4), Some(7.into()));
    assert_eq!(st.query(3..=4), Some(9.into()));
    assert_eq!(st.query(5..), Some(13.into()));

    assert_eq!(st.query(..1), Some(1.into()));
    assert_eq!(st.query(..2), Some(3.into()));
    assert_eq!(st.query(..3), Some(6.into()));
    assert_eq!(st.query(..4), Some(10.into()));
    assert_eq!(st.query(..5), Some(15.into()));
    assert_eq!(st.query(..6), Some(21.into()));
    assert_eq!(st.query(..7), Some(28.into()));
}

#[test]
fn mul_seg_tree() {
    let st = MulSegTree::from(vec![1, 2, 3, 4, 5, 6, 7]);
    assert_eq!(st.query(..), Some(5040.into()));
    assert_eq!(st.query(2..4), Some(12.into()));
    assert_eq!(st.query(3..=4), Some(20.into()));
    assert_eq!(st.query(5..), Some(42.into()));

    assert_eq!(st.query(..1), Some(1.into()));
    assert_eq!(st.query(..2), Some(2.into()));
    assert_eq!(st.query(..3), Some(6.into()));
    assert_eq!(st.query(..4), Some(24.into()));
    assert_eq!(st.query(..5), Some(120.into()));
    assert_eq!(st.query(..6), Some(720.into()));
    assert_eq!(st.query(..7), Some(5040.into()));
}

#[test]
fn max_seg_tree() {
    let st = MaxSegTree::from(vec![1, 2, 3, 4, 5, 6, 7]);
    assert_eq!(st.query(..), Some(7.into()));
    assert_eq!(st.query(2..4), Some(4.into()));
    assert_eq!(st.query(3..=4), Some(5.into()));
    assert_eq!(st.query(5..), Some(7.into()));

    assert_eq!(st.query(..1), Some(1.into()));
    assert_eq!(st.query(..2), Some(2.into()));
    assert_eq!(st.query(..3), Some(3.into()));
    assert_eq!(st.query(..4), Some(4.into()));
    assert_eq!(st.query(..5), Some(5.into()));
    assert_eq!(st.query(..6), Some(6.into()));
    assert_eq!(st.query(..7), Some(7.into()));
}

#[test]
fn min_seg_tree() {
    let st = MinSegTree::from(vec![1, 2, 3, 4, 5, 6, 7]);
    assert_eq!(st.query(..), Some(1.into()));
    assert_eq!(st.query(2..4), Some(3.into()));
    assert_eq!(st.query(3..=4), Some(4.into()));
    assert_eq!(st.query(5..), Some(6.into()));

    assert_eq!(st.query(..1), Some(1.into()));
    assert_eq!(st.query(..2), Some(1.into()));
    assert_eq!(st.query(..3), Some(1.into()));
    assert_eq!(st.query(..4), Some(1.into()));
    assert_eq!(st.query(..5), Some(1.into()));
    assert_eq!(st.query(..6), Some(1.into()));
    assert_eq!(st.query(..7), Some(1.into()));
}

#[test]
fn xor_seg_tree() {
    // [001, 010, 011, 100, 101, 110, 111]
    let st = XorSegTree::from(vec![1, 2, 3, 4, 5, 6, 7]);
    assert_eq!(st.query(..), Some(0.into()));
    assert_eq!(st.query(2..4), Some(7.into()));
    assert_eq!(st.query(3..=4), Some(1.into()));
    assert_eq!(st.query(5..), Some(1.into()));

    assert_eq!(st.query(..1), Some(1.into()));
    assert_eq!(st.query(..2), Some(3.into()));
    assert_eq!(st.query(..3), Some(0.into()));
    assert_eq!(st.query(..4), Some(4.into()));
    assert_eq!(st.query(..5), Some(1.into()));
    assert_eq!(st.query(..6), Some(7.into()));
    assert_eq!(st.query(..7), Some(0.into()));
}
