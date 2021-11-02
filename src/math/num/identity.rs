#[codesnip::entry]
#[allow(unused_macros)]
macro_rules! impl_identity {
    (impl $imp:ident, $method:ident, $e:expr; for $($t:ty)*) => {$(
        impl $imp for $t {
            fn $method() -> Self { $e }
        }
    )*};
}
#[codesnip::entry("impl_identity")]
#[allow(unused_imports)]
pub(crate) use impl_identity;

#[codesnip::entry(inline, "Zero", include("impl_identity"))]
mod zero {
    #[codesnip::skip]
    use super::impl_identity;

    pub trait Zero {
        fn zero() -> Self;
    }

    impl_identity! { impl Zero, zero, 0; for i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }
    impl_identity! { impl Zero, zero, 0.0; for f32 f64 }
}
pub use zero::*;

#[codesnip::entry(inline, "One", include("impl_identity"))]
mod one {
    #[codesnip::skip]
    use super::impl_identity;

    pub trait One {
        fn one() -> Self;
    }

    impl_identity! { impl One, one, 1; for i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }
    impl_identity! { impl One, one, 1.0; for f32 f64 }
}
pub use one::*;
