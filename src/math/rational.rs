use super::GcdLcm;
use core::cmp::Ordering::{self, Equal, Greater, Less};
use core::fmt;
use core::iter::{Product, Sum};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

type RationalUnit = usize;

/// The rational number type.
#[derive(Clone, Copy, Hash)]
pub struct Rational {
    minus: bool,
    numerator: RationalUnit,
    denominator: RationalUnit,
}

impl Rational {
    /// Returns zero with rational.
    ///
    /// # Examples
    ///
    /// ```
    /// use lib_rust::math::rational::Rational;
    ///
    /// assert_eq!(Rational::ZERO, 0);
    /// ```
    pub const ZERO: Self = Self {
        minus: false,
        numerator: 0,
        denominator: 1,
    };

    /// Returns one with rational.
    ///
    /// # Examples
    ///
    /// ```
    /// use lib_rust::math::rational::Rational;
    ///
    /// assert_eq!(Rational::ONE, 1);
    /// ```
    pub const ONE: Self = Self {
        minus: false,
        numerator: 1,
        denominator: 1,
    };

    /// Creates a rational without checking zero division.
    /// This results in undefined behaviour if the `denominator` is zero.
    ///
    /// # Safety
    ///
    /// The `denominator` must not be zero.
    pub unsafe fn new_unchecked(
        minus: bool,
        numerator: RationalUnit,
        denominator: RationalUnit,
    ) -> Self {
        Self {
            minus,
            numerator,
            denominator,
        }
        .simplify()
    }

    /// Makes a new rational number.
    ///
    /// The return number is simplify fraction.
    ///
    /// # Examples
    ///
    /// ```
    /// use lib_rust::math::rational::Rational;
    ///
    /// // It means `-1/19`.
    /// let num = Rational::new(true, 3, 57);
    /// let num2 = Rational::new(true, 1, 19);
    /// assert_eq!(num, num2);
    ///
    /// let num = num.unwrap();
    /// assert_eq!(num, -3.0/57.0);
    /// assert_eq!(num, -1.0/19.0);
    /// // Rational can compare with int tuple.
    /// assert_eq!(num, (3, -57));
    /// assert_eq!(num, (-1, 19));
    ///
    /// // The denominator cannot be zero.
    /// assert_eq!(Rational::new(false, 1, 0), None);
    /// ```
    pub fn new(minus: bool, numerator: RationalUnit, denominator: RationalUnit) -> Option<Self> {
        if denominator != 0 {
            Some(unsafe { Self::new_unchecked(minus, numerator, denominator) })
        } else {
            None
        }
    }

    /// Reduce a fraction.
    /// This function does *not* check zero denominator.
    fn simplify(self) -> Self {
        let gcd = self.numerator.gcd(self.denominator);
        Self {
            minus: self.minus,
            numerator: self.numerator / gcd,
            denominator: self.denominator / gcd,
        }
    }

    pub const fn is_negative(&self) -> bool {
        self.minus
    }

    pub const fn numerator(&self) -> RationalUnit {
        self.numerator
    }

    pub const fn denominator(&self) -> RationalUnit {
        self.denominator
    }

    /// Computes the absolute value of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use lib_rust::math::rational::Rational;
    ///
    /// let num: Rational = (-1, 3).into();
    ///
    /// assert_eq!(num.abs(), -num);
    /// ```
    pub const fn abs(self) -> Self {
        Self {
            minus: false,
            numerator: self.numerator,
            denominator: self.denominator,
        }
    }

    /// Takes the reciprocal (inverse) of a number.
    ///
    /// # Examples
    ///
    /// ```
    /// use lib_rust::math::rational::Rational;
    ///
    /// let num: Rational = (-2, 3).into();
    ///
    /// assert_eq!(num.recip(), -3.0 / 2.0);
    /// ```
    ///
    /// # Panics
    ///
    /// This function will panic if `self == 0`.
    pub fn recip(self) -> Self {
        if self.numerator == 0 {
            panic!("Cannot compute reciprocal of {:?}", self)
        }
        unsafe { Self::new_unchecked(self.minus, self.denominator, self.numerator) }
    }

