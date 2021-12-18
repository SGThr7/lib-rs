#[cfg_attr(nightly, codesnip::entry("GcdLcm"))]
pub mod gcd_lcm;
#[codesnip::entry("GcdLcm")]
pub use gcd_lcm::GcdLcm;

// #[codesnip::entry("Rational", include("GcdLcm"))]
#[cfg_attr(nightly, codesnip::entry("Rational", include("GcdLcm")))]
pub mod rational;

#[cfg_attr(nightly, codesnip::entry("Prime"))]
pub mod prime;

pub mod modint;
pub mod multiset;
pub mod num;
