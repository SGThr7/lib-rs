#[cfg_attr(nightly, codesnip::entry(include("impl_identity")))]
#[macro_export]
macro_rules! impl_bound {
    (impl $imp:ident, $method:ident, $bound:tt; for $($t:tt)*) => {$(
        $crate::impl_identity! { impl $imp, $method, core::$t::$bound; for $t }
    )*};
}

pub use bounded_above::*;
#[codesnip::entry(inline, "BoundedAbove", include("impl_bound", "impl_identity"))]
mod bounded_above {
    pub trait BoundedAbove {
        fn upper_bound() -> Self;
    }

    impl_bound! { impl BoundedAbove, upper_bound, MAX; for i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }
    impl_bound! { impl BoundedAbove, upper_bound, INFINITY; for f32 f64 }
}

pub use bounded_below_impl::*;
#[codesnip::entry(inline, "BoundedBelow", include("impl_bound", "impl_identity"))]
mod bounded_below_impl {
    pub trait BoundedBelow {
        fn lower_bound() -> Self;
    }

    impl_bound! { impl BoundedBelow, lower_bound, MIN; for i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }
    impl_bound! { impl BoundedBelow, lower_bound, NEG_INFINITY; for f32 f64 }
}

pub use bounded_impl::*;
#[codesnip::entry(inline, "Bounded", include("BoundedAbove", "BoundedBelow"))]
mod bounded_impl {
    #[codesnip::skip]
    use super::{BoundedAbove, BoundedBelow};

    pub trait Bounded: BoundedAbove + BoundedBelow {}

    macro_rules! impl_bounded {
        ($($t:ty)*) => {$(
            impl Bounded for $t {}
        )*};
    }

    impl_bounded! { i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize f32 f64 }
}
