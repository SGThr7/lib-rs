// #[codesnip::entry(include("ActMonoid", "Semigroup"))]
// pub struct LazySegmentTree<AM: ActMonoid> {
//     n: usize,
//     depth: usize,
//     tree: Vec<<AM::Monoid as Semigroup>::Set>,
//     lazy: Vec<AM::Act>,
// }

// #[codesnip::entry("LazySegmentTree", include("ActMonoid", "Semigroup"))]
// impl<AM: ActMonoid> LazySegmentTree<AM> {
//     pub fn new(size: usize) -> Self {
//         Self::init(size, &vec![])
//     }

//     fn init(n: usize, s: &[<AM::Monoid as Semigroup>::Set]) -> Self {
//         let depth = (32 - (n.saturating_sub(1) as u32).leading_zeros()) as usize;
//         let n = 1 << depth;
//         let size = n * 2 - 1;
//         let mut tree = vec![AM::identity(); size];
//         tree[n - 1..n - 1 + s.len()].clone_from_slice(s);
//         let lazy = vec![AM::identity_act(); n - 1];

//         for i in (0..n - 1).rev() {
//             let lhs = &tree[i * 2 + 1];
//             let rhs = &tree[i * 2 + 2];
//             tree[i] = AM::operate(lhs, rhs);
//         }

//         Self {
//             n,
//             depth,
//             tree,
//             lazy,
//         }
//     }

//     pub fn len(&self) -> usize {
//         self.n
//     }

//     pub fn depth(&self) -> usize {
//         self.depth
//     }

//     fn eval(&mut self, i: usize) {
//         if self.is_leaves(i) {
//             return;
//         }
//         if let Some((li, ri)) = self.get_children_index(i) {
//             if self.is_leaves(li) && self.is_leaves(ri) {
//                 self.tree[li] = AM::act(&self.tree[li], &self.lazy[i]);
//                 self.tree[ri] = AM::act(&self.tree[ri], &self.lazy[i]);
//             } else {
//                 self.lazy[li] = AM::merge_act(&self.lazy[li], &self.lazy[i]);
//                 self.lazy[ri] = AM::merge_act(&self.lazy[ri], &self.lazy[i]);
//             }
//         }
//         self.tree[i] = AM::act(&self.tree[i], &self.lazy[i]);
//         self.lazy[i] = AM::identity_act();
//     }

//     pub fn get(&mut self, index: usize) -> Option<&<AM::Monoid as Semigroup>::Set>
//     where
//         <AM::Monoid as Semigroup>::Set: core::fmt::Debug,
//         AM::Act: core::fmt::Debug,
//     {
//         if self.len() <= index {
//             return None;
//         }

//         let mut l = 0;
//         let mut r = self.len();
//         let mut i = 0;
//         while r - l != 1 {
//             self.eval(i);
//             let mid = (r - l) / 2 + l;
//             if l <= index && index < mid {
//                 r = mid;
//                 i = i * 2 + 1;
//             } else if mid <= index && index < r {
//                 l = mid;
//                 i = i * 2 + 2;
//             }
//         }
//         assert_eq!(index, l);
//         self.eval(i);
//         Some(&self.tree[i])
//     }

//     pub fn query<R>(&mut self, range: R) -> <AM::Monoid as Semigroup>::Set
//     where
//         R: core::ops::RangeBounds<usize>,
//         <AM::Monoid as Semigroup>::Set: core::fmt::Debug,
//         AM::Act: core::fmt::Debug,
//     {
//         use core::ops::Bound::{Excluded, Included, Unbounded};
//         let s = match range.start_bound() {
//             Included(&l) => l,
//             Excluded(&l) => l + 1,
//             Unbounded => 0,
//         };
//         let e = match range.end_bound() {
//             Included(&r) => r + 1,
//             Excluded(&r) => r,
//             Unbounded => self.len(),
//         };
//         assert!(e <= self.len());

//         self._query(s, e, 0, 0, self.len())
//     }

//     fn _query(
//         &mut self,
//         s: usize,
//         e: usize,
//         i: usize,
//         l: usize,
//         r: usize,
//     ) -> <AM::Monoid as Semigroup>::Set
//     where
//         <AM::Monoid as Semigroup>::Set: core::fmt::Debug,
//         AM::Act: core::fmt::Debug,
//     {
//         self.eval(i);
//         if s <= l && r <= e {
//             self.tree[i].clone()
//         } else if s < r && l < e {
//             let left = self._query(s, e, i * 2 + 1, l, (l + r) / 2);
//             let right = self._query(s, e, i * 2 + 2, (l + r) / 2, r);
//             AM::operate(&left, &right)
//         } else {
//             AM::identity()
//         }
//     }

//     pub fn range_apply<R>(&mut self, range: R, value: AM::Act)
//     where
//         R: core::ops::RangeBounds<usize>,
//         <AM::Monoid as Semigroup>::Set: core::fmt::Debug,
//         AM::Act: core::fmt::Debug,
//     {
//         use core::ops::Bound::{Excluded, Included, Unbounded};
//         let s = match range.start_bound() {
//             Included(&l) => l,
//             Excluded(&l) => l + 1,
//             Unbounded => 0,
//         };
//         let e = match range.end_bound() {
//             Included(&r) => r + 1,
//             Excluded(&r) => r,
//             Unbounded => self.n,
//         };
//         assert!(e <= self.len());
//         assert!(s <= e);

