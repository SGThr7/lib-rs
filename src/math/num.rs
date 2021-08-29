#[codesnip::entry]
#[macro_export]
macro_rules! impl_identity {
    (impl $imp:ident, $method:ident, $e:expr; for $($t:ty)*) => {$(
        impl $imp for $t {
            fn $method() -> Self {
                $e
            }
        }

        impl<'a> $imp for &'a $t {
            fn $method() -> Self {
                &$e
            }
        }
    )*};
}

#[codesnip::entry]
pub trait Zero {
    fn zero() -> Self;
}

#[codesnip::entry("Zero", include("impl_identity"))]
impl_identity!(impl Zero, zero, 0; for i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);
#[codesnip::entry("Zero", include("impl_identity"))]
impl_identity!(impl Zero, zero, 0.0; for f32 f64);

#[codesnip::entry]
pub trait One {
    fn one() -> Self;
}

#[codesnip::entry("One", include("impl_identity"))]
impl_identity!(impl One, one, 1; for i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);
#[codesnip::entry("One", include("impl_identity"))]
impl_identity!(impl One, one, 1.0; for f32 f64);

#[codesnip::entry]
pub trait BoundedAbove {
    fn upper_bound() -> Self;
}

#[codesnip::entry(include("BoundedAbove", "impl_identity"))]
#[macro_export]
macro_rules! impl_upper_bound {
    ($($t:tt)*) => {$(impl_identity!{impl BoundedAbove, upper_bound, core::$t::MAX; for $t})*};
}

#[codesnip::entry("BoundedAbove", include("impl_upper_bound"))]
impl_upper_bound!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

#[codesnip::entry]
pub trait BoundedBelow {
    fn lower_bound() -> Self;
}

#[codesnip::entry(include("BoundedBelow", "impl_identity"))]
#[macro_export]
macro_rules! impl_lower_bound {
    ($($t:tt)*) => {$(impl_identity!{impl BoundedBelow, lower_bound, core::$t::MIN; for $t})*};
}

#[codesnip::entry("BoundedBelow", include("impl_lower_bound"))]
impl_lower_bound!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

#[codesnip::entry(include("BoundedAbove", "BoundedBelow"))]
pub trait Bounded: BoundedAbove + BoundedBelow {}
