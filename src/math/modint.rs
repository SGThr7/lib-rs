use crate::math::num::{One, Zero};

#[macro_use]
mod modulo;
pub use modulo::Modulo;

#[cfg_attr(nightly, codesnip::entry("ModInt", include("Modulo", "One", "Zero")))]
mod modint;
#[codesnip::entry("ModInt")]
#[allow(unused_imports)]
pub use modint::ModInt;

#[cfg_attr(nightly, codesnip::entry("ModIntFactMap", include("ModInt", "Modulo")))]
mod factmap;
#[codesnip::entry("ModIntFactMap")]
#[allow(unused_imports)]
pub use factmap::ModIntFactMap;

pub mod types;