//         self._range_apply(s, e, value, 0, 0, self.len())
//     }

//     fn _range_apply(&mut self, s: usize, e: usize, v: AM::Act, i: usize, l: usize, r: usize)
//     where
//         <AM::Monoid as Semigroup>::Set: core::fmt::Debug,
//         AM::Act: core::fmt::Debug,
//     {
//         println!("{}, {}, {:?}, {}, {}, {}", s, e, &v, i, l, r);
//         self.eval(i);
//         if s <= l && r <= e {
//             if self.is_leaves(i) {
//                 self.tree[i] = AM::act(&self.tree[i], &v);
//             } else {
//                 self.lazy[i] = AM::merge_act(&self.lazy[i], &v);
//                 self.eval(i);
//             }
//         } else if s < r && l < e {
//             if let Some((li, ri)) = self.get_children_index(i) {
//                 let mid = (r - l) / 2 + l;
//                 self._range_apply(s, e, v.clone(), li, l, mid);
//                 self._range_apply(s, e, v.clone(), ri, mid, r);
//                 self.tree[i] = AM::operate(&self.tree[li], &self.tree[ri]);
//             }
//         }
//     }

//     fn is_leaves(&self, i: usize) -> bool {
//         let n = self.len();
//         n - 1 <= i && i < self.tree.len()
//     }

//     fn get_children_index(&self, i: usize) -> Option<(usize, usize)> {
//         if self.is_leaves(i) {
//             None
//         } else {
//             Some((i * 2 + 1, i * 2 + 2))
//         }
//     }
// }

// #[codesnip::entry("LazySegmentTree", include("ActMonoid", "Semigroup"))]
// impl<AM: ActMonoid> From<Vec<<AM::Monoid as Semigroup>::Set>> for LazySegmentTree<AM> {
//     fn from(v: Vec<<AM::Monoid as Semigroup>::Set>) -> Self {
//         Self::init(v.len(), &v)
//     }
// }

// #[codesnip::entry("LazySegmentTree", include("ActMonoid", "Semigroup"))]
// impl<AM> core::fmt::Debug for LazySegmentTree<AM>
// where
//     AM: ActMonoid,
//     <AM::Monoid as Semigroup>::Set: core::fmt::Debug,
//     AM::Act: core::fmt::Debug,
// {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         use std::fmt::Write;

//         for i in 0..self.depth() {
//             let l = (1 << i) - 1;
//             let r = (1 << (i + 1)) - 1;
//             for k in l..r {
//                 f.write_fmt(format_args!("{:?}[{:?}]\t", self.tree[k], self.lazy[k]))?;
//             }
//             f.write_char('\n')?;
//         }
//         let n = self.len();
//         for i in (n - 1)..(n * 2 - 1) {
//             f.write_fmt(format_args!("{:?}\t", self.tree[i]))?;
//         }
//         Ok(())
//     }
// }

// #[codesnip::entry(include("LazySegmentTree", "ReplaceMax"))]
// pub type RepMaxLazySegmentTree<T> = LazySegmentTree<ReplaceMax<T>>;

// #[codesnip::entry(include("LazySegmentTree", "ReplaceMin"))]
// pub type RepMinLazySegmentTree<T> = LazySegmentTree<ReplaceMin<T>>;

#[cfg(test)]
mod test_lazy_segtree {
    // use super::LazySegmentTree;
    // use crate::math::num_structs::{ActMonoid, Semigroup};
    // use core::cmp::PartialEq;
    // use std::fmt::Debug;

    macro_rules! test_segtree {
        ($($name:ident, $monoid:ident)*) => {$(
            #[test]
            fn $name() {
                type Mono<T> = $monoid<T>;
                type Seg<T> = LazySegmentTree<Mono<T>>;
                let v = vec![2, 7, 1, 8, 2, 8, 1, 8, 2, 8, 4, 6];
                check_segtree(&v, &mut Seg::from(v.clone()));

                let len = 14;
                let mut ans = vec![Mono::identity(); len];
                let mut segtree = Seg::new(len);
                for i in 0..len {
                    for k in i..=len {
                        for m in i..k {
                            ans[m] = Mono::act(&ans[m], &Some(33));
                        }
                        segtree.range_apply(i..k, Some(33));
                        check_segtree(&ans, &mut segtree);
                    }
                }
            }
        )*};
    }

    test_segtree! {
        // replace_max, ReplaceMax
        // replace_min, ReplaceMin
    }

    // fn check_segtree<AM>(ans: &[<AM::Monoid as Semigroup>::Set], segtree: &mut LazySegmentTree<AM>)
    // where
    //     AM: ActMonoid,
    //     <AM::Monoid as Semigroup>::Set: Debug + PartialEq,
    //     <AM::Monoid as Semigroup>::Set: core::fmt::Debug,
    //     AM::Act: core::fmt::Debug,
    // {
    //     let n = ans.len();
    //     // get for each
    //     for i in 0..n {
    //         assert_eq!(Some(&ans[i]), segtree.get(i));
    //     }

    //     // query for each
    //     for i in 0..n {
    //         for k in i..=n {
    //             let ans = ans[i..k]
    //                 .iter()
    //                 .fold(AM::identity(), |a, b| AM::operate(&a, b));
    //             let r = segtree.query(i..k);
    //             println!("{}..{},{:?}", i, k, &segtree);
    //             assert_eq!(ans, r, "range: `{}..{}`", i, k);
    //         }
    //     }
    // }
}
