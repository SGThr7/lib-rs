#[codesnip::entry("GCD_LCM")]
pub trait CommonNum<Rhs = Self> {
    type Output;
    fn gcd(self, other: Rhs) -> Self::Output;
    fn lcm(self, other: Rhs) -> Self::Output;
}

#[codesnip::entry("GCD_LCM")]
mod common_num_impl {
    use super::CommonNum;

    macro_rules! impl_common_num {
        (@forward_ref $t:ty) => {
            impl CommonNum<&$t> for $t {
                type Output = <$t as CommonNum>::Output;
                fn gcd(self, other: &$t) -> Self::Output { self.gcd(*other) }
                fn lcm(self, other: &$t) -> Self::Output { self.lcm(*other) }
            }
            impl CommonNum<$t> for &$t {
                type Output = <$t as CommonNum>::Output;
                fn gcd(self, other: $t) -> Self::Output { self.clone().gcd(other) }
                fn lcm(self, other: $t) -> Self::Output { self.clone().lcm(other) }
            }
            impl CommonNum<&$t> for &$t {
                type Output = <$t as CommonNum>::Output;
                fn gcd(self, other: &$t) -> Self::Output { self.clone().gcd(other.clone()) }
                fn lcm(self, other: &$t) -> Self::Output { self.clone().lcm(other.clone()) }
            }
        };
        ($zero:expr, for $($t:ty)*) => {$(
            impl CommonNum for $t {
                type Output = $t;

                fn gcd(self, other: Self) -> Self {
                    let (mut a, mut b) = if self >= other {
                        (self, other)
                    } else {
                        (other, self)
                    };
                    while b != $zero {
                        let r = a.rem_euclid(b);
                        a = b;
                        b = r;
                    }
                    a
                }

                fn lcm(self, other: Self) -> Self {
                    let gcd = self.gcd(other);
                    self / gcd * other
                }
            }
            impl_common_num! { @forward_ref $t }
        )*};
    }

    impl_common_num!(0, for i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);
    impl_common_num!(0., for f32 f64);

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn gcd_test() {
            assert_eq!(3, 57.gcd(3));
            assert_eq!(3, 3.gcd(57));
            assert_eq!(57, 57.gcd(0));
            assert_eq!(57, 0.gcd(57));
            let a = 4;
            let b = 6;
            let c = 2;
            assert_eq!(c, a.gcd(b));
            assert_eq!(c, (-a).gcd(b));
            assert_eq!(c, a.gcd(-b));
            assert_eq!(c, (-a).gcd(-b));
        }

        #[test]
        fn lcm_test() {
            assert_eq!(57, 57.lcm(3));
            assert_eq!(57, 3.lcm(57));
            assert_eq!(0, 57.lcm(0));
            assert_eq!(0, 0.lcm(57));
            let a = 4;
            let b = 6;
            let c = 12;
            assert_eq!(c, a.lcm(b));
            assert_eq!(-c, (-a).lcm(b));
            assert_eq!(-c, a.lcm(-b));
            assert_eq!(c, (-a).lcm(-b));
        }
    }
}
