use macro_forward_ref_binop::forward_ref_binop;
use std::{
    cmp::Ordering,
    fmt,
    iter::{Product, Sum},
    marker::PhantomData,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    str::FromStr,
};

pub mod modulus;
pub use modulus::Modulus;
use modulus::{Modulus1e9_7, Modulus998_244_353};

/// The inner type of [`ModInt`].
pub type Set = usize;

/// An integer that defines [modular arithmetic](https://en.wikipedia.org/wiki/Modular_arithmetic).
///
/// The number that consider only remainders.
/// Given an integer `n > 1`, called a **modulus**,
/// two integers `a` and `b` are said to be **congruent** modulo `n`
/// if `n` is a divisor of their difference.
///
/// # Examples
///
/// In modulus `7`, `13` and `27` are congruent (`13 % 7 = 27 % 7`).
///
/// [`ModInt`] can represent this relation following:
///
/// ```
/// use modint::define_modint;
/// define_modint! { MI mod M = 7 }
///
/// let a: MI = 13.into();
/// let b: MI = 27.into();
///
/// // 13 ≡ 27 (mod 7)
/// assert_eq!(a, b);
/// ```
///
/// It can be compared with a primitive number.
///
/// ```
/// # use modint::define_modint;
/// # define_modint! { MI mod M = 7 }
/// # let a: MI = 13.into();
/// assert_eq!(a, 27);
/// assert_eq!(a, 13);
/// assert_eq!(a, 6);
/// ```
///
/// It can also calculate arithmetic operations.
///
/// ```
/// # use modint::define_modint;
/// # define_modint! { MI mod M = 7 }
/// let a: MI = 3.into();
/// let b: MI = 5.into();
///
/// assert_eq!(a + b, 1);
/// assert_eq!(a - b, 5);
/// assert_eq!(a * b, 1);
/// assert_eq!(a / b, 2);
///
/// assert_eq!(a + -b, a - b);
/// assert_eq!(a * b.recip(), a / b);
/// assert_eq!(a.pow(3), 6);
/// ```
#[derive(Clone, Copy)]
pub struct ModInt<M>(Set, PhantomData<M>);

/// Convenient macro to define [`ModInt`].
///
/// # Examples
///
/// ```
/// use modint::{define_modint, Modulus};
///
/// define_modint!(MI mod M = 7);
///
/// assert_eq!(M::MOD, 7);
/// assert_eq!(MI::from(13), 6);
/// ```
#[macro_export]
macro_rules! define_modint {
    ($mint:ident mod $mod_name:ident = $mod:expr) => {
        $crate::define_modulus! { $mod_name = $mod }
        $crate::define_modint! { $mint mod $mod_name }
    };
    ($mint:ident mod $mod_name:ident) => {
        pub type $mint = $crate::ModInt<$mod_name>;
    };
}

define_modint! { ModInt1e9_7 mod Modulus1e9_7 }
define_modint! { ModInt998_244_353 mod Modulus998_244_353 }

impl<M: Modulus> ModInt<M> {
    /// Creates a integer mod `M`.
    ///
    /// # Examples
    ///
    /// ```
    /// use modint::define_modint;
    /// define_modint!(MI mod M = 7);
    ///
    /// let a = MI::new(57);
    ///
    /// assert_eq!(a, 1);
    /// ```
    pub fn new(x: Set) -> Self {
        // SAFETY: pass remainder
        unsafe { Self::new_raw(x % M::MOD) }
    }

    /// Creates a new [`ModInt`] without calculate remainder.
    ///
    /// # Safety
    ///
    /// Calling this function with a value that is lager than `M::MOD` is undefined behavior.
    pub unsafe fn new_raw(x: Set) -> Self {
        Self(x, PhantomData)
    }

    /// Returns zero.
    ///
    /// # Examples
    ///
    /// ```
    /// # use modint::define_modint;
    /// define_modint!(MI mod M = 7);
    ///
    /// let a = MI::new(3);
    /// let zero = MI::zero();
    ///
    /// assert_eq!(a + zero, a);
    /// ```
    pub fn zero() -> Self {
        // SAFETY: modulus is greater than 1
        unsafe { Self::new_raw(0) }
    }

