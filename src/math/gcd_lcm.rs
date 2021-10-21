#[codesnip::entry("GCD_LCM")]
pub trait CommonNum {
    fn gcd(&self, other: &Self) -> Self;
    fn lcm(&self, other: &Self) -> Self;
}

#[codesnip::entry("GCD_LCM")]
mod common_num_impl {
    use super::CommonNum;

    macro_rules! impl_common_num {
        ($zero:expr, for $($t:ty)*) => {$(
            impl CommonNum for $t {
                fn gcd(&self, other: &Self) -> Self {
                    let (mut a, mut b) = if self >= other {
                        (*self, *other)
                    } else {
                        (*other, *self)
                    };
                    let mut r = a.rem_euclid(b);
                    while r > $zero {
                        a = b;
                        b = r;
                        r = a.rem_euclid(b);
                    }
                    a.div_euclid(b)
                }

                fn lcm(&self, other: &Self) -> Self {
                    let gcd = self.gcd(other);
                    self / gcd * other
                }
            }
        )*};
    }

    impl_common_num!(0, for i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);
    impl_common_num!(0., for f32 f64);
}
