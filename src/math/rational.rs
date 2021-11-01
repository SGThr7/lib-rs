use crate::math::gcd_lcm::GcdLcm;

/// The rational number type.
#[codesnip::entry]
#[derive(Clone, Copy, Hash)]
pub struct Rational {
    minus: bool,
    numerator: rational_impl::RationalUnit,
    denominator: rational_impl::RationalUnit,
}

#[codesnip::entry("Rational", include("GcdLcm"))]
mod rational_impl {
    use super::{GcdLcm, Rational};
    use core::{
        cmp::Ordering::{self, Equal, Greater, Less},
        fmt::{Debug, Display},
        iter::{Product, Sum},
        ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    };

    pub(super) type RationalUnit = usize;

    impl Rational {
        /// Makes a new rational number.
        ///
        /// The return number is simplify fraction.
        ///
        /// # Example
        ///
        /// ```
        /// use lib_rust::math::rational::Rational;
        ///
        /// // It means `-1/19`.
        /// let num = Rational::new(true, 3, 57);
        ///
        /// // `Rational` can compare with tuple.
        /// assert_eq!(num, (-1, 19));
        /// assert_eq!(num, (1, -19));
        /// assert_eq!(num, (-3, 57));
        /// // Same as `num * 4`.
        /// assert_eq!(num, (-4, 76));
        /// ```
        pub fn new(minus: bool, numerator: RationalUnit, denominator: RationalUnit) -> Self {
            Self::new_raw(minus, numerator, denominator).simplify()
        }

        /// Makes a new rational number that not be simplify.
        fn new_raw(minus: bool, numerator: RationalUnit, denominator: RationalUnit) -> Self {
            assert_ne!(0, denominator, "Rational: denominator must not equal 0.");
            Self {
                minus,
                numerator,
                denominator,
            }
        }

        /// Makes a new `Rational` with value of one.
        ///
        /// # Example
        ///
        /// ```
        /// use lib_rust::math::rational::Rational;
        ///
        /// let one = Rational::one();
        ///
        /// assert_eq!(one, (1, 1));
        /// assert_eq!(one, (3, 3));
        /// ```
        pub fn one() -> Self {
            Self {
                minus: false,
                numerator: 1,
                denominator: 1,
            }
        }

        /// Makes a new `Rational` with value of zero.
        ///
        /// # Example
        ///
        /// ```
        /// use lib_rust::math::rational::Rational;
        ///
        /// let zero = Rational::zero();
        ///
        /// assert_eq!(zero, (0, 1));
        /// assert_eq!(zero, (0, 3));
        /// ```
        pub fn zero() -> Self {
            Self {
                minus: false,
                numerator: 0,
                denominator: 1,
            }
        }

        /// Reduce a fraction.
        fn simplify(self) -> Self {
            let gcd = self.numerator.gcd(self.denominator);
            Self {
                minus: self.minus,
                numerator: self.numerator / gcd,
                denominator: self.denominator / gcd,
            }
        }

        /// Multiplicative inverse a number.
        ///
        /// # Example
        ///
        /// ```
        /// use lib_rust::math::rational::Rational;
        ///
        /// let num = Rational::from((-4, 6));
        ///
        /// assert_eq!(num.reciprocal(), (3, -2));
        /// ```
        pub fn reciprocal(self) -> Self {
            Self::new(self.minus, self.denominator, self.numerator)
        }

        /// Raises self to the power of exp, using exponentiation by squaring.
        ///
        /// # Example
        ///
        /// ```
        /// use lib_rust::math::rational::Rational;
        ///
        /// let num = Rational::from((-4, 6));
        ///
        /// assert_eq!(num.pow(3), (-8, 27))
        /// ```
        pub fn pow(self, exp: u32) -> Self {
            Self::new(
                self.minus & (exp % 2 == 1),
                self.numerator.pow(exp),
                self.denominator.pow(exp),
            )
        }
    }

    impl Default for Rational {
        fn default() -> Self {
            Self::zero()
        }
    }

