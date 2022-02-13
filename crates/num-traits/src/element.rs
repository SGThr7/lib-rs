macro_rules! impl_id {
    ($trait:ident, $fn:ident, $id:expr, $($t:ty)*) => {$(
        impl $trait for $t {
            #[inline]
            fn $fn() -> Self {
                $id
            }
        }
    )*};
}

mod all_bit_one;
mod bound;
mod one;
mod recip;
mod zero;

pub use all_bit_one::AllBitOne;
pub use bound::{Bounded, BoundedAbove, BoundedBelow};
pub use one::One;
pub use recip::Recip;
pub use zero::Zero;
