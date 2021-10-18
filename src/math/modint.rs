pub mod modulo;
pub use modulo::*;

pub mod map;
pub use map::*;

#[codesnip::entry("ModInt1000000007")]
pub use modint_1000000007::ModInt1000000007;
#[codesnip::entry("ModInt1000000007", include("define_modint", "Modulo1000000007"))]
mod modint_1000000007 {
    use super::{define_modint, Modulo1000000007};
    define_modint!(Modulo1000000007 for ModInt1000000007<usize>);

    #[cfg(test)]
    mod tests {
        use super::super::modint_tests;
        modint_tests!(ModInt1000000007);
    }
}

#[codesnip::entry("ModInt998244353")]
pub use modint_998244353::ModInt998244353;
#[codesnip::entry("ModInt998244353", include("define_modint", "Modulo998244353"))]
mod modint_998244353 {
    use super::{define_modint, Modulo998244353};
    define_modint!(Modulo998244353 for ModInt998244353<usize>);

    #[cfg(test)]
    mod tests {
        use super::super::modint_tests;
        modint_tests!(ModInt998244353);
    }
}

#[codesnip::entry("ModInt", include("Modulo"))]
pub trait ModInt {
    type Set;
    type Modulo: Modulo<Set = Self::Set>;
    fn new(x: Self::Set) -> Self;
    fn get(&self) -> &Self::Set;
    fn pow(&self, exp: u32) -> Self;
    fn inverse(&self) -> Self;
}

