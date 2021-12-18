use super::SegmentTree;
use crate::math::num::alge_struct::types::{
    AddMonoid, BitAndMonoid, BitOrMonoid, BitXorMonoid, MaxMonoid, MinMonoid, MulMonoid,
};

#[codesnip::entry("AddSegTree", include("SegmentTree", "AddMonoid"))]
pub type AddSegTree<T> = SegmentTree<AddMonoid<T>>;
#[codesnip::entry("MulSegTree", include("SegmentTree", "MulMonoid"))]
pub type MulSegTree<T> = SegmentTree<MulMonoid<T>>;
#[codesnip::entry("MaxSegTree", include("SegmentTree", "MaxMonoid"))]
pub type MaxSegTree<T> = SegmentTree<MaxMonoid<T>>;
#[codesnip::entry("MinSegTree", include("SegmentTree", "MinMonoid"))]
pub type MinSegTree<T> = SegmentTree<MinMonoid<T>>;
#[codesnip::entry("BitXorSegTree", include("SegmentTree", "BitXorMonoid"))]
pub type BitXorSegTree<T> = SegmentTree<BitXorMonoid<T>>;
#[codesnip::entry("BitOrSegTree", include("SegmentTree", "BitOrMonoid"))]
pub type BitOrSegTree<T> = SegmentTree<BitOrMonoid<T>>;
#[codesnip::entry("BitAndSegTree", include("SegmentTree", "BitAndMonoid"))]
pub type BitAndSegTree<T> = SegmentTree<BitAndMonoid<T>>;

// #[codesnip::entry]
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub struct LMRep<M>(core::marker::PhantomData<M>);

// #[codesnip::entry("LMRep", include("Monoid", "LSTMonoid"))]
// impl<M: Monoid> LSTMonoid<M> for LMRep<M> {
//     fn act(set: &M::Set, acter: &Option<M::Set>, range: usize) -> M::Set {
//         match acter {
//             Some(acter) => (0..range).fold(M::id(), |acc, _| M::operate(&acc, acter)),
//             None => set.clone(),
//         }
//     }

//     fn merge_act(lhs: &Option<M::Set>, rhs: &Option<M::Set>) -> Option<M::Set> {
//         match rhs {
//             Some(_) => rhs.clone(),
//             None => lhs.clone(),
//         }
//     }
// }

// #[codesnip::entry(include("LMRep", "AddMonoid"))]
// pub type RepSum<T> = LMRep<AddMonoid<T>>;
// #[codesnip::entry(include("LMRep", "MulMonoid"))]
// pub type RepProd<T> = LMRep<MulMonoid<T>>;
// #[codesnip::entry(include("LMRep", "MaxMonoid"))]
// pub type RepMax<T> = LMRep<MaxMonoid<T>>;
// #[codesnip::entry(include("LMRep", "MinMonoid"))]
// pub type RepMin<T> = LMRep<MinMonoid<T>>;
// #[codesnip::entry(include("LMRep", "BitXorMonoid"))]
// pub type RepXor<T> = LMRep<BitXorMonoid<T>>;
// #[codesnip::entry(include("LMRep", "BitOrMonoid"))]
// pub type RepOr<T> = LMRep<BitOrMonoid<T>>;

// #[codesnip::entry]
// pub struct LMAdd<M>(core::marker::PhantomData<M>);

// #[codesnip::entry("LMAdd", include("Monoid", "LSTMonoid"))]
// impl<M> LSTMonoid<M> for LMAdd<M>
// where
//     M: Monoid,
//     M::Set: core::ops::Add<Output = M::Set>,
// {
//     fn act(set: &M::Set, acter: &Option<M::Set>, range: usize) -> M::Set {
//         match acter {
//             Some(acter) => (0..range).fold(set.clone(), |acc, _| M::operate(&acc, acter)),
//             None => set.clone(),
//         }
//     }

//     fn merge_act(lhs: &Option<M::Set>, rhs: &Option<M::Set>) -> Option<M::Set> {
//         match (lhs, rhs) {
//             (Some(lhs), Some(rhs)) => Some(lhs.clone() + rhs.clone()),
//             (lhs, rhs) => lhs.clone().or(rhs.clone()),
//         }
//     }
// }

