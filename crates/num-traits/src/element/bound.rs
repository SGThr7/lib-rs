use std::{f32, f64, i128, i16, i32, i64, i8, isize, u128, u16, u32, u64, u8, usize};

macro_rules! impl_bound {
    ($trait:ident, $fn:ident, $id:tt, $($t:tt)*) => {$(
        impl_id! { $trait, $fn, $t::$id, $t }
    )*};
}

pub trait BoundedAbove {
    fn upper_bound() -> Self;
}

impl_bound! { BoundedAbove, upper_bound, MAX, u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }
impl_bound! { BoundedAbove, upper_bound, INFINITY, f32 f64 }

pub trait BoundedBelow {
    fn lower_bound() -> Self;
}

impl_bound! { BoundedBelow, lower_bound, MIN, u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }
impl_bound! { BoundedBelow, lower_bound, NEG_INFINITY, f32 f64 }

pub trait Bounded: BoundedAbove + BoundedBelow {}

macro_rules! impl_bounded {
    ($($t:ty)*) => {$( impl Bounded for $t {} )*};
}

impl_bounded! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64 }
