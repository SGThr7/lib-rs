#[cfg_attr(nightly, codesnip::entry("OrdEx"))]
pub mod ord;
#[codesnip::entry("OrdEx")]
pub use ord::*;

#[cfg_attr(nightly, codesnip::entry("IteratorEx"))]
pub mod iterator;
#[codesnip::entry("IteratorEx")]
pub use iterator::*;