    /// Raises self to the power of exp, using exponentiation by squaring.
    ///
    /// # Examples
    ///
    /// ```
    /// use lib_rust::math::rational::Rational;
    ///
    /// let num: Rational = (-2, 3).into();
    ///
    /// assert_eq!(num.pow(2), (4, 9));
    /// assert_eq!(num.pow(3), (-8, 27));
    /// ```
    pub fn pow(self, exp: u32) -> Self {
        unsafe {
            Self::new_unchecked(
                self.minus && (exp % 2 == 1),
                self.numerator.pow(exp),
                self.denominator.pow(exp),
            )
        }
    }
}

impl Default for Rational {
    fn default() -> Self {
        Self::ZERO
    }
}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let minus = if self.minus { "-" } else { "" };
        write!(f, "{}{} // {}", minus, self.numerator, self.denominator)
    }
}

impl fmt::Debug for Rational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self, f)
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
        unsafe { Self::new_unchecked(minus, numerator, denominator) }
    }
}
forward_ref_binop! { Mul, mul for Rational, Rational }
impl_assignop! { MulAssign, mul_assign, Mul, mul for Rational }

impl Div for Rational {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.recip()
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
                unsafe { Self::new_unchecked(self.minus, numerator, denominator) }
            }
            (true, false) | (false, true) => {
                let ln = self.numerator * (denominator / self.denominator);
                let rn = rhs.numerator * (denominator / rhs.denominator);
                let (minus, numerator) = if ln >= rn {
                    (self.minus, ln - rn)
                } else {
                    (rhs.minus, rn - ln)
                };
                unsafe { Self::new_unchecked(minus, numerator, denominator) }
            }
        }
    }
}
forward_ref_binop! { Add, add for Rational, Rational }
impl_assignop! { AddAssign, add_assign, Add, add for Rational }

impl Neg for Rational {
    type Output = Self;

    fn neg(self) -> Self::Output {
        unsafe { Self::new_unchecked(!self.minus, self.numerator, self.denominator) }
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
            (true, true) | (false, false) => {
                let t = self - other;
                if t.numerator == 0 {
                    Equal
                } else if t.minus {
                    Less
                } else {
                    Greater
                }
            }
        }
    }
}

impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

macro_rules! impl_from_uint {
        ($($t:ty)*) => {$(
            impl From<$t> for Rational {
                fn from(x: $t) -> Self {
                    (x, 1).into()
                }
            }
            impl From<($t, $t)> for Rational {
                fn from(x: ($t, $t)) -> Self {
                    unsafe { Self::new_unchecked(false, x.0 as RationalUnit, x.1 as RationalUnit) }
                }
            }
        )*};
    }

macro_rules! impl_from_int {
    ($($t:ty)*) => {$(
        impl From<$t> for Rational {
            fn from(x: $t) -> Self {
                (x, 1).into()
            }
        }
        impl From<($t, $t)> for Rational {
            fn from(x: ($t, $t)) -> Self {
                unsafe {
                    Self::new_unchecked(
                        x.0.is_negative() ^ x.1.is_negative(),
                        x.0.abs() as RationalUnit,
                        x.1.abs() as RationalUnit
                    )
                }
            }
        }
    )*};
}

impl_from_uint! { u8 u16 u32 usize }
impl_from_uint! { u64 }
impl_from_int! { i8 i16 i32 i64 isize }
impl_from_int! { i128 }

macro_rules! impl_from_float {
    ($($t:tt)*) => {$(
        impl From<Rational> for $t {
            fn from(x: Rational) -> Self {
                let num = x.numerator as $t;
                let deno = x.denominator as $t;
                let f = num / deno;
                if x.minus { -f } else { f }
            }
        }
    )*};
}

impl_from_float! { f32 f64 }

macro_rules! impl_binop_prim {
    ($trait:ident, $fn:ident, $assign_trait:ident, $assign_fn:ident for $($t:ty)*) => {$(
        impl $trait<$t> for Rational {
            type Output = <Rational as $trait>::Output;
            fn $fn(self, rhs: $t) -> Self::Output {
                $trait::$fn(self, Rational::from(rhs))
            }
        }
        impl $trait<Rational> for $t {
            type Output = <Rational as $trait>::Output;
            fn $fn(self, rhs: Rational) -> Self::Output {
                $trait::$fn(Rational::from(self), rhs)
            }
        }
        forward_ref_binop! { $trait, $fn for Rational, $t }
        forward_ref_binop! { $trait, $fn for $t, Rational }
        impl_assignop! { $assign_trait, $assign_fn, $trait, $fn for $t }
    )*};
}