    /// Returns one.
    ///
    /// # Examples
    ///
    /// ```
    /// # use modint::define_modint;
    /// define_modint!(MI mod M = 7);
    ///
    /// let a = MI::new(3);
    /// let one = MI::one();
    ///
    /// assert_eq!(a * one, a);
    /// ```
    pub fn one() -> Self {
        // SAFETY: modulus is greater than 1
        unsafe { Self::new_raw(1) }
    }

    /// Calculate `self` to the power of `exp` on modulo `M`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use modint::define_modint;
    /// define_modint!(MI mod M = 7);
    ///
    /// let a = MI::new(3);
    ///
    /// assert_eq!(a.pow(2), 2);
    /// assert_eq!(a.pow(3), 6);
    /// assert_eq!(a.pow(0), 1);
    /// ```
    pub fn pow(self, mut exp: usize) -> Self {
        let mut base = self;
        let mut acc = Self::one();

        while exp > 0 {
            if (exp & 1) == 1 {
                acc *= base;
            }

            exp >>= 1;
            base *= base;
        }

        acc
    }

    /// Take the reciprocal of number, `1/self`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use modint::define_modint;
    /// define_modint!(MI mod M = 7);
    ///
    /// let a = MI::new(3);
    ///
    /// assert_eq!(a.recip(), 5);
    /// assert_eq!(a.recip() * a, 1);
    /// ```
    pub fn recip(self) -> Self {
        self.pow(M::MOD - 2)
    }
}

impl<M: Modulus> Default for ModInt<M> {
    fn default() -> Self {
        Self::zero()
    }
}

impl<M: Modulus> From<Set> for ModInt<M> {
    fn from(x: Set) -> Self {
        Self::new(x)
    }
}

impl<M: Modulus> From<ModInt<M>> for Set {
    fn from(x: ModInt<M>) -> Self {
        x.0
    }
}

impl<M: Modulus> FromStr for ModInt<M> {
    type Err = <Set as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.parse::<Set>()?.into())
    }
}

impl<M: Modulus> Add for ModInt<M> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let y = self.0 + rhs.0;
        let y = if y >= M::MOD { y - M::MOD } else { y };
        // SAFETY: first `y` is `y <= 2 * (M::MOD - 1)`
        // and second `y` is `y <= M::MOD - 2`.
        unsafe { Self::new_raw(y) }
    }
}

impl<M: Modulus> Sub for ModInt<M> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let y = if self.0 < rhs.0 {
            self.0 + M::MOD - rhs.0
        } else {
            self.0 - rhs.0
        };
        // SAFETY: if self.0 < rhs.0, min value is `1` (self.0 = 0, rhs.0 = M::MOD-1)
        // max value is `M::MOD - 1` (rhs.0 - self.0 = 1)
        // else (self.0 >= rhs.0), min value is `0` (self.0 = rhs.0)
        // max value is `M::MOD - 1` (self.0 = M::MOD-1, rhs.0 = 0).
        unsafe { Self::new_raw(y) }
    }
}

impl<M: Modulus> Mul for ModInt<M> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.0 * rhs.0)
    }
}

impl<M: Modulus> Div for ModInt<M> {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.recip()
    }
}

impl<M: Modulus> Neg for ModInt<M> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        if self.0 == 0 {
            Self::zero()
        } else {
            // SAFETY: min value is `0` (self.0 = M::MOD-1)
            // max value is `M::MOD - 1` (self.0 = 1).
            unsafe { Self::new_raw(M::MOD - self.0) }
        }
    }
}

forward_ref_binop! { impl<M: Modulus> Add, add for ModInt<M>, ModInt<M> }
forward_ref_binop! { impl<M: Modulus> Sub, sub for ModInt<M>, ModInt<M> }
forward_ref_binop! { impl<M: Modulus> Mul, mul for ModInt<M>, ModInt<M> }
forward_ref_binop! { impl<M: Modulus> Div, div for ModInt<M>, ModInt<M> }

