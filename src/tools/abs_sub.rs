#[codesnip::entry("AbsSub")]
pub trait AbsSub {
    fn abs_sub(&self, other: &Self) -> Self;
}

#[codesnip::entry("AbsSub")]
#[macro_export]
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

#[codesnip::entry("AbsSub")]
impl_abs_sub!(u8 u16 u32 u64 u128 usize);
