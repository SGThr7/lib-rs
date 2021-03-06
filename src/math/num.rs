pub mod identity;
pub use identity::{One, Zero};

pub mod bound;
pub use bound::{Bounded, BoundedAbove, BoundedBelow};

pub mod element;
pub use element::Reciprocal;

pub mod alge_struct;
pub use alge_struct::{Group, Monoid, Semigroup};

#[cfg_attr(nightly, codesnip::entry("AllBitOne"))]
pub mod all_bit_one;
#[codesnip::entry("AllBitOne")]
pub use all_bit_one::AllBitOne;
