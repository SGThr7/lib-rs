#[cfg_attr(nightly, codesnip::entry("BTreeMultiSet"))]
mod btree;
#[codesnip::entry("BTreeMultiSet")]
#[allow(unused_imports)]
pub use btree::BTreeMultiSet;

#[cfg_attr(nightly, codesnip::entry("HashMultiSet"))]
mod hash;
#[codesnip::entry("HashMultiSet")]
pub use hash::HashMultiSet;
