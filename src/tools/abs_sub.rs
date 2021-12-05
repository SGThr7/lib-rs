#[codesnip::entry("AbsSub")]
pub use abs_diff_impl::AbsDiff;

#[codesnip::entry("AbsSub")]
mod abs_diff_impl {
    pub trait AbsDiff<Rhs = Self> {
        type Output;
        fn abs_diff(self, other: Self) -> Self;
    }

    macro_rules! impl_abs_sub {
        ($($t:ty)*) => ($(
            impl AbsDiff for $t {
                type Output = Self;

                fn abs_diff(self, other: Self) -> Self {
                    if self >= other {
                        self - other
                    } else {
                        other - self
                    }
                }
            }
        )*)
    }

    impl_abs_sub! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abs_diff() {
        assert_eq!(10.abs_diff(20), 10);
        assert_eq!(100.abs_diff(20), 80);
    }
}
