use crate::math::{
    math_structs::{Monoid, Semigroup},
    num::{One, Reciprocal, Zero},
};

/// [Group (mathematics)](https://en.wikipedia.org/wiki/Group_(mathematics)) is an monoid with inverse element.
///
/// # Inverse element
///
/// When `e` is identity element,
///
/// ~~~text
/// ∀ a ∈ Set, ∃ b ∈ Set, a ◦ b = b ◦ a = e
/// ~~~
#[codesnip::entry(include("Monoid"))]
pub trait MathGroup: Monoid {
    fn inverse(x: &Self::Set) -> Self::Set;
    fn inv_operate(lhs: &Self::Set, rhs: &Self::Set) -> Self::Set {
        Self::operate(lhs, &Self::inverse(rhs))
    }
}

#[codesnip::entry(inline, "AddGroup", include("define_math_group", "Zero"))]
mod math_group_add {
    #[codesnip::skip]
    use super::*;

    pub use math_group_add_impl::AddGroup;
    mod math_group_add_impl {

        use super::{define_math_group, Zero};
        use core::ops::{Add, Neg};

        define_math_group!(AddGroup<T: Add<Output = T>, Zero, Neg<Output = T>>, |lhs,rhs| lhs+rhs, T::zero(), |x| -x);
    }
}
pub use math_group_add::*;

#[codesnip::entry(inline, "MulGroup", include("define_math_group", "One", "Reciprocal"))]
mod math_group_mul {
    #[codesnip::skip]
    use super::*;

    pub use math_group_mul_impl::MulGroup;
    mod math_group_mul_impl {
        use super::{define_math_group, One, Reciprocal};
        use core::ops::{Div, Mul};

        define_math_group!(MulGroup<T: Mul<Output = T>, One, Div<Output = T>, Reciprocal<Output = T>>, |lhs,rhs| lhs*rhs, T::one(), |x| x.reciprocal());
    }
}
pub use math_group_mul::*;

#[codesnip::entry(inline, "BitXorGroup", include("define_math_group", "Zero"))]
mod math_group_bitxor {
    #[codesnip::skip]
    use super::*;

    pub use math_group_bitxor_impl::BitXorGroup;
    mod math_group_bitxor_impl {
        use super::{define_math_group, Zero};
        use core::ops::BitXor;

        define_math_group!(BitXorGroup<T: BitXor<Output = T>, Zero>, |lhs,rhs| lhs^rhs, T::zero(), |x| x);
    }
}
pub use math_group_bitxor::*;

#[codesnip::entry(include("MathGroup", "Monoid", "Semigroup"))]
#[allow(unused_macros)]
macro_rules! define_math_group {
    (@impl $group:ident<T: $($bounds:path),* $(,)?>, |$lhs:ident,$rhs:ident| $expr:expr, $identity:expr, |$x:ident| $neg:expr) => {
        impl<T: Clone + $($bounds+)*> Semigroup for $group<T> {
            type Set = T;
            fn operate(lhs: &Self::Set, rhs: &Self::Set) -> Self::Set {
                let $lhs = lhs.clone();
                let $rhs = rhs.clone();
                $expr
            }
        }
        impl<T: Clone + $($bounds+)*> Monoid for $group<T> {
            fn identity() -> Self::Set { $identity }
        }
        impl<T: Clone + $($bounds+)*> MathGroup for $group<T> {
            fn inverse(x: &Self::Set) -> Self::Set {
                let $x = x.clone();
                $neg
            }
        }
    };
    ($($group:ident<T: $($bounds:path),* $(,)?>, |$lhs:ident,$rhs:ident| $expr:expr, $identity:expr, |$x:ident| $neg:expr);* $(;)?) => {$(
        use super::{MathGroup, Monoid, Semigroup};
        use core::marker::PhantomData;

        pub struct $group<T>(PhantomData<T>);
        define_math_group!(@impl $group<T: $($bounds),*>, |$lhs,$rhs| $expr, $identity, |$x| $neg);
    )*};
}
#[codesnip::entry("define_math_group")]
#[allow(unused_imports)]
pub(crate) use define_math_group;
