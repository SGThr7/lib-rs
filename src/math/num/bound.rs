use super::identity::impl_identity;

#[codesnip::entry(include("impl_identity"))]
#[allow(unused_macros)]
macro_rules! impl_bound {
    (impl $imp:ident, $method:ident, $bound:tt; for $($t:tt)*) => {$(
        impl_identity! { impl $imp, $method, core::$t::$bound; for $t }
    )*};
}
#[codesnip::entry("impl_bound")]
#[allow(unused_imports)]
pub(crate) use impl_bound;

#[codesnip::entry(inline, "BoundedAbove", include("impl_bound", "impl_identity"))]
mod bounded_above {
    #[codesnip::skip]
    use super::{impl_bound, impl_identity};

    pub trait BoundedAbove {
        fn upper_bound() -> Self;
    }

    impl_bound! { impl BoundedAbove, upper_bound, MAX; for i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }
    impl_bound! { impl BoundedAbove, upper_bound, INFINITY; for f32 f64 }
}
pub use bounded_above::*;

#[codesnip::entry(inline, "BoundedBelow", include("impl_bound", "impl_identity"))]
mod bounded_below_impl {
    #[codesnip::skip]
    use super::{impl_bound, impl_identity};

    pub trait BoundedBelow {
        fn lower_bound() -> Self;
    }

    impl_bound! { impl BoundedBelow, lower_bound, MIN; for i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }
    impl_bound! { impl BoundedBelow, lower_bound, NEG_INFINITY; for f32 f64 }
}
pub use bounded_below_impl::*;

#[codesnip::entry(inline, "Bounded", include("BoundedAbove", "BoundedBelow"))]
mod bounded_impl {
    #[codesnip::skip]
    use super::{BoundedAbove, BoundedBelow};

    pub trait Bounded: BoundedAbove + BoundedBelow {}

    macro_rules! impl_bounded {
        (for $($t:ty)*) => {$(
            impl Bounded for $t {}
        )*};
    }

    impl_bounded! { for i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize f32 f64 }
}
pub use bounded_impl::*;