    impl Display for Rational {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let minus = if self.minus { "-" } else { "" };
            write!(f, "{}{} // {}", minus, self.numerator, self.denominator)
        }
    }

    impl Debug for Rational {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            <Self as Display>::fmt(&self, f)
        }
    }

    macro_rules! forward_ref_binop {
        ($trait:ident, $fn:ident for $t:ty, $u:ty) => {
            impl $trait<&$u> for $t {
                type Output = <$t as $trait<$u>>::Output;
                fn $fn(self, rhs: &$u) -> Self::Output {
                    $trait::$fn(self, *rhs)
                }
            }
            impl $trait<$u> for &$t {
                type Output = <$t as $trait<$u>>::Output;
                fn $fn(self, rhs: $u) -> Self::Output {
                    $trait::$fn(*self, rhs)
                }
            }
            impl $trait<&$u> for &$t {
                type Output = <$t as $trait<$u>>::Output;
                fn $fn(self, rhs: &$u) -> Self::Output {
                    $trait::$fn(*self, *rhs)
                }
            }
        };
    }

    macro_rules! impl_assignop {
        ($assign_trait:ident, $assign_fn:ident, $op_trait:ident, $op_fn:ident for $t:ty) => {
            impl $assign_trait<$t> for Rational {
                fn $assign_fn(&mut self, other: $t) {
                    *self = $op_trait::$op_fn(*self, other)
                }
            }
            impl $assign_trait<&$t> for Rational {
                fn $assign_fn(&mut self, other: &$t) {
                    *self = $op_trait::$op_fn(*self, other)
                }
            }
        };
    }

    impl Mul for Rational {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self::Output {
            let minus = self.minus ^ rhs.minus;
            let numerator = self.numerator * rhs.numerator;
            let denominator = self.denominator * rhs.denominator;
            Self::new(minus, numerator, denominator)
        }
    }
    forward_ref_binop! { Mul, mul for Rational, Rational }
    impl_assignop! { MulAssign, mul_assign, Mul, mul for Rational }

    impl Div for Rational {
        type Output = Self;

        fn div(self, rhs: Self) -> Self::Output {
            self * rhs.reciprocal()
        }
    }
    forward_ref_binop! { Div, div for Rational, Rational }
    impl_assignop! { DivAssign, div_assign, Div, div for Rational }

    impl Add for Rational {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            let denominator = self.denominator.lcm(rhs.denominator);
            match (self.minus, rhs.minus) {
                (true, true) | (false, false) => {
                    let numerator = self.numerator * (denominator / self.denominator)
                        + rhs.numerator * (denominator / rhs.denominator);
                    Self::new(self.minus, numerator, denominator)
                }
                (true, false) | (false, true) => {
                    let ln = self.numerator * (denominator / self.denominator);
                    let rn = rhs.numerator * (denominator / rhs.denominator);
                    let (minus, numerator) = if ln >= rn {
                        (self.minus, ln - rn)
                    } else {
                        (rhs.minus, rn - ln)
                    };
                    Self::new(minus, numerator, denominator)
                }
            }
        }
    }
    forward_ref_binop! { Add, add for Rational, Rational }
    impl_assignop! { AddAssign, add_assign, Add, add for Rational }

    impl Neg for Rational {
        type Output = Self;

        fn neg(self) -> Self::Output {
            Self::new(!self.minus, self.numerator, self.denominator)
        }
    }

    impl Sub for Rational {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            self + -rhs
        }
    }
    forward_ref_binop! { Sub, sub for Rational, Rational }
    impl_assignop! { SubAssign, sub_assign, Sub, sub for Rational }

    impl PartialEq for Rational {
        fn eq(&self, other: &Self) -> bool {
            matches!(self.cmp(other), Equal)
        }
    }

    impl Eq for Rational {}

    impl Ord for Rational {
        fn cmp(&self, other: &Self) -> Ordering {
            match (self.minus, other.minus) {
                (true, false) => Less,
                (false, true) => Greater,
                (lm, rm) if lm == rm => {
                    let t = self - other;
                    if t.numerator == 0 {
                        Equal
                    } else if t.minus {
                        Less
                    } else {
                        Greater
                    }
                }
                (_, _) => unreachable!(),
            }
        }
    }

    impl PartialOrd for Rational {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    macro_rules! impl_from_uint {
        (for $($t:ty)*) => {$(
            impl From<$t> for Rational {
                fn from(x: $t) -> Self {
                    Self::new(false, x as RationalUnit, 1)
                }
            }
            impl From<($t, $t)> for Rational {
                fn from(x: ($t, $t)) -> Self {
                    Self::new(false, x.0 as RationalUnit, x.1 as RationalUnit)
                }
            }
        )*};
    }

    macro_rules! impl_from_int {
        (for $($t:ty)*) => {$(
            impl From<$t> for Rational {
                fn from(x: $t) -> Self {
                    Self::new(x.is_negative(), x.abs() as RationalUnit, 1)
                }
            }
            impl From<($t, $t)> for Rational {
                fn from(x: ($t, $t)) -> Self {
                    Self::new(
                        x.0.is_negative() ^ x.1.is_negative(),
                        x.0.abs() as RationalUnit,
                        x.1.abs() as RationalUnit
                    )
                }
            }
        )*};
    }

    impl_from_uint! { for u8 u16 u32 u64 u128 usize }
    impl_from_int! { for i8 i16 i32 i64 i128 isize }

    macro_rules! impl_binop {
        ($trait:ident, $fn:ident, $assign_trait:ident, $assign_fn:ident for $($t:ty)*) => {$(
            impl $trait<$t> for Rational {
                type Output = <Rational as $trait>::Output;
                fn $fn(self, rhs: $t) -> Self::Output {
                    let rhs = Rational::from(rhs);
                    $trait::$fn(self, rhs)
                }
            }
            impl $trait<Rational> for $t {
                type Output = <Rational as $trait>::Output;
                fn $fn(self, rhs: Rational) -> Self::Output {
                    let lhs = Rational::from(self);
                    $trait::$fn(lhs, rhs)
                }
            }
            forward_ref_binop! { $trait, $fn for Rational, $t }
            forward_ref_binop! { $trait, $fn for $t, Rational }
            impl_assignop! { $assign_trait, $assign_fn, $trait, $fn for $t }
        )*};
    }

    impl_binop! { Add, add, AddAssign, add_assign for u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }
    impl_binop! { Sub, sub, SubAssign, sub_assign for u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }
    impl_binop! { Mul, mul, MulAssign, mul_assign for u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }
    impl_binop! { Div, div, DivAssign, div_assign for u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }

    macro_rules! impl_cmp {
        (@rec $t:ty) => {
            impl PartialEq<$t> for Rational {
                fn eq(&self, other: &$t) -> bool {
                    self.eq(&Rational::from(*other))
                }
            }
            impl PartialEq<Rational> for $t {
                fn eq(&self, other: &Rational) -> bool {
                    Rational::from(*self).eq(other)
                }
            }
            impl PartialOrd<$t> for Rational {
                fn partial_cmp(&self, other: &$t) -> Option<Ordering> {
                    self.partial_cmp(&Rational::from(*other))
                }
            }
            impl PartialOrd<Rational> for $t {
                fn partial_cmp(&self, other: &Rational) -> Option<Ordering> {
                    Rational::from(*self).partial_cmp(other)
                }
            }
        };
        (for $($t:ty)*) => {$(
            impl_cmp! { @rec $t }
            impl_cmp! { @rec ($t, $t) }
        )*};
    }

    impl_cmp! { for u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }

    impl Sum for Rational {
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            iter.fold(Self::zero(), |a, b| a + b)
        }
    }

    impl<'a> Sum<&'a Rational> for Rational {
        fn sum<I: Iterator<Item = &'a Rational>>(iter: I) -> Self {
            iter.fold(Self::zero(), |a, b| a + b)
        }
    }

    impl Product for Rational {
        fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
            iter.fold(Self::one(), |a, b| a * b)
        }
    }

    impl<'a> Product<&'a Rational> for Rational {
        fn product<I: Iterator<Item = &'a Rational>>(iter: I) -> Self {
            iter.fold(Self::one(), |a, b| a * b)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let a: Rational = (7, 6).into();
        let b: Rational = (7, 4).into();
        assert_eq!((35, 12), a + b);
    }

    #[test]
    fn sub() {
        let a: Rational = (7, 6).into();
        let b: Rational = (7, 4).into();
        assert_eq!((-7, 12), a - b);
    }

    #[test]
    fn mul() {
        let a: Rational = (7, 6).into();
        let b: Rational = (7, 4).into();
        assert_eq!((49, 24), a * b);
    }

    #[test]
    fn div() {
        let a: Rational = (7, 6).into();
        let b: Rational = (7, 4).into();
        assert_eq!((2, 3), a / b);
    }
}
