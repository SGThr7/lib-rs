#[codesnip::entry("AbsSub")]
pub use abs_sub_impl::AbsSub;

#[codesnip::entry("AbsSub")]
mod abs_sub_impl {
    pub trait AbsSub {
        fn abs_sub(&self, other: &Self) -> Self;
    }

    macro_rules! impl_abs_sub {
        ($($t:ty)*) => ($(
            impl AbsSub for $t {
                fn abs_sub(&self, other: &Self) -> Self {
                    if self > other {
                        self - other
                    } else {
                        other - self
                    }
                }
            }
        )*)
    }

    impl_abs_sub!(u8 u16 u32 u64 u128 usize);
}
