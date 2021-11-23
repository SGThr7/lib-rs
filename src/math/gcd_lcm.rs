#[codesnip::entry("GcdLcm")]
pub use gcd_lcm_impl::GcdLcm;

#[codesnip::entry("GcdLcm")]
mod gcd_lcm_impl {
    pub trait GcdLcm<Rhs = Self> {
        type Output;
        fn gcd(self, other: Rhs) -> Self::Output;
        fn lcm(self, other: Rhs) -> Self::Output;
        fn gcd_lcm(self, other: Rhs) -> (Self::Output, Self::Output)
        where
            Self: Sized + Clone,
            Rhs: Clone,
        {
            (self.clone().gcd(other.clone()), self.lcm(other))
        }
    }

    macro_rules! impl_gcd_lcm_int {
        (@forward_ref $t:ty) => {
            impl GcdLcm<&$t> for $t {
                type Output = <$t as GcdLcm>::Output;
                fn gcd(self, other: &$t) -> Self::Output { GcdLcm::gcd(self, *other) }
                fn lcm(self, other: &$t) -> Self::Output { GcdLcm::lcm(self, *other) }
                fn gcd_lcm(self, other: &$t) -> (Self::Output, Self::Output) { GcdLcm::gcd_lcm(self, *other) }
            }
            impl GcdLcm<$t> for &$t {
                type Output = <$t as GcdLcm>::Output;
                fn gcd(self, other: $t) -> Self::Output { GcdLcm::gcd(*self, other) }
                fn lcm(self, other: $t) -> Self::Output { GcdLcm::gcd(*self, other) }
                fn gcd_lcm(self, other: $t) -> (Self::Output, Self::Output) { GcdLcm::gcd_lcm(*self, other) }
            }
            impl GcdLcm<&$t> for &$t {
                type Output = <$t as GcdLcm>::Output;
                fn gcd(self, other: &$t) -> Self::Output { GcdLcm::gcd(*self, *other) }
                fn lcm(self, other: &$t) -> Self::Output { GcdLcm::gcd(*self, *other) }
                fn gcd_lcm(self, other: &$t) -> (Self::Output, Self::Output) { GcdLcm::gcd_lcm(*self, *other) }
            }
        };
        ($($t:ty)*) => {$(
            impl GcdLcm for $t {
                type Output = $t;

                fn gcd(self, other: $t) -> $t {
                    let (mut a, mut b) = if self >= other {
                        (self, other)
                    } else {
                        (other, self)
                    };
                    while b != 0 {
                        let r = a.rem_euclid(b);
                        a = b;
                        b = r;
                    }
                    a
                }

                fn lcm(self, other: $t) -> $t {
                    let gcd = self.gcd(other);
                    self / gcd * other
                }

                fn gcd_lcm(self, other: $t) -> ($t, $t){
                    let gcd = self.gcd(other);
                    let lcm = self / gcd * other;
                    (gcd, lcm)
                }
            }
            impl_gcd_lcm_int! { @forward_ref $t }
        )*};
    }

    impl_gcd_lcm_int! { i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }
}

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
