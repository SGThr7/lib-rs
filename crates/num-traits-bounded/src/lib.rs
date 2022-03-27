use num_traits_macro_impl_id::impl_id;
use std::{f32, f64, i128, i16, i32, i64, i8, isize, u128, u16, u32, u64, u8, usize};

/// Trait for [Bounded](https://en.wikipedia.org/wiki/Bounded_set).
///
/// See also [`BoundedAbove`] and [`BoundedBelow`].
///
/// # Examples
///
/// ```
/// use num_traits_bounded::*;
/// // for below 1.43.0
/// use std::u8;
///
/// fn test_bounded<T: Bounded + PartialOrd>(x: T) -> bool {
///     T::INFIMUM <= x && x <= T::SUPREMUM
/// }
///
/// (u8::MIN..=u8::MAX).for_each(|x| assert!(test_bounded(x)));
/// ```
pub trait Bounded: BoundedAbove + BoundedBelow {}

macro_rules! impl_bounded {
    ($($t:ty)*) => {$( impl Bounded for $t {} )*};
}

impl_bounded! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64 }

macro_rules! impl_bound {
    ($trait:ident, $fn:ident, $id:tt, $($t:tt)*) => {$(
        impl_id! { $trait, $fn, $t::$id, $t }
    )*};
}

/// Trait for supremum.
///
/// # Supremum
///
/// The [supremum](https://en.wikipedia.org/wiki/Infimum_and_supremum) is the least [upper bound](https://en.wikipedia.org/wiki/Upper_and_lower_bounds) of a set *T*.
///
/// ~~~text
/// ∀x ∈ T, U ⊃ T, ∃y ∈ U, ∀z ∈ T, x ≤ y ∧ y ≤ z
/// ~~~
///
/// # Examples
///
/// ```
/// use num_traits_bounded::BoundedAbove;
/// // for below 1.43.0
/// use std::u8;
///
/// (u8::MIN..=u8::MAX).for_each(|x| assert!(x <= u8::SUPREMUM));
/// ```
pub trait BoundedAbove {
    const SUPREMUM: Self;
}

impl_bound! { BoundedAbove, SUPREMUM, MAX, u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64 }

/// Trait for infimum.
///
/// # Infimum
///
/// The [infimum](https://en.wikipedia.org/wiki/Infimum_and_supremum) is the greatest [lower bound](https://en.wikipedia.org/wiki/Upper_and_lower_bounds) of a set *T*.
///
/// ~~~text
/// ∀x ∈ T, U ⊃ T, ∃y ∈ U, ∀z ∈ U, x ≥ y ∧ y ≥ z
/// ~~~
///
/// # Examples
///
/// ```
/// use num_traits_bounded::BoundedBelow;
/// // for below 1.43.0
/// use std::u8;
///
/// (u8::MIN..=u8::MAX).for_each(|x| assert!(u8::INFIMUM <= x));
/// ```
pub trait BoundedBelow {
    const INFIMUM: Self;
}

impl_bound! { BoundedBelow, INFIMUM, MIN, u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64 }
