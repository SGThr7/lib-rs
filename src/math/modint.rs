use crate::math::num::{One, Zero};

pub mod modulo;
pub use modulo::{Modulo, Modulo1e9_7, Modulo998244353};

pub mod factmap;
pub use factmap::{FactMap1e9_7, FactMap998244353, ModIntFactMap};

#[codesnip::entry("ModInt1e9_7", include("ModInt", "Modulo1e9_7"))]
pub type ModInt1e9_7 = ModInt<Modulo1e9_7>;

#[codesnip::entry("ModInt998244353", include("ModInt", "Modulo998244353"))]
pub type ModInt998244353 = ModInt<Modulo998244353>;

#[codesnip::entry("ModInt")]
pub use modint_impl::ModInt;

#[codesnip::entry("ModInt", include("Modulo", "One", "Zero"))]
mod modint_impl {
    use super::{Modulo, One, Zero};
    use core::{
        cmp::Ordering,
        fmt,
        iter::{Product, Sum},
        marker::PhantomData,
        ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
        str::FromStr,
    };

    type Set = usize;

    #[derive(Default, Clone, Copy)]
    pub struct ModInt<M>(Set, PhantomData<M>);

    impl<M> ModInt<M> {
        fn new_raw(x: Set) -> Self {
            Self(x, PhantomData)
        }

        pub fn zero() -> Self {
            Self::new_raw(0)
        }
    }
    impl<M> ModInt<M>
    where
        M: Modulo<Set = Set>,
    {
        pub fn new(x: Set) -> Self {
            Self::new_raw(x % M::MOD)
        }

        pub fn one() -> Self {
            Self::new(1)
        }
    }
    impl<M> ModInt<M>
    where
        M: Modulo<Set = Set> + Clone,
    {
        pub fn pow(self, mut exp: usize) -> Self {
            if exp == 0 {
                Self::one()
            } else {
                let mut base = self;
                let mut acc = Self::one();

                while exp > 1 {
                    if (exp & 1) == 1 {
                        acc = acc * base.clone();
                    }
                    exp >>= 1;
                    base = base.clone() * base;
                }

                acc * base
            }
        }

        pub fn recip(self) -> Self {
            self.pow(M::MOD - 2)
        }
    }

    impl<M> Zero for ModInt<M> {
        fn zero() -> Self {
            Self::zero()
        }
    }

    impl<M: Modulo<Set = usize>> One for ModInt<M> {
        fn one() -> Self {
            Self::one()
        }
    }

    impl<M: Modulo<Set = Set>> From<Set> for ModInt<M> {
        fn from(x: Set) -> Self {
            Self::new(x)
        }
    }

    impl<M> From<ModInt<M>> for Set {
        fn from(x: ModInt<M>) -> Self {
            x.0
        }
    }

