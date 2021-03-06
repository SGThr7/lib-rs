use crate::math::num::{AllBitOne, BoundedAbove, BoundedBelow, Monoid, One, Semigroup, Zero};

#[cfg_attr(nightly, codesnip::entry(include("Monoid", "Semigroup")))]
#[macro_export]
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
            $crate::define_monoid! { @impl $monoid <$t$(: $($bounds),*)?>, |$lhs,$rhs| $operate, $id }
        }
    };
}

#[codesnip::entry("AddMonoid", include("define_monoid", "Zero"))]
define_monoid! {
    AddMonoid<T: Clone, Zero, Add<Output = T>>,
    |lhs,rhs| lhs.clone() + rhs.clone(),
    Zero::zero(),
    mod add_monoid_impl { use core::ops::Add; }
}

#[codesnip::entry("MulMonoid", include("define_monoid", "One"))]
define_monoid! {
    MulMonoid<T: Clone, One, Mul<Output = T>>,
    |lhs,rhs| lhs.clone() * rhs.clone(),
    One::one(),
    mod mul_monoid_impl { use core::ops::Mul; }
}

#[codesnip::entry("MaxMonoid", include("define_monoid", "BoundedBelow"))]
define_monoid! {
    MaxMonoid<T: Clone, Ord, BoundedBelow>,
    |lhs,rhs| Ord::max(lhs.clone(), rhs.clone()),
    BoundedBelow::lower_bound(),
    mod max_monoid_impl
}

#[codesnip::entry("MinMonoid", include("define_monoid", "BoundedAbove"))]
define_monoid! {
    MinMonoid<T: Clone, Ord, BoundedAbove>,
    |lhs,rhs| Ord::min(lhs.clone(), rhs.clone()),
    BoundedAbove::upper_bound(),
    mod min_monoid_impl
}

#[codesnip::entry("BitXorMonoid", include("define_monoid", "Zero"))]
define_monoid! {
    BitXorMonoid<T: Clone, Zero, BitXor<Output = T>>,
    |lhs,rhs| BitXor::bitxor(lhs.clone(), rhs.clone()),
    Zero::zero(),
    mod bitxor_monoid_impl { use core::ops::BitXor; }
}

#[codesnip::entry("BitOrMonoid", include("define_monoid", "Zero"))]
define_monoid! {
    BitOrMonoid<T: Clone, Zero, BitOr<Output = T>>,
    |lhs,rhs| BitOr::bitor(lhs.clone(), rhs.clone()),
    Zero::zero(),
    mod bitor_monoid_impl { use core::ops::BitOr; }
}

#[codesnip::entry("BitAndMonoid", include("define_monoid", "AllBitOne"))]
define_monoid! {
    BitAndMonoid<T: Clone, AllBitOne, BitAnd<Output = T>>,
    |lhs,rhs| BitAnd::bitand(lhs.clone(), rhs.clone()),
    AllBitOne::ALL_BIT_ONE,
    mod bitand_monoid_impl { use core::ops::BitAnd; }
}
