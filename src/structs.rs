use crate::math::num::Monoid;
#[cfg_attr(nightly, codesnip::entry("BinaryIndexedTree", include("Monoid")))]
pub mod binary_indexed_tree;
#[codesnip::entry("BinaryIndexedTree")]
#[allow(unused_imports)]
use binary_indexed_tree::{BinaryIndexedTree, BIT};

pub mod segment_tree;
pub mod union_find;