impl<M: Modulus> Neg for &ModInt<M> {
    type Output = ModInt<M>;

    fn neg(self) -> Self::Output {
        -(*self)
    }
}

macro_rules! impl_binop_prim {
    ($trait:ident, $fn:ident for $t:ty) => {
        impl<M: Modulus> $trait<$t> for ModInt<M> {
            type Output = Self;

            fn $fn(self, rhs: $t) -> Self::Output {
                <ModInt<M> as $trait>::$fn(self, rhs.into())
            }
        }

        forward_ref_binop! { impl<M: Modulus> $trait, $fn for ModInt<M>, $t }

        impl<M: Modulus> $trait<ModInt<M>> for $t {
            type Output = ModInt<M>;

            fn $fn(self, rhs: ModInt<M>) -> Self::Output {
                <ModInt<M> as $trait>::$fn(self.into(), rhs)
            }
        }

        forward_ref_binop! { impl<M: Modulus> $trait, $fn for $t, ModInt<M> }
    };
}

impl_binop_prim! { Add, add for Set }
impl_binop_prim! { Sub, sub for Set }
impl_binop_prim! { Mul, mul for Set }
impl_binop_prim! { Div, div for Set }

macro_rules! impl_assign {
    ($trait:ident, $fn:ident, $expr:ident for $($t:ty)*) => {$(
        impl<M: Modulus> $trait<$t> for ModInt<M> {
            fn $fn(&mut self, rhs: $t) {
                *self = self.$expr(rhs)
            }
        }

        impl<M: Modulus> $trait<&$t> for ModInt<M> {
            fn $fn(&mut self, rhs: &$t) {
                *self = self.$expr(rhs)
            }
        }
    )*};
}

impl_assign! { AddAssign, add_assign, add for ModInt<M> Set }
impl_assign! { SubAssign, sub_assign, sub for ModInt<M> Set }
impl_assign! { MulAssign, mul_assign, mul for ModInt<M> Set }
impl_assign! { DivAssign, div_assign, div for ModInt<M> Set }

impl<M> PartialEq for ModInt<M> {
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(&self.0, &other.0)
    }
}

impl<M: Modulus> Eq for ModInt<M> {}

impl<M: Modulus> Ord for ModInt<M> {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(&self.0, &other.0)
    }
}

impl<M: Modulus> PartialOrd for ModInt<M> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

macro_rules! impl_cmp_prim {
    ($trait:ident, $fn:ident -> $ret:ty; for $($t:ty)*) => {$(
        impl<M: Modulus> $trait<$t> for ModInt<M> {
            fn $fn(&self, other: &$t) -> $ret {
                <ModInt<M> as $trait>::$fn(self, &(*other).into())
            }
        }

        impl<M: Modulus> $trait<ModInt<M>> for $t {
            fn $fn(&self, other: &ModInt<M>) -> $ret {
                <ModInt<M> as $trait>::$fn(&(*self).into(), other)
            }
        }
    )*};
}

impl_cmp_prim! { PartialEq, eq -> bool; for Set }
impl_cmp_prim! { PartialOrd, partial_cmp -> Option<Ordering>; for Set }

macro_rules! impl_iter_sum_product {
    ($trait:ident, $fn:ident, $id:expr, $op:ident) => {
        impl<M: Modulus> $trait for ModInt<M> {
            fn $fn<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold($id, |a, b| a.$op(b))
            }
        }

        impl<'a, M: Modulus> $trait<&'a ModInt<M>> for ModInt<M> {
            fn $fn<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
                iter.fold($id, |a, b| a.$op(b))
            }
        }
    };
}

impl_iter_sum_product! { Sum, sum, Self::zero(), add }
impl_iter_sum_product! { Product, product, Self::one(), mul }

macro_rules! impl_fmt {
    ($($trait:ident)*) => {$(
        impl<M> fmt::$trait for ModInt<M> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::$trait::fmt(&self.0, f)
            }
        }
    )*};
}

impl_fmt! { Debug Display Binary Octal LowerHex UpperHex }
