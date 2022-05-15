use crate::{MathGroup, Monoid, PartialGroup, Semigroup};
use std::marker::PhantomData;

mod add;
pub use add::AddAlge;

mod mul;
pub use mul::MulAlge;

mod max;
pub use max::MaxAlge;

mod min;
pub use min::MinAlge;

mod bit_or;
pub use bit_or::BitOrAlge;

mod bit_and;
pub use bit_and::BitAndAlge;

mod bit_xor;
pub use bit_xor::BitXorAlge;
