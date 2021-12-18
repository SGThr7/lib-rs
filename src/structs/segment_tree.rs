use crate::math::num::Monoid;

#[cfg_attr(nightly, codesnip::entry("LazySegTree", include("Monoid")))]
mod lazy;
#[codesnip::entry("LazySegTree")]
pub use lazy::{LSTMonoid, LazySegTree};

#[cfg_attr(nightly, codesnip::entry("SegmentTree", include("Monoid")))]
mod segtree;
#[codesnip::entry("SegmentTree")]
pub use segtree::SegmentTree;

pub mod types;