#[codesnip::entry(include("Modulo", "ModInt"))]
#[allow(unused_macros)]
macro_rules! define_modint {
    (@impl_ops_with_plain for $modint:ident<$t:ty> $($imp:ident, $fn:ident, $assign_imp:ident, $assign_fn:ident);* $(;)?) => {$(
        impl $imp<$t> for $modint {
            type Output = $modint;

            fn $fn(self, rhs: $t) -> $modint {
                $imp::$fn(self, $modint::from(rhs))
            }
        }

        impl $imp<$modint> for $t {
            type Output = $modint;

            fn $fn(self, rhs: $modint) -> $modint {
                $imp::$fn($modint::from(self), rhs)
            }
        }

        define_modint! { @forward_ref_binops
            $imp, $fn for ($modint,$modint; $modint,$t; $t,$modint)
        }
        define_modint! { @impl_assign_ops
            $assign_imp, $assign_fn, $imp, $fn for ($modint,$modint; $modint, $t)
        }
    )*};
    (@forward_ref_binops $imp:ident, $fn:ident for ($($t:ty, $u:ty);+)) => {$(
        impl $imp<$u> for &$t {
            type Output = <$t as $imp<$u>>::Output;

            fn $fn(self, rhs: $u) -> <$t as $imp<$u>>::Output {
                $imp::$fn(self.clone(), rhs)
            }
        }

        impl $imp<&$u> for $t {
            type Output = <$t as $imp<$u>>::Output;

            fn $fn(self, rhs: &$u) -> <$t as $imp<$u>>::Output {
                $imp::$fn(self, rhs.clone())
            }
        }

        impl $imp<&$u> for &$t {
            type Output = <$t as $imp<$u>>::Output;

            fn $fn(self, rhs: &$u) -> <$t as $imp<$u>>::Output {
                $imp::$fn(self.clone(), rhs.clone())
            }
        }
    )+};
    (@impl_assign_ops $imp:ident, $fn:ident, $ops_imp:ident, $ops_fn:ident for ($($t:ty, $u:ty);+)) => {$(
        impl $imp<$u> for $t {
            fn $fn(&mut self, rhs: $u) {
                *self = $ops_imp::$ops_fn(*self, rhs)
            }
        }
    )+};
    (@impl_fmt ($($trait:ident),+) for $t:ty) => {$(
        impl fmt::$trait for $t {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                self.get().fmt(f)
            }
        }
    )+};
    (@impl_cmp for $modint:ident<$t:ty> $($trait:ident, $fn:ident -> $ret:ty);* $(;)?) => {$(
        impl $trait<$t> for $modint {
            fn $fn(&self, other: &$t) -> $ret {
                $trait::$fn(self.get(), other)
            }
        }
        impl $trait<$modint> for $t {
            fn $fn(&self, other: &$modint) -> $ret {
                $trait::$fn(self, other.get())
            }
        }
    )*};
    ($modulo:ident for $modint:ident<$t:ty>) => {
        use super::{Modulo, ModInt};
        use core::{
            cmp::Ordering,
            fmt::{self, Formatter},
            iter::{Product, Sum},
            ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
            str::FromStr,
        };
        #[derive(Clone, Copy, Default, Hash, PartialOrd, Ord, PartialEq, Eq)]
        pub struct $modint($t);

        impl ModInt for $modint {
            type Set = $t;
            type Modulo = $modulo;

            fn new(x: $t) -> Self {
                Self(x % Self::Modulo::MOD)
            }

            fn get(&self) -> &Self::Set {
                &self.0
            }

            fn pow(&self, mut exp: u32) -> Self {
                let mut x = self.clone();
                let mut res = 1.into();
                while exp > 1 {
                    if exp & 1 == 1 {
                        res *= x;
                    }
                    x *= x;
                    exp >>= 1;
                }
                if exp == 1 {
                    res *= x;
                }
                res
            }

            fn inverse(&self) -> Self {
                self.pow((<Self as ModInt>::Modulo::MOD - 2) as u32)
            }
        }

        impl From<$t> for $modint {
            fn from(x: $t) -> $modint {
                Self::new(x)
            }
        }

        impl From<$modint> for $t {
            fn from(x: $modint) -> $t {
                x.0
            }
        }

        define_modint! { @impl_fmt
            (Debug, Display, Binary, Octal, LowerHex, UpperHex, LowerExp, UpperExp) for $modint
        }

        impl FromStr for $modint {
            type Err = <$t as FromStr>::Err;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(s.parse::<$t>()?.into())
            }
        }

        define_modint! { @impl_cmp for $modint<$t>
            PartialEq, eq -> bool;
            PartialOrd, partial_cmp -> Option<Ordering>;
        }

        impl Add for $modint {
            type Output = Self;

            fn add(self, rhs: Self) -> Self {
                let res = self.0 + rhs.0;
                if res >= <Self as ModInt>::Modulo::MOD {
                    res - <Self as ModInt>::Modulo::MOD
                } else {
                    res
                }
                .into()
            }
        }

        impl Sub for $modint {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self {
                if self < rhs {
                    self.0 + <Self as ModInt>::Modulo::MOD - rhs.0
                } else {
                    self.0 - rhs.0
                }
                .into()
            }
        }

        impl Mul for $modint {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self {
                (self.0 * rhs.0).into()
            }
        }

        impl Div for $modint {
            type Output = Self;

            fn div(self, rhs: Self) -> Self {
                self * rhs.inverse()
            }
        }

        impl Neg for $modint {
            type Output = Self;

            fn neg(self) -> Self {
                <Self as ModInt>::Modulo::MOD - self
            }
        }

        impl Sum for $modint {
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(0.into(), |a, b| a + b)
            }
        }

        impl<'a> Sum<&'a $modint> for $modint {
            fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
                iter.fold(0.into(), |a, b| a + b)
            }
        }

        impl Product for $modint {
            fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(1.into(), |a, b| a * b)
            }
        }

        impl <'a>Product <&'a $modint>for $modint {
            fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
                iter.fold(1.into(), |a, b| a * b)
            }
        }

        define_modint! { @impl_ops_with_plain for $modint<$t>
            Add, add, AddAssign, add_assign;
            Sub, sub, SubAssign, sub_assign;
            Mul, mul, MulAssign, mul_assign;
            Div, div, DivAssign, div_assign;
        }
    };
}
#[codesnip::entry("define_modint")]
#[allow(unused_imports)]
pub(crate) use define_modint;

#[cfg(test)]
macro_rules! modint_tests {
    ($modint:ident) => {
        use super::{$modint, ModInt, Modulo};
        const MOD: usize = <$modint as ModInt>::Modulo::MOD;

        #[test]
        fn add() {
            let a = $modint::new(13);
            let b = $modint::new(20);
            assert_eq!(33, a + b);
            assert_eq!(a, a + MOD);
        }

        #[test]
        fn sub() {
            let a = $modint::new(13);
            let b = $modint::new(20);
            assert_eq!(MOD - 7, a - b);
            assert_eq!(a, a - MOD);
        }

        #[test]
        fn mul() {
            let a = $modint::new(13);
            let b = $modint::new(20);
            assert_eq!(260, a * b);
            assert_eq!(0, a * MOD);
        }

        #[test]
        fn div() {
            let a = $modint::new(33);
            let b = $modint::new(3);
            assert_eq!(11, a / b);
            assert_eq!(1, a / a);
        }

        #[test]
        fn pow() {
            let a = $modint::new(2);
            assert_eq!(1024, a.pow(10));
            assert_eq!(a, a.pow(MOD as u32));
        }

        #[test]
        fn inv() {
            let a = $modint::new(33);
            assert_eq!(1, a * a.inverse());
        }
    };
}
#[cfg(test)]
pub(crate) use modint_tests;
