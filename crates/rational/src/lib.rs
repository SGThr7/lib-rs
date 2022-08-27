use std::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    iter::{Product, Sum},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    u128, u16, u32, u64, u8, usize,
};

use gcd_lcm::{Gcd, Lcm};
use macro_forward_ref_binop::forward_ref_binop;

pub type RationalBase = usize;
const RATIONAL_BASE_MAX: RationalBase = usize::MAX;

#[derive(Clone, Copy)]
pub struct Rational {
    minus: bool,
    numerator: usize,
    denominator: usize,
}

impl Rational {
    /// Returns zero rational.
    ///
    /// # Examples
    ///
    /// ```
    /// use rational::Rational;
    ///
    /// assert_eq!(Rational::ZERO, 0.into());
    ///
    /// let a: Rational = 7.into();
    /// assert_eq!(a + Rational::ZERO, a);
    /// assert_eq!(a * Rational::ZERO, Rational::ZERO);
    /// ```
    pub const ZERO: Self = Self {
        minus: false,
        numerator: 0,
        denominator: 1,
    };

    /// Returns one rational.
    ///
    /// # Examples
    ///
    /// ```
    /// use rational::Rational;
    ///
    /// assert_eq!(Rational::ONE, 1.into());
    ///
    /// let a: Rational = 7.into();
    /// assert_eq!(a * Rational::ONE, a);
    /// ```
    pub const ONE: Self = Self {
        minus: false,
        numerator: 1,
        denominator: 1,
    };

    /// Returns the maximum rational.
    ///
    /// # Examples
    ///
    /// ```
    /// use rational::Rational;
    ///
    /// let a: Rational = (3, 57).into();
    /// assert!(Rational::MAX >= a);
    /// ```
    pub const MAX: Self = Self {
        minus: false,
        numerator: RATIONAL_BASE_MAX,
        denominator: 1,
    };

    /// Returns the minimum rational.
    ///
    /// # Examples
    ///
    /// ```
    /// use rational::Rational;
    ///
    /// let a: Rational = (3, 57).into();
    /// assert!(Rational::MIN <= a);
    /// ```
    pub const MIN: Self = Self {
        minus: true,
        numerator: RATIONAL_BASE_MAX,
        denominator: 1,
    };

    /// Makes a new rational number without checking zero denominator.
    fn new_unchecked(minus: bool, numerator: usize, denominator: usize) -> Self {
        if numerator == 0 {
            Self::ZERO
        } else {
            Self {
                minus,
                numerator,
                denominator,
            }
            .normalize()
        }
    }

    /// Makes a new rational number.
    ///
    /// # Examples
    ///
    /// ```
    /// use rational::Rational;
    ///
    /// // 3 // 57 == 1 // 19
    /// let a: Rational = (3, 57).into();
    /// // 1 // 19
    /// let b: Rational = (1, 19).into();
    /// assert_eq!(a, b);
    /// ```
    pub fn new(minus: bool, numerator: usize, denominator: usize) -> Self {
        assert_ne!(denominator, 0, "denominator must be non-zero");

        Self::new_unchecked(minus, numerator, denominator)
    }

    /// Reduce a fraction.
    /// This function does *not* check zero denominator.
    fn normalize(self) -> Self {
        let gcd = self.numerator.gcd(self.denominator);
        Self {
            minus: self.minus,
            numerator: self.numerator / gcd,
            denominator: self.denominator / gcd,
        }
    }

    /// Takes the reciprocal (inverse) of a number.
    ///
    /// # Examples
    ///
    /// ```
    /// use rational::Rational;
    ///
    /// let a: Rational = (3, 57).into();
    /// assert_eq!(a.recip(), (57, 3).into());
    /// assert_eq!(a.recip(), Rational::ONE / a);
    /// assert_eq!(a * a.recip(), Rational::ONE);
    /// ```
    ///
    /// ```should_panic
    /// # use rational::Rational;
    /// let a = Rational::ZERO;
    /// // panic: division by zero
    /// let recip = a.recip();
    /// ```
    pub fn recip(self) -> Self {
        assert_ne!(self.numerator, 0, "cannot get reciprocal of zero");

        Self::new_unchecked(self.minus, self.denominator, self.numerator)
    }

