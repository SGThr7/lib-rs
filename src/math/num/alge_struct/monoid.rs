use super::Semigroup;
use crate::math::num::{BoundedAbove, BoundedBelow, One, Zero};

/// [Monoid](https://en.wikipedia.org/wiki/Monoid) is an semigroup with identity element.
///
/// # Identity element
///
/// ~~~text
/// ∃ e ∈ Set, ∀ a ∈ Set, e ◦ a = a ◦ e = a
/// ~~~
#[codesnip::entry("Monoid", include("Semigroup"))]
pub trait Monoid: Semigroup {
    fn id() -> Self::Set;
}

#[codesnip::entry(include("Monoid", "Semigroup"))]
#[allow(unused_macros)]
macro_rules! define_monoid {
    (@impl $monoid:ident <$t:tt $(: $($bounds:path),*)?>, |$lhs:ident,$rhs:ident| $operate:expr, $id:expr) => {
        impl<$t$(: $($bounds+)*)?> Monoid for $monoid<$t> {
            fn id() -> Self::Set { $id }
        }
        impl<$t$(: $($bounds+)*)?> Semigroup for $monoid<$t> {
            type Set = $t;
            fn operate($lhs: &Self::Set, $rhs: &Self::Set) -> Self::Set { $operate }
        }
    };
    ($monoid:ident <$t:tt$(: $($bounds:path),*)?>, |$lhs:ident,$rhs:ident| $operate:expr, $id:expr, mod $mod:ident $({$($items:item)+})? ) => {
        pub struct $monoid<$t>(core::marker::PhantomData<$t>);
        mod $mod {
            use super::*;
            $($($items)+)?
            define_monoid! { @impl $monoid <$t$(: $($bounds),*)?>, |$lhs,$rhs| $operate, $id }
        }
    };
}
#[codesnip::entry("define_monoid")]
#[allow(unused_imports)]
pub(crate) use define_monoid;

#[codesnip::entry("AddMonoid", include("define_monoid", "Zero"))]
define_monoid! { AddMonoid<T: Clone, Zero, Add<Output = T>>, |lhs,rhs| lhs.clone() + rhs.clone(), Zero::zero(), mod add_monoid_impl { use core::ops::Add; } }

#[codesnip::entry("MulMonoid", include("define_monoid", "One"))]
define_monoid! { MulMonoid<T: Clone, One, Mul<Output = T>>, |lhs,rhs| lhs.clone() * rhs.clone(), One::one(), mod mul_monoid_impl { use core::ops::Mul; } }

#[codesnip::entry("MaxMonoid", include("define_monoid", "BoundedBelow"))]
define_monoid! { MaxMonoid<T: Clone, Ord, BoundedBelow>, |lhs,rhs| Ord::max(lhs.clone(), rhs.clone()), BoundedBelow::lower_bound(), mod max_monoid_impl }

#[codesnip::entry("MinMonoid", include("define_monoid", "BoundedAbove"))]
define_monoid! { MinMonoid<T: Clone, Ord, BoundedAbove>, |lhs,rhs| Ord::min(lhs.clone(), rhs.clone()), BoundedAbove::upper_bound(), mod min_monoid_impl }

#[codesnip::entry("BitXorMonoid", include("define_monoid", "Zero"))]
define_monoid! { BitXorMonoid<T: Clone, Zero, BitXor<Output = T>>, |lhs,rhs| BitXor::bitxor(lhs.clone(), rhs.clone()), Zero::zero(), mod bitxor_monoid_impl { use core::ops::BitXor; } }

#[codesnip::entry("BitOrMonoid", include("define_monoid", "Zero"))]
define_monoid! { BitOrMonoid<T: Clone, Zero, BitOr<Output = T>>, |lhs,rhs| BitOr::bitor(lhs.clone(), rhs.clone()), Zero::zero(), mod bitor_monoid_impl { use core::ops::BitOr; } }
