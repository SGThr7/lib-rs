use crate::math::num::alge_struct::monoid::{
    AddMonoid, BitOrMonoid, BitXorMonoid, MaxMonoid, MinMonoid, Monoid, MulMonoid,
};

mod lazy;

#[codesnip::entry("SegmentTree")]
pub use segment_tree_impl::SegmentTree;

#[codesnip::entry("AddSegmentTree", include("SegmentTree", "AddMonoid"))]
pub type AddSegmentTree<T> = SegmentTree<AddMonoid<T>>;
#[codesnip::entry("MulSegmentTree", include("SegmentTree", "MulMonoid"))]
pub type MulSegmentTree<T> = SegmentTree<MulMonoid<T>>;
#[codesnip::entry("MaxSegmentTree", include("SegmentTree", "MaxMonoid"))]
pub type MaxSegmentTree<T> = SegmentTree<MaxMonoid<T>>;
#[codesnip::entry("MinSegmentTree", include("SegmentTree", "MinMonoid"))]
pub type MinSegmentTree<T> = SegmentTree<MinMonoid<T>>;
#[codesnip::entry("BitXorSegmentTree", include("SegmentTree", "BitXorMonoid"))]
pub type BitXorSegmentTree<T> = SegmentTree<BitXorMonoid<T>>;
#[codesnip::entry("BitOrSegmentTree", include("SegmentTree", "BitOrMonoid"))]
pub type BitOrSegmentTree<T> = SegmentTree<BitOrMonoid<T>>;

#[codesnip::entry("SegmentTree", include("Monoid"))]
mod segment_tree_impl {
    use super::Monoid;

    pub struct SegmentTree<M: Monoid> {
        n: usize,
        tree: Vec<M::Set>,
    }

    impl<M: Monoid> SegmentTree<M> {
        pub fn new(n: usize) -> Self {
            Self::init(n, &vec![])
        }

        fn init(n: usize, s: &[M::Set]) -> Self {
            // let n = n.next_power_of_two();
            let mut tree = vec![M::id(); n * 2 - 1];
            tree[n - 1..n - 1 + s.len()].clone_from_slice(s);
            let mut res = Self { n, tree };
            for i in (0..n - 1).rev() {
                res.update(i);
            }
            res
        }

        fn update(&mut self, i: usize) {
            self.tree[i] = M::operate(&self.tree[i * 2 + 1], &self.tree[i * 2 + 2]);
        }

        pub fn set(&mut self, index: usize, value: M::Set) {
            assert!(index < self.n);
            let mut i = index + self.n - 1;
            self.tree[i] = value;
            while i > 0 {
                i = (i - 1) / 2;
                self.update(i);
            }
        }

        pub fn get<I>(&self, index: I) -> Option<&I::Output>
        where
            I: core::slice::SliceIndex<[M::Set]>,
        {
            self.tree[self.n - 1..].get(index)
        }

        pub fn query<R>(&self, range: R) -> M::Set
        where
            R: core::ops::RangeBounds<usize>,
        {
            let l = match range.start_bound() {
                core::ops::Bound::Included(&l) => l,
                core::ops::Bound::Excluded(&l) => l + 1,
                core::ops::Bound::Unbounded => 0,
            };
            let r = match range.end_bound() {
                core::ops::Bound::Included(&r) => r + 1,
                core::ops::Bound::Excluded(&r) => r,
                core::ops::Bound::Unbounded => self.n,
            };
            assert!(l <= r);
            assert!(r <= self.n);

            let mut l = l + self.n - 1;
            let mut r = r + self.n - 1;

            let is_odd = |x: usize| x & 1 == 0;
            let div2 = |x: &mut usize| *x >>= 1;

            let mut vl = M::id();
            let mut vr = M::id();
            while l < r {
                if is_odd(l) {
                    vl = M::operate(&vl, &self.tree[l]);
                }
                if is_odd(r) {
                    r -= 1;
                    vr = M::operate(&self.tree[r], &vr);
                }
                div2(&mut l);
                div2(&mut r);
            }
            M::operate(&vl, &vr)
        }
    }

    impl<M: Monoid> From<Vec<M::Set>> for SegmentTree<M> {
        fn from(v: Vec<M::Set>) -> Self {
            Self::init(v.len(), &v)
        }
    }

    impl<M, I> core::ops::Index<I> for SegmentTree<M>
    where
        M: Monoid,
        I: core::slice::SliceIndex<[M::Set]>,
    {
        type Output = <I as core::slice::SliceIndex<[M::Set]>>::Output;

        fn index(&self, index: I) -> &Self::Output {
            core::ops::Index::index(&self.tree[self.n - 1..], index)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::num::alge_struct::monoid::{
        AddMonoid, BitXorMonoid, MaxMonoid, MinMonoid, Monoid, MulMonoid,
    };
    use core::{cmp::PartialEq, fmt::Debug};

    macro_rules! test_segtree {
            ($($name:ident, $monoid:ident)*) => {$(
                #[test]
                fn $name() {
                    type Mono<T> = $monoid<T>;
                    type Seg<T> = SegmentTree<Mono<T>>;
                    let v = vec![2, 7, 1, 8, 2, 8, 1, 8, 2, 8, 4, 6];
                    let n = v.len();

                    // from test
                    let segtree: Seg<_> = v.clone().into();
                    check_segtree(&v, &segtree);

                    // set test
                    let mut segtree = Seg::new(n);
                    let mut ans = vec![Mono::id(); n];
                    for i in 0..n {
                        segtree.set(i, v[i]);
                        ans[i] = v[i];
                        check_segtree(&ans, &segtree);
                    }
                }
            )*};
        }

    test_segtree! {
        add, AddMonoid
        mul, MulMonoid
        max, MaxMonoid
        min, MinMonoid
        xor, BitXorMonoid
    }

    fn check_segtree<M>(ans: &[M::Set], segtree: &SegmentTree<M>)
    where
        M: Monoid,
        M::Set: Debug + PartialEq,
    {
        let n = ans.len();
        // get for each
        for i in 0..n {
            assert_eq!(ans[i], *(segtree.get(i)).unwrap());
        }

        // query for each range
        for i in 0..=n {
            for k in i..=n {
                assert_eq!(
                    ans[i..k].iter().fold(M::id(), |a, b| M::operate(&a, &b)),
                    segtree.query(i..k),
                    "range: `{}..{}`",
                    i,
                    k
                );
            }
        }
    }
}