// #[codesnip::entry(include("LMAdd", "AddMonoid"))]
// pub type AddSum<T> = LMAdd<AddMonoid<T>>;
// #[codesnip::entry(include("LMAdd", "MulMonoid"))]
// pub type AddProd<T> = LMAdd<MulMonoid<T>>;
// #[codesnip::entry(include("LMAdd", "MaxMonoid"))]
// pub type AddMax<T> = LMAdd<MaxMonoid<T>>;
// #[codesnip::entry(include("LMAdd", "MinMonoid"))]
// pub type AddMin<T> = LMAdd<MinMonoid<T>>;
// #[codesnip::entry(include("LMAdd", "BitXorMonoid"))]
// pub type AddXor<T> = LMAdd<BitXorMonoid<T>>;
// #[codesnip::entry(include("LMAdd", "BitOrMonoid"))]
// pub type AddOr<T> = LMAdd<BitOrMonoid<T>>;

// #[cfg(test)]
// mod lst_tests {
//     use super::super::LazySegTree;
//     use super::*;
//     use core::fmt::Debug;

//     fn check_segtree<M, LM>(ans: &[M::Set], segtree: &mut LazySegTree<M, LM>)
//     where
//         M: Monoid,
//         M::Set: PartialEq + Debug,
//         LM: LSTMonoid<M>,
//     {
//         let n = ans.len();
//         // get for each
//         for i in 0..n {
//             assert_eq!(
//                 segtree.get(i).as_ref(),
//                 Some(&ans[i]),
//                 "i={}\n{:?}",
//                 i,
//                 segtree
//             );
//         }

//         // get for each range
//         for i in 0..n {
//             for k in i..=n {
//                 let ans = ans[i..k].iter().fold(M::id(), |a, b| M::operate(&a, b));
//                 let r = segtree.get(i..k);
//                 assert_eq!(r.unwrap(), ans, "range: `{}..{}`\n{:?}", i, k, segtree);
//             }
//         }
//     }

//     macro_rules! test_segtree {
//         ($($name:ident: $lst_monoid:ident, $monoid:ident);* $(;)?) => {$(
//             mod $name {
//                 use super::*;
//                 type Mono = $monoid<isize>;
//                 type LSTMono = $lst_monoid<Mono>;
//                 type Seg = LazySegTree<Mono, LSTMono>;

//                 #[test]
//                 fn from() {
//                     let v = vec![2, 7, 1, 8, 2, 8, 1, 8, 2, 8, 4, 6];
//                     let mut seg = Seg::from(v.clone());
//                     check_segtree(&v, &mut seg);
//                 }

//                 #[test]
//                 fn operate() {
//                     let len = 14;
//                     let mut ans = vec![Mono::id(); len];
//                     let mut segtree = Seg::new(len);
//                     for i in 0..len {
//                         for k in i..=len {
//                             let val = Some(((k - i) % 7) as isize);
//                             for m in i..k {
//                                 ans[m] = LSTMono::act(&ans[m], &val, 1);
//                             }
//                             segtree.operate(i..k, val);
//                             check_segtree(&ans, &mut segtree);
//                         }
//                     }
//                 }
//             }
//         )*};
//     }

//     test_segtree! {
//         replace_sum:  LMRep, AddMonoid;
//         replace_prod: LMRep, MulMonoid;
//         replace_max:  LMRep, MaxMonoid;
//         replace_min:  LMRep, MinMonoid;
//         replace_xor:  LMRep, BitXorMonoid;
//         replace_or:   LMRep, BitOrMonoid;
//         replace_and:  LMRep, BitAndMonoid;
//         add_sum:  LMAdd, AddMonoid;
//         add_prod: LMAdd, MulMonoid;
//         add_max:  LMAdd, MaxMonoid;
//         add_min:  LMAdd, MinMonoid;
//         add_xor:  LMAdd, BitXorMonoid;
//         add_or:   LMAdd, BitOrMonoid;
//         add_and:  LMAdd, BitAndMonoid;
//     }
// }