    /// Raises `self` to the power of `exp`.
    ///
    /// Using exponentiation by squaring.
    ///
    /// # Examples
    ///
    /// ```
    /// use rational::Rational;
    ///
    /// let a: Rational = (3, 57).into();
    ///
    /// assert_eq!(a.pow(0), Rational::ONE);
    /// assert_eq!(a.pow(1), a);
    /// assert_eq!(a.pow(2), a * a);
    /// ```
    pub fn pow(self, exp: u32) -> Self {
        let is_odd_exp = (exp & 1) == 1;
        Self::new_unchecked(
            self.minus && is_odd_exp,
            self.numerator.pow(exp),
            self.denominator.pow(exp),
        )
    }

    /// Computes the absolute value of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rational::Rational;
    ///
    /// let a: Rational = (3, 57).into();
    /// let b: Rational = (-3, 57).into();
    ///
    /// assert_eq!(a.abs(), a);
    /// assert_eq!(b.abs(), a);
    /// ```
    pub fn abs(mut self) -> Self {
        self.minus = false;
        self
    }

    /// Returns `true` if `self` is positive.
    ///
    /// # Examples
    ///
    /// ```
    /// use rational::Rational;
    ///
    /// let a: Rational = (3, 57).into();
    ///
    /// assert!(    a.is_positive());
    /// assert!(!(-a).is_positive());
    /// ```
    pub fn is_positive(&self) -> bool {
        !self.minus
    }

    /// Returns `true` if `self` is negative.
    ///
    /// # Examples
    ///
    /// ```
    /// use rational::Rational;
    ///
    /// let a: Rational = (3, 57).into();
    ///
    /// assert!(!   a.is_negative());
    /// assert!( (-a).is_negative());
    /// ```
    pub fn is_negative(&self) -> bool {
        self.minus
    }
}

impl Default for Rational {
    fn default() -> Self {
        Self::ZERO
    }
}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.minus {
            write!(f, "-")?;
        }
        write!(f, "{} // {}", self.numerator, self.denominator)
    }
}

impl fmt::Debug for Rational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

macro_rules! impl_assignop {
    ($assign_trait:ident, $assign_fn:ident, $op_trait:ident, $op_fn:ident for $t:tt $u:tt) => {
        impl $assign_trait<$u> for $t {
            fn $assign_fn(&mut self, other: $u) {
                *self = $op_trait::$op_fn(*self, other)
            }
        }
        impl $assign_trait<&$u> for $t {
            fn $assign_fn(&mut self, other: &$u) {
                *self = $op_trait::$op_fn(*self, other)
            }
        }
    };
}

impl Add for Rational {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let denominator = self.denominator.lcm(rhs.denominator);
        let lhs_numerator = self.numerator * (denominator / self.denominator);
        let rhs_numerator = rhs.numerator * (denominator / rhs.denominator);
        match (self.minus, rhs.minus) {
            (true, true) | (false, false) => {
                let numerator = lhs_numerator + rhs_numerator;
                Self::new(self.minus, numerator, denominator)
            }
            (true, false) | (false, true) => {
                let (minus, numerator) = if lhs_numerator >= rhs_numerator {
                    (self.minus, lhs_numerator - rhs_numerator)
                } else {
                    (rhs.minus, rhs_numerator - lhs_numerator)
                };
                Self::new(minus, numerator, denominator)
            }
        }
    }
}
forward_ref_binop! { impl Add, add for Rational, Rational }
impl_assignop! { AddAssign, add_assign, Add, add for Rational Rational }

impl Neg for Rational {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            minus: !self.minus,
            numerator: self.numerator,
            denominator: self.denominator,
        }
    }
}

impl Neg for &'_ Rational {
    type Output = <Rational as Neg>::Output;

    fn neg(self) -> Self::Output {
        Rational::neg(*self)
    }
}

impl Sub for Rational {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}
forward_ref_binop! { impl Sub, sub for Rational, Rational }
impl_assignop! { SubAssign, sub_assign, Sub, sub for Rational Rational }

impl Mul for Rational {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let minus = self.minus ^ rhs.minus;
        let numerator = self.numerator * rhs.numerator;
        let denominator = self.denominator * rhs.denominator;
        Self::new_unchecked(minus, numerator, denominator)
    }
}
forward_ref_binop! { impl Mul, mul for Rational, Rational }
impl_assignop! { MulAssign, mul_assign, Mul, mul for Rational Rational }

impl Div for Rational {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.recip()
    }
}
forward_ref_binop! { impl Div, div for Rational, Rational }
impl_assignop! { DivAssign, div_assign, Div, div for Rational Rational }

