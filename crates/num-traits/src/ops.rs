use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

macro_rules! impl_binop {
    ($trait:ident, $fn:ident, $u:ty, -> $ret:ty, $($t:ty)*) => {
        pub trait $trait: Sized {
            fn $fn(self, rhs: $u) -> $ret;
        }

        $(
            impl $trait for $t {
                fn $fn(self, rhs: $u) -> $ret {
                    <$t>::$fn(self, rhs)
                }
            }
        )*
    };
    ($trait:ident, $fn:ident -> $ret:ty, $($t:ty)*) => {
        impl_binop! { $trait, $fn, Self, -> $ret, $($t)* }
    };
    ($trait:ident, $fn:ident, $u:ty, -> $ret:ty) => {
        impl_binop! { $trait, $fn, $u, -> $ret, u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }
    };
    ($trait:ident, $fn:ident -> $ret:ty) => {
        impl_binop! { $trait, $fn, Self, -> $ret }
    };
}

macro_rules! impl_unaryop {
    ($trait:ident, $fn:ident -> $ret:ty, $($t:ty)*) => {
        pub trait $trait: Sized {
            fn $fn(self) -> $ret;
        }

        $(impl $trait for $t {
            fn $fn(self) -> $ret {
                <$t>::$fn(self)
            }
        })*
    };
    ($trait:ident, $fn:ident -> $ret:ty) => {
        impl_unaryop! { $trait, $fn -> $ret, i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }
    };
}

macro_rules! impl_uint_abs {
    ($trait:ident, $fn:ident -> $ret:ty, |$self:ident| $expr:expr, $($t:ty)*) => {$(
        impl $trait for $t {
            fn $fn($self) -> $ret {
                $expr
            }
        }
    )*};
    ($trait:ident, $fn:ident -> $ret:ty, |$self:ident| $expr:expr) => {
        impl_uint_abs! { $trait, $fn -> $ret, |$self| $expr, u8 u16 u32 u64 u128 usize }
    };
}

pub mod checked;
pub mod overflowing;
pub mod saturating;
// pub mod unchecked;

pub use checked::CheckedOps;
pub use overflowing::OverflowingOps;
pub use saturating::SaturatingOps;

pub trait Ops:
    Sized
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + CheckedOps
    + OverflowingOps
    + SaturatingOps
{
}

impl_defs! { Ops }

pub trait AssignOps: Sized + AddAssign + SubAssign + MulAssign + DivAssign {}

impl_defs! { AssignOps }
