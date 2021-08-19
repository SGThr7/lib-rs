use crate::math::{
    group::{AddGroup, Group, MulGroup},
    monoid::Monoid,
};

#[codesnip::entry(include("Monoid"))]
pub struct BinaryIndexedTree<T: Monoid + Copy> {
    tree: Vec<T>,
}

#[codesnip::entry("BinaryIndexedTree", include("Monoid", "Group"))]
impl<T: Monoid + Copy> BinaryIndexedTree<T> {
    pub fn new(size: usize) -> Self {
        Self {
            tree: vec![T::e(); size],
        }
    }

    pub fn len(&self) -> usize {
        self.tree.len()
    }

    /// Least Significant Bit
    fn lsb(i: usize) -> usize {
        let i = i + 1;
        i & (!i + 1)
    }

    /// O(log(N))
    ///
    /// N: array length
    ///
    /// Operate to element.
    pub fn operate(&mut self, mut index: usize, rhs: T) {
        let len = self.len();
        while index < len {
            self.tree[index] = self.tree[index].op(rhs);
            index += Self::lsb(index);
        }
    }

    /// O(log(N))
    ///
    /// N: array length
    ///
    /// Get query of `0..right` (half-open range).
    pub fn query(&self, right: usize) -> T {
        let mut res = T::e();
        if right <= 0 {
            return res;
        }
        let mut i = right - 1;
        loop {
            res = res.op(self.tree[i]);
            let lsb = Self::lsb(i);
            if i < lsb {
                break;
            }
            i -= lsb;
        }
        res
    }

    pub fn range_query<R>(&self, range: R) -> T
    where
        T: Group,
        R: core::ops::RangeBounds<usize>,
    {
        let start = match range.start_bound() {
            std::ops::Bound::Included(&l) => l,
            std::ops::Bound::Excluded(&l) => l + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            std::ops::Bound::Included(&r) => r + 1,
            std::ops::Bound::Excluded(&r) => r,
            std::ops::Bound::Unbounded => self.len(),
        };
        self.query(end).op(self.query(start).inv())
    }

    pub fn lower_bound(&self, x: impl Into<T>) -> usize
    where
        T: Group + core::cmp::PartialOrd,
    {
        let mut x = x.into();
        let n = self.len();
        let mut i = 0;
        let mut len = n.next_power_of_two();
        while len > 0 {
            if i + len - 1 < n && self.tree[i + len - 1] <= x {
                x = x.op(self.tree[i + len - 1].inv());
                i += len - 1;
            }
            len >>= 1;
        }
        i
    }
}

#[codesnip::entry("BinaryIndexedTree", include("Monoid"))]
impl<T: Monoid + Copy, F: Into<T>> From<Vec<F>> for BinaryIndexedTree<T> {
    fn from(v: Vec<F>) -> Self {
        let mut res = Self::new(v.len());
        for (i, x) in v.into_iter().enumerate() {
            res.operate(i, x.into());
        }
        res
    }
}

#[codesnip::entry(include("BinaryIndexedTree", "AddGroup"))]
pub type AddBIT<T> = BinaryIndexedTree<AddGroup<T>>;

#[codesnip::entry(include("BinaryIndexedTree", "MulGroup"))]
pub type MulBIT<T> = BinaryIndexedTree<MulGroup<T>>;

#[test]
fn lsb() {
    use rand::{thread_rng, Rng};
    type Test = AddBIT<isize>;
    let mut rng = thread_rng();
    (0..10)
        .map(|_| rng.gen::<isize>())
        .for_each(|x| assert_eq!(Test::lsb((x - 1) as usize), (x & -x) as usize));
    assert_eq!(Test::lsb(0b01 - 1), 0b01);
    assert_eq!(Test::lsb(0b10 - 1), 0b10);
    assert_eq!(Test::lsb(0b11 - 1), 0b01);
}

#[test]
fn query() {
    let bit = AddBIT::from(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert_eq!(bit.query(0), 0.into());
    assert_eq!(bit.query(1), 1.into());
    assert_eq!(bit.query(2), 3.into());
    assert_eq!(bit.query(3), 6.into());
    assert_eq!(bit.query(4), 10.into());
    assert_eq!(bit.query(5), 15.into());
    assert_eq!(bit.query(6), 21.into());
    assert_eq!(bit.query(7), 28.into());
    assert_eq!(bit.query(8), 36.into());
    assert_eq!(bit.query(9), 45.into());

    assert_eq!(bit.range_query(2..5), 12.into());
    assert_eq!(bit.range_query(2..=5), 18.into());
    assert_eq!(bit.range_query(2..), 42.into());
    assert_eq!(bit.range_query(..3), 6.into());
}

#[test]
fn lower_bound() {
    let bit = AddBIT::from(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert_eq!(bit.lower_bound(19), 4);
    assert_eq!(bit.lower_bound(15), 4);
}