    impl<M: Modulo<Set = Set>> Add for ModInt<M> {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            let res = self.0 + rhs.0;
            let res = if res >= M::MOD { res - M::MOD } else { res };
            Self::new_raw(res)
        }
    }

    impl<M: Modulo<Set = Set>> Sub for ModInt<M> {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            let res = if self.0 < rhs.0 {
                self.0 + M::MOD - rhs.0
            } else {
                self.0 - rhs.0
            };
            Self::new_raw(res)
        }
    }

    impl<M: Modulo<Set = Set>> Neg for ModInt<M> {
        type Output = Self;

        fn neg(self) -> Self::Output {
            if self == 0 {
                Self::zero()
            } else {
                Self::new_raw(M::MOD - self.0)
            }
        }
    }

    impl<M: Modulo<Set = Set>> Mul for ModInt<M> {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self::Output {
            Self::new(self.0 * rhs.0)
        }
    }

    impl<M: Clone + Modulo<Set = Set>> Div for ModInt<M> {
        type Output = Self;

        fn div(self, rhs: Self) -> Self {
            self * rhs.recip()
        }
    }

    macro_rules! forward_ref_binop {
        ($trait:ident, $fn:ident for $t:ty) => {
            impl<M: Clone + Modulo<Set = Set>> $trait<$t> for &ModInt<M> {
                type Output = <ModInt<M> as $trait>::Output;
                fn $fn(self, rhs: $t) -> Self::Output {
                    $trait::$fn(self.clone(), rhs)
                }
            }
            impl<M: Clone + Modulo<Set = Set>> $trait<&$t> for ModInt<M> {
                type Output = <ModInt<M> as $trait>::Output;
                fn $fn(self, rhs: &$t) -> Self::Output {
                    $trait::$fn(self, rhs.clone())
                }
            }
            impl<M: Clone + Modulo<Set = Set>> $trait<&$t> for &ModInt<M> {
                type Output = <ModInt<M> as $trait>::Output;
                fn $fn(self, rhs: &$t) -> Self::Output {
                    $trait::$fn(self.clone(), rhs.clone())
                }
            }
        };
    }

    forward_ref_binop! { Add, add for ModInt<M> }
    forward_ref_binop! { Sub, sub for ModInt<M> }
    forward_ref_binop! { Mul, mul for ModInt<M> }
    forward_ref_binop! { Div, div for ModInt<M> }

    impl<M: Clone + Modulo<Set = Set>> Neg for &ModInt<M> {
        type Output = ModInt<M>;

        fn neg(self) -> Self::Output {
            Neg::neg(self.clone())
        }
    }

    macro_rules! impl_ops {
        ($(<M: $($bounds:path),* $(,)?>)? $trait:ident, $fn:ident for $t:ty) => {
            impl<M: Modulo<Set = Set> $($(+$bounds)*)?> $trait<$t> for ModInt<M> {
                type Output = Self;
                fn $fn(self, rhs: $t) -> Self::Output {
                    $trait::<ModInt<M>>::$fn(self, rhs.into())
                }
            }
            impl<M: Modulo<Set = Set> $($(+$bounds)*)?> $trait<ModInt<M>> for $t {
                type Output = ModInt<M>;
                fn $fn(self, rhs: ModInt<M>) -> Self::Output {
                    <ModInt<M> as $trait>::$fn(self.into(), rhs)
                }
            }
            forward_ref_binop! { $trait, $fn for $t }
        };
    }

    impl_ops! { Add, add for Set }
    impl_ops! { Sub, sub for Set }
    impl_ops! { Mul, mul for Set }
    impl_ops! {<M: Clone> Div, div for Set }

    macro_rules! impl_assign {
        ($trait:ident, $fn:ident, $expr:ident for $($t:ty)*) => {$(
            impl<M: Modulo<Set = Set> + Clone> $trait<$t> for ModInt<M> {
                fn $fn(&mut self, rhs: $t) {
                    *self = self.clone().$expr(Self::from(rhs))
                }
            }
        )*};
    }

    impl_assign! { AddAssign, add_assign, add for ModInt<M> Set }
    impl_assign! { SubAssign, sub_assign, sub for ModInt<M> Set }
    impl_assign! { MulAssign, mul_assign, mul for ModInt<M> Set }
    impl_assign! { DivAssign, div_assign, div for ModInt<M> Set }

    impl<M1, M2> PartialEq<ModInt<M2>> for ModInt<M1> {
        fn eq(&self, other: &ModInt<M2>) -> bool {
            self.0.eq(&other.0)
        }
    }

    impl<M> Eq for ModInt<M> {}

    impl<M1, M2> PartialOrd<ModInt<M2>> for ModInt<M1> {
        fn partial_cmp(&self, other: &ModInt<M2>) -> Option<Ordering> {
            self.0.partial_cmp(&other.0)
        }
    }

    impl<M> Ord for ModInt<M> {
        fn cmp(&self, other: &Self) -> Ordering {
            self.0.cmp(&other.0)
        }
    }

    impl<M> PartialEq<Set> for ModInt<M> {
        fn eq(&self, other: &Set) -> bool {
            self.0.eq(other)
        }
    }

    impl<M> PartialOrd<Set> for ModInt<M> {
        fn partial_cmp(&self, other: &Set) -> Option<Ordering> {
            self.0.partial_cmp(other)
        }
    }

    impl<M> PartialEq<ModInt<M>> for Set {
        fn eq(&self, other: &ModInt<M>) -> bool {
            self.eq(&other.0)
        }
    }

    impl<M> PartialOrd<ModInt<M>> for Set {
        fn partial_cmp(&self, other: &ModInt<M>) -> Option<Ordering> {
            self.partial_cmp(&other.0)
        }
    }

    impl<M: Modulo<Set = Set>> Sum for ModInt<M> {
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            iter.fold(Self::zero(), |a, b| a + b)
        }
    }

    impl<'a, M: Clone + Modulo<Set = Set>> Sum<&'a ModInt<M>> for ModInt<M> {
        fn sum<I: Iterator<Item = &'a ModInt<M>>>(iter: I) -> Self {
            iter.fold(Self::zero(), |a, b| a + b)
        }
    }

    impl<M: Modulo<Set = Set>> Product for ModInt<M> {
        fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
            iter.fold(Self::one(), |a, b| a * b)
        }
    }

    impl<'a, M: Clone + Modulo<Set = Set>> Product<&'a ModInt<M>> for ModInt<M> {
        fn product<I: Iterator<Item = &'a ModInt<M>>>(iter: I) -> Self {
            iter.fold(Self::one(), |a, b| a * b)
        }
    }

    macro_rules! impl_fmt {
        (for $($trait:ident)*) => {$(
            impl<M> fmt::$trait for ModInt<M> {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    fmt::$trait::fmt(&self.0, f)
                }
            }
        )*};
    }

    impl_fmt! {for Debug Display Binary Octal LowerHex UpperHex }

    impl<M: Modulo<Set = Set>> FromStr for ModInt<M> {
        type Err = <Set as FromStr>::Err;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(s.parse::<Set>()?.into())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::ModInt;
        use super::Modulo;

        #[derive(Clone, Copy)]
        enum Modulo7 {}
        impl Modulo for Modulo7 {
            type Set = usize;
            const MOD: Self::Set = 7;
        }

        type MI = ModInt<Modulo7>;

        #[test]
        fn add() {
            let ans = vec![
                vec![0, 1, 2, 3, 4, 5, 6, 0, 1],
                vec![1, 2, 3, 4, 5, 6, 0, 1, 2],
                vec![2, 3, 4, 5, 6, 0, 1, 2, 3],
                vec![3, 4, 5, 6, 0, 1, 2, 3, 4],
                vec![4, 5, 6, 0, 1, 2, 3, 4, 5],
                vec![5, 6, 0, 1, 2, 3, 4, 5, 6],
                vec![6, 0, 1, 2, 3, 4, 5, 6, 0],
            ];
            for (i, v) in ans.into_iter().enumerate() {
                let a = MI::new(i);
                for (b, ans) in v.into_iter().enumerate() {
                    assert_eq!(a + b, ans, "{} + {}", i, b);
                }
            }
        }

        #[test]
        fn sub() {
            let ans = vec![
                vec![0, 6, 5, 4, 3, 2, 1, 0, 6],
                vec![1, 0, 6, 5, 4, 3, 2, 1, 0],
                vec![2, 1, 0, 6, 5, 4, 3, 2, 1],
                vec![3, 2, 1, 0, 6, 5, 4, 3, 2],
                vec![4, 3, 2, 1, 0, 6, 5, 4, 3],
                vec![5, 4, 3, 2, 1, 0, 6, 5, 4],
                vec![6, 5, 4, 3, 2, 1, 0, 6, 5],
            ];
            for (i, v) in ans.into_iter().enumerate() {
                let a = MI::new(i);
                for (b, ans) in v.into_iter().enumerate() {
                    assert_eq!(a - b, ans, "{} - {}", i, b);
                }
            }
        }

        #[test]
        fn neg() {
            for (i, ans) in vec![0, 6, 5, 4, 3, 2, 1, 0, 6].into_iter().enumerate() {
                let a = MI::new(i);
                assert_eq!(-a, ans, "-{}", i);
            }
        }

        #[test]
        fn mul() {
            let ans = vec![
                vec![0, 0, 0],
                vec![0, 1, 2, 3, 4, 5, 6, 0, 1],
                vec![0, 2, 4, 6, 1, 3, 5, 0, 2],
                vec![0, 3, 6, 2, 5, 1, 4, 0, 3],
                vec![0, 4, 1, 5, 2, 6, 3, 0, 4],
                vec![0, 5, 3, 1, 6, 4, 2, 0, 5],
                vec![0, 6, 5, 4, 3, 2, 1, 0, 6],
            ];
            for (i, v) in ans.into_iter().enumerate() {
                let a = MI::new(i);
                for (b, ans) in v.into_iter().enumerate() {
                    assert_eq!(a * b, ans, "{} * {}", i, b);
                }
            }
        }

        #[test]
        fn pow() {
            let ans = vec![
                vec![1, 0, 0],
                vec![1, 1, 1],
                vec![1, 2, 4, 1, 2, 4, 1, 2],
                vec![1, 3, 2, 6, 4, 5, 1, 3],
                vec![1, 4, 2, 1, 4, 2, 1, 4],
                vec![1, 5, 4, 6, 2, 3, 1, 5],
                vec![1, 6, 1, 6, 1, 6, 1, 6],
            ];
            for (i, v) in ans.into_iter().enumerate() {
                let a = MI::new(i);
                for (b, ans) in v.into_iter().enumerate() {
                    assert_eq!(a.pow(b), ans, "{}^{}", i, b);
                }
            }
        }

        #[test]
        fn recip() {
            for (i, ans) in vec![0, 1, 4, 5, 2, 3, 6, 0, 1].into_iter().enumerate() {
                let a = MI::new(i);
                assert_eq!(a.recip(), ans, "1 / {}", i);
            }
        }

        #[test]
        fn div() {
            let ans = vec![
                vec![0, 0, 0],
                vec![0, 1, 4, 5, 2, 3, 6, 0, 1],
                vec![0, 2, 1, 3, 4, 6, 5, 0, 2],
                vec![0, 3, 5, 1, 6, 2, 4, 0, 3],
                vec![0, 4, 2, 6, 1, 5, 3, 0, 4],
                vec![0, 5, 6, 4, 3, 1, 2, 0, 5],
                vec![0, 6, 3, 2, 5, 4, 1, 0, 6],
            ];
            for (i, v) in ans.into_iter().enumerate() {
                let a = MI::new(i);
                for (b, ans) in v.into_iter().enumerate() {
                    assert_eq!(a / b, ans, "{} / {}", i, b);
                }
            }
        }
    }
}
