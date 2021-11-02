use super::{monoid::define_monoid, Monoid, Semigroup};
use crate::math::num::{One, Reciprocal, Zero};

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
pub trait Group: Monoid {
    fn inv(x: &Self::Set) -> Self::Set;
    fn inv_operate(lhs: &Self::Set, rhs: &Self::Set) -> Self::Set {
        Self::operate(lhs, &Self::inv(rhs))
    }
}

#[codesnip::entry(include("define_monoid", "Group"))]
#[allow(unused_macros)]
macro_rules! define_group {
    (@impl $group:ident <$t:tt $(: $($bounds:path),*)?>, |$oplhs:ident,$oprhs:ident| $operate:expr, $id:expr, |$x:ident| $inv:expr, $(|$ioplhs:ident,$ioprhs:ident| $inv_operate:expr)?) => {
        impl<$t$(: $($bounds+)*)?> Group for $group<$t> {
            fn inv($x: &Self::Set) -> Self::Set { $inv }
            $(fn inv_operate($ioplhs: &Self::Set, $ioprhs: &Self::Set) -> Self::Set { $inv_operate })?
        }
        define_monoid! { @impl $group <$t$(: $($bounds),*)?>, |$oplhs,$oprhs| $operate, $id }
    };
    ($group:ident <$t:tt$(: $($bounds:path),*)?>, |$oplhs:ident,$oprhs:ident| $operate:expr, $id:expr, |$ix:ident| $inv:expr, $(|$ioplhs:ident,$ioprhs:ident| $inv_operate:expr,)? mod $mod:ident $({$($items:item)+})? ) => {
        struct $group<$t>(core::marker::PhantomData<$t>);
        mod $mod {
            use super::*;
            $($($items)+)?
            define_group! { @impl $group <$t$(: $($bounds),*)?>, |$oplhs,$oprhs| $operate, $id, |$ix| $inv, $(|$ioplhs,$ioprhs| $inv_operate)? }
        }
    };
}
#[codesnip::entry("define_group")]
#[allow(unused_imports)]
pub(crate) use define_group;

#[codesnip::entry("AddGroup", include("define_group", "Zero"))]
define_group! { AddGroup<T: Clone, Zero, Add<Output = T>, Neg<Output = T>>, |lhs,rhs| lhs.clone() + rhs.clone(), Zero::zero(), |x| -x.clone(), mod add_group_impl { use core::ops::{Add, Neg}; } }

#[codesnip::entry("MulGroup", include("define_group", "One", "Reciprocal"))]
define_group! { MulGroup<T: Clone, One, Reciprocal<Output = T>, Mul<Output = T>, Div<Output = T>>, |lhs,rhs| lhs.clone() * rhs.clone(), One::one(), |x| Reciprocal::recip(x.clone()), |lhs,rhs| lhs.clone() / rhs.clone(), mod mul_group_impl { use core::ops::{Mul, Div}; } }

#[codesnip::entry("BitXorGroup", include("define_group", "Zero"))]
define_group! { BitXorGroup<T: Clone, Zero, BitXor<Output = T>>, |lhs,rhs| lhs.clone() ^ rhs.clone(), Zero::zero(), |x| x.clone(), mod bitxor_group_impl { use core::ops::BitXor; } }