impl Ord for Rational {
    /// Returns an [`Ordering`] between `self` and `other`.
    ///
    /// # Complexity
    ///
    /// | Time  |
    /// | ----- |
    /// | ln(n) |
    ///
    /// # Examples
    ///
    /// ```
    /// use rational::Rational;
    /// use std::cmp::Ordering;
    ///
    /// let five: Rational = 5.into();
    /// let ten: Rational = 10.into();
    ///
    /// assert_eq!(five.cmp(&ten), Ordering::Less);
    /// assert_eq!(ten.cmp(&five), Ordering::Greater);
    /// assert_eq!(five.cmp(&five), Ordering::Equal);
    /// ```
    fn cmp(&self, other: &Self) -> Ordering {
        use Ordering::*;
        match (self.minus, other.minus) {
            (true, false) => Less,
            (false, true) => Greater,
            // (true, true) | (false, false)
            (minus, _) => {
                let mut lhs = (self.numerator, self.denominator);
                let mut rhs = (other.numerator, other.denominator);

                if lhs == rhs || (lhs.0 == 0 && rhs.0 == 0) {
                    return Equal;
                }

                let mut rev = minus;

                while lhs.1 != 0 && rhs.1 != 0 {
                    let lhs_quot = lhs.0 / lhs.1;
                    let rhs_quot = rhs.0 / rhs.1;

                    match lhs_quot.cmp(&rhs_quot) {
                        Equal => (),
                        cmp => return if rev { cmp.reverse() } else { cmp },
                    }

                    rev = !rev;
                    lhs = (lhs.1, lhs.0.rem_euclid(lhs.1));
                    rhs = (rhs.1, rhs.0.rem_euclid(rhs.1));
                }

                let ret = match (lhs.1 == 0, rhs.1 == 0) {
                    // Equal
                    (true, true) => unreachable!(),
                    (false, false) => unreachable!(),
                    (true, false) => Greater,
                    (false, true) => Less,
                };

                if rev {
                    ret.reverse()
                } else {
                    ret
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

impl PartialEq for Rational {
    fn eq(&self, other: &Self) -> bool {
        matches!(self.cmp(other), Ordering::Equal)
    }
}

impl Eq for Rational {}

impl Hash for Rational {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.minus.hash(state);
        self.numerator.hash(state);
        self.denominator.hash(state);
    }
}

// impl From<RationalBase> for Rational {
//     fn from(v: RationalBase) -> Self {
//         unsafe { Rational::new_raw(false, v, 1) }
//     }
// }

macro_rules! impl_from_int {
    ($($t:ty)*) => {$(
        impl From<$t> for Rational {
            fn from(v: $t) -> Self {
                (v, 1).into()
            }
        }

        impl From<($t, $t)> for Rational {
            fn from(v: ($t, $t)) -> Self {
                let (numerator, denominator) = v;
                let minus = numerator.is_negative() ^ denominator.is_negative();
                Self::new(minus, numerator.abs() as RationalBase, denominator.abs() as RationalBase)
            }
        }
    )*};
}

macro_rules! impl_from_uint {
    ($($t:ty)*) => {$(
        impl From<$t> for Rational {
            fn from(v: $t) -> Self {
                (v, 1).into()
            }
        }

        impl From<($t, $t)> for Rational {
            fn from(v: ($t, $t)) -> Self {
                Self::new(false, v.0 as RationalBase, v.1 as RationalBase)
            }
        }
    )*};
}

macro_rules! impl_into_float {
    ($($u:tt)*) => {$(
        impl From<Rational> for $u {
            fn from(v: Rational) -> Self {
                let f = v.numerator as $u / v.denominator as $u;
                if v.minus {
                    -f
                } else {
                    f
                }
            }
        }
    )*};
}

// Like `as` expression
impl_from_int! { i8 i16 i32 i64 i128 isize }
impl_from_uint! { u8 u16 u32 u64 u128 usize }

impl_into_float! { f32 f64 }

// pub struct TryFromFloatError {
//     kind: TryFromFloatErrorKind,
// }

// pub enum TryFromFloatErrorKind {
//     Nan,
//     Infinite,
// }

// impl TryFrom<f64> for Rational {
//     type Error = TryFromFloatError;

//     fn try_from(value: f64) -> Result<Self, Self::Error> {
//         match value.classify() {
//             FpCategory::Nan => Err(TryFromFloatError {
//                 kind: TryFromFloatErrorKind::Nan,
//             }),
//             FpCategory::Infinite => Err(TryFromFloatError {
//                 kind: TryFromFloatErrorKind::Infinite,
//             }),
//             FpCategory::Zero => Ok(Self::ZERO),
//             FpCategory::Subnormal => todo!(),
//             FpCategory::Normal => {
//                 todo!();
//                 let minus = value.is_sign_negative();
//                 let digit = unsafe { value.log10().to_int_unchecked() };
//                 let order = 10usize.pow(digit);
//                 let numerator = unsafe { (value * order as f64).to_int_unchecked() };
//                 Ok(Self::new(minus, numerator, order))
//             }
//         }
//     }
// }

macro_rules! impl_fold {
    ($trait:ident, $name:ident, $t:tt, $id:expr, $fn:expr) => {
        impl $trait for $t {
            fn $name<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold($id, $fn)
            }
        }

        impl<'a> $trait<&'a $t> for $t {
            fn $name<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
                iter.fold($id, $fn)
            }
        }
    };
}

impl_fold! { Sum, sum, Rational, Self::ZERO, Add::add }
impl_fold! { Product, product, Rational, Self::ONE, Mul::mul }

#[cfg(test)]
mod tests {
    use super::*;

    const RANGE: isize = 25;
    const EPSILON: f64 = 1e-10;

    macro_rules! rat_iter {
        ($range:expr) => {
            $range
                .clone()
                .flat_map(|a1| $range.filter(|&a2| a2 != 0).map(move |a2| (a1, a2)))
        };
    }

    macro_rules! rat_test {
        ($ra:ident, $fa:ident, $rb:ident, $fb:ident, $test:expr) => {
            for (a1, a2) in rat_iter!(-RANGE..=RANGE) {
                let $ra: Rational = (a1, a2).into();
                let $fa: f64 = a1 as f64 / a2 as f64;
                for (b1, b2) in rat_iter!(-RANGE..=RANGE) {
                    let $rb: Rational = (b1, b2).into();
                    let $fb: f64 = b1 as f64 / b2 as f64;

                    $test
                }
            }
        };
    }

    macro_rules! rat_test_op {
        ($fn:path) => {
            rat_test! { ra, fa, rb, fb, {
                let r = $fn(ra, rb);
                let r: f64 = r.into();
                let f = $fn(fa, fb);
                let diff = f - r;
                assert!(diff.abs() <= EPSILON);
            }}
        };
    }

    #[test]
    fn from_int() {
        let _: Rational = 7.into();
        let a: Rational = (3, 57).into();
        let b: Rational = (-3, 57).into();
        let c: Rational = (3, -57).into();
        let d: Rational = (-3, -57).into();
        assert_eq!(a, d);
        assert_eq!(b, c);
    }

    #[test]
    fn add() {
        let a: Rational = (7, 6).into();
        let b: Rational = (7, 4).into();
        let res: Rational = (35, 12).into();
        assert_eq!(a + b, res);

        rat_test_op!(Add::add);
    }

    #[test]
    fn sub() {
        let a: Rational = (7, 6).into();
        let b: Rational = (7, 4).into();
        assert_eq!(a - b, (-7, 12).into());

        rat_test_op!(Sub::sub);
    }

    #[test]
    fn mul() {
        let a: Rational = (7, 6).into();
        let b: Rational = (7, 4).into();
        assert_eq!(a * b, (49, 24).into());

        rat_test_op!(Mul::mul);
    }

    #[test]
    fn div() {
        let a: Rational = (7, 6).into();
        let b: Rational = (7, 4).into();
        assert_eq!(a / b, (2, 3).into());

        rat_test! { ra, fa, rb, fb, {
            if rb == Rational::ZERO || fb == 0.0 {
                continue;
            }
            let r = ra / rb;
            let r: f64 = r.into();
            let f = fa / fb;
            let diff = f - r;
            assert!(diff.abs() <= EPSILON);
        }}
    }

    #[test]
    fn cmp() {
        use Ordering::*;
        let a: Rational = (2, 3).into();
        let b: Rational = (3, 4).into();

        assert_eq!(a.cmp(&b), Less);
        assert_eq!(b.cmp(&a), Greater);
        assert_eq!(a.cmp(&a), Equal);

        assert_eq!(a.cmp(&Rational::MAX), Less);
        assert_eq!(a.cmp(&Rational::MIN), Greater);
    }

    #[test]
    fn cmp_in_range() {
        rat_test! { ra, fa, rb, fb, {
            let r = ra.cmp(&rb);
            let f = fa.partial_cmp(&fb).unwrap();
            assert_eq!(r, f, "a: {}, b: {}", ra, rb);
        }}
    }
}