impl_binop_prim! { Add, add, AddAssign, add_assign for u8 u16 u32 usize i8 i16 i32 i64 isize }
impl_binop_prim! { Add, add, AddAssign, add_assign for u64 i128 }
impl_binop_prim! { Sub, sub, SubAssign, sub_assign for u8 u16 u32 usize i8 i16 i32 i64 isize }
impl_binop_prim! { Sub, sub, SubAssign, sub_assign for u64 i128 }
impl_binop_prim! { Mul, mul, MulAssign, mul_assign for u8 u16 u32 usize i8 i16 i32 i64 isize }
impl_binop_prim! { Mul, mul, MulAssign, mul_assign for u64 i128 }
impl_binop_prim! { Div, div, DivAssign, div_assign for u8 u16 u32 usize i8 i16 i32 i64 isize }
impl_binop_prim! { Div, div, DivAssign, div_assign for u64 i128 }

macro_rules! impl_cmp_int {
    (@for $t:ty) => {
        impl PartialEq<$t> for Rational {
            fn eq(&self, other: &$t) -> bool {
                PartialEq::eq(self, &Rational::from(*other))
            }
        }

        impl PartialEq<Rational> for $t {
            fn eq(&self, other: &Rational) -> bool {
                PartialEq::eq(&Rational::from(*self), other)
            }
        }

        impl PartialOrd<$t> for Rational {
            fn partial_cmp(&self, other: &$t) -> Option<Ordering> {
                PartialOrd::partial_cmp(self, &Rational::from(*other))
            }
        }

        impl PartialOrd<Rational> for $t {
            fn partial_cmp(&self, other: &Rational) -> Option<Ordering> {
                PartialOrd::partial_cmp(&Rational::from(*self), other)
            }
        }
    };
    ($($t:ty)*) => {$(
        impl_cmp_int! { @for $t }
        impl_cmp_int! { @for ($t, $t) }
    )*};
}

impl_cmp_int! { u8 u16 u32 usize i8 i16 i32 i64 isize }
impl_cmp_int! { u64 i128 }

macro_rules! impl_cmp_float {
    ($($t:tt)*) => {$(
        impl PartialEq<$t> for Rational {
            fn eq(&self, other: &$t) -> bool {
                PartialEq::eq(&$t::from(*self), other)
            }
        }

        impl PartialEq<Rational> for $t {
            fn eq(&self, other: &Rational) -> bool {
                PartialEq::eq(self, &$t::from(*other))
            }
        }

        impl PartialOrd<$t> for Rational {
            fn partial_cmp(&self, other: &$t) -> Option<Ordering> {
                PartialOrd::partial_cmp(&$t::from(*self), other)
            }
        }

        impl PartialOrd<Rational> for $t {
            fn partial_cmp(&self, other: &Rational) -> Option<Ordering> {
                PartialOrd::partial_cmp(self, &$t::from(*other))
            }
        }
    )*};
}

impl_cmp_float! { f32 f64 }

impl Sum for Rational {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |a, b| a + b)
    }
}

impl<'a> Sum<&'a Rational> for Rational {
    fn sum<I: Iterator<Item = &'a Rational>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |a, b| a + b)
    }
}

impl Product for Rational {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, b| a * b)
    }
}

impl<'a> Product<&'a Rational> for Rational {
    fn product<I: Iterator<Item = &'a Rational>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, b| a * b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cmp_float() {
        assert_eq!(Rational::from((1, -3)), -1.0 / 3.0);
    }

    #[test]
    fn cmp_int() {
        assert_eq!(Rational::from((9, 3)), 3);
        assert_eq!(Rational::from((-9, 3)), -3);
        assert_eq!(Rational::from((9, -3)), -3);
        assert_eq!(Rational::from((-9, -3)), 3);
    }

    #[test]
    fn add() {
        let a: Rational = (7, 6).into();
        let b: Rational = (7, 4).into();
        assert_eq!(a + b, (35, 12));
    }

    #[test]
    fn sub() {
        let a: Rational = (7, 6).into();
        let b: Rational = (7, 4).into();
        assert_eq!(a - b, (-7, 12));
    }

    #[test]
    fn mul() {
        let a: Rational = (7, 6).into();
        let b: Rational = (7, 4).into();
        assert_eq!(a * b, (49, 24));
    }

    #[test]
    fn div() {
        let a: Rational = (7, 6).into();
        let b: Rational = (7, 4).into();
        assert_eq!(a / b, (2, 3));
    }
}
