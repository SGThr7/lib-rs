use crate::math::{
    math_structs::Semigroup,
    num::{BoundedAbove, BoundedBelow, One, Zero},
};

/// [Monoid](https://en.wikipedia.org/wiki/Monoid) is an semigroup with identity element.
///
/// # Identity element
///
/// ~~~text
/// ∃ e ∈ Set, ∀ a ∈ Set, e ◦ a = a ◦ e = a
/// ~~~
#[codesnip::entry("Monoid", include("Semigroup"))]
pub trait Monoid: Semigroup {
    fn identity() -> Self::Set;
}

#[codesnip::entry(inline, "AddMonoid", include("define_monoid", "Zero"))]
mod monoid_add {
    #[codesnip::skip]
    use super::*;

    pub use monoid_add_impl::AddMonoid;
    mod monoid_add_impl {
    use super::{define_monoid, Zero};
    use core::ops::Add;

    define_monoid!(AddMonoid<T: Add<Output = T>, Zero>, |lhs,rhs| lhs+rhs, T::zero());
}
}
pub use monoid_add::*;

#[codesnip::entry(inline, "MulMonoid", include("define_monoid", "One"))]
mod monoid_mul {
    #[codesnip::skip]
    use super::*;

    pub use monoid_mul_impl::MulMonoid;
    mod monoid_mul_impl {
    use super::{define_monoid, One};
    use core::ops::Mul;

    define_monoid!(MulMonoid<T: Mul<Output = T>, One>, |lhs,rhs| lhs*rhs, T::one());
}
}
pub use monoid_mul::*;

#[codesnip::entry(inline, "MaxMonoid", include("define_monoid", "BoundedBelow"))]
mod monoid_max {
    #[codesnip::skip]
    use super::*;

    pub use monoid_max_impl::MaxMonoid;
    mod monoid_max_impl {
    use super::{define_monoid, BoundedBelow};

    define_monoid!(MaxMonoid<T: Ord, BoundedBelow>, |lhs,rhs| T::max(lhs,rhs), T::lower_bound());
}
}
pub use monoid_max::*;

#[codesnip::entry(inline, "MinMonoid", include("define_monoid", "BoundedAbove"))]
mod monoid_min {
    #[codesnip::skip]
    use super::*;

    pub use monoid_min_impl::MinMonoid;
    mod monoid_min_impl {
    use super::{define_monoid, BoundedAbove};

    define_monoid!(MinMonoid<T: Ord, BoundedAbove>, |lhs,rhs| T::min(lhs,rhs), T::upper_bound());
}
}
pub use monoid_min::*;

#[codesnip::entry(inline, "BitXorMonoid", include("define_monoid", "Zero"))]
mod monoid_bitxor {
    #[codesnip::skip]
    use super::*;

    pub use monoid_bitxor_impl::BitXorMonoid;
    mod monoid_bitxor_impl {
    use super::{define_monoid, Zero};
    use core::ops::BitXor;

    define_monoid!(BitXorMonoid<T: BitXor<Output = T>, Zero>, |lhs,rhs| lhs^rhs, T::zero());
}
}
pub use monoid_bitxor::*;

#[codesnip::entry(inline, "BitOrMonoid", include("define_monoid", "Zero"))]
mod monoid_bitor {
    #[codesnip::skip]
    use super::*;

    pub use monoid_bitor_impl::BitOrMonoid;
    mod monoid_bitor_impl {
    use super::{define_monoid, Zero};
    use core::ops::BitOr;

    define_monoid!(BitOrMonoid<T: BitOr<Output = T>, Zero>, |lhs,rhs| lhs|rhs, T::zero());
}
}
pub use monoid_bitor::*;

#[codesnip::entry]
macro_rules! define_monoid {
    (@impl $monoid:ident<T:$($bounds:path),*>, |$lhs:ident, $rhs:ident| $expr:expr, $identity:expr) => {
        impl<T: Clone + $($bounds+)*> Semigroup for $monoid<T> {
            type Set = T;
            fn operate(lhs: &Self::Set, rhs: &Self::Set) -> Self::Set {
                let $lhs = lhs.clone();
                let $rhs = rhs.clone();
                $expr
            }
        }
        impl<T: Clone + $($bounds+)*> Monoid for $monoid<T> {
            fn identity() -> Self::Set { $identity }
        }
    };
    ($monoid:ident<T: $($bounds:path),* $(,)?>, |$lhs:ident, $rhs:ident| $expr:expr, $identity:expr) => {
        use super::{Monoid, Semigroup};
        use core::marker::PhantomData;

        pub struct $monoid<T>(PhantomData<T>);
        define_monoid!(@impl $monoid<T:$($bounds),*>, |$lhs,$rhs| $expr, $identity);
    };
}
#[codesnip::entry("define_monoid")]
pub(super) use define_monoid;
