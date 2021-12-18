use core::fmt::Debug;
use core::hash::Hash;
use core::iter::{FromIterator, FusedIterator, Inspect};
use core::num::Wrapping;
use std::collections::HashMap;

pub trait IteratorEx: Iterator {
    fn mod_sum<S, M>(self, modulo: M) -> S
    where
        Self: Sized,
        S: ModSum<Self::Item, M>,
    {
        ModSum::mod_sum(self, modulo)
    }

    fn mod_product<S, M>(self, modulo: M) -> S
    where
        Self: Sized,
        S: ModProduct<Self::Item, M>,
    {
        ModProduct::mod_product(self, modulo)
    }

    fn collect_string(self) -> String
    where
        Self: Sized,
        String: FromIterator<Self::Item>,
    {
        self.collect()
    }

    fn counts(self) -> HashMap<Self::Item, usize>
    where
        Self: Sized,
        Self::Item: Eq + Hash,
    {
        self.fold(HashMap::new(), |mut map, t| {
            *map.entry(t).or_default() += 1;
            map
        })
    }

    fn to_digit(self, radix: u32) -> ToDigit<Self>
    where
        Self: Sized + Iterator<Item = char>,
    {
        to_digit(self, radix)
    }

    fn as_usize(self) -> AsUsize<Self>
    where
        Self: Sized,
    {
        as_usize(self)
    }

    fn as_isize(self) -> AsIsize<Self>
    where
        Self: Sized,
    {
        as_isize(self)
    }

    /// Output iterator values.
    /// It is equivalent to `iter.inspect(|v| { dbg!(v); })`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use lib_rust::extends::iterator::IteratorEx;
    /// let v = vec![1, 2, 3];
    ///
    /// let sum = v.iter()
    ///     .map(|x| x * 2)
    /// // Outputs `2, 4, 6`.
    ///     .dbg()
    ///     .sum::<usize>();
    /// ```
    fn dbg(self) -> Inspect<Self, fn(&Self::Item)>
    where
        Self: Sized,
        Self::Item: Debug,
    {
        self.inspect(|v| {
            dbg!(v);
        })
    }
}

impl<I: ?Sized + Iterator> IteratorEx for I {}

pub trait ModSum<A = Self, M = Self>: Sized {
    fn mod_sum<I: Iterator<Item = A>>(iter: I, modulo: M) -> Self;
}

pub trait ModProduct<A = Self, M = Self>: Sized {
    fn mod_product<I: Iterator<Item = A>>(iter: I, modulo: M) -> Self;
}

macro_rules! mod_sum_product {
    ($zero:expr, $one:expr, $t:ty) => {
        impl ModSum for $t {
            fn mod_sum<I: Iterator<Item = Self>>(iter: I, modulo: Self) -> Self {
                iter.fold($zero, |a, b| (a + b) % modulo)
            }
        }

        impl<'a> ModSum<&'a $t> for $t {
            fn mod_sum<I: Iterator<Item = &'a Self>>(iter: I, modulo: Self) -> Self {
                iter.fold($zero, |a, b| (a + b) % modulo)
            }
        }

        impl ModProduct for $t {
            fn mod_product<I: Iterator<Item = Self>>(iter: I, modulo: Self) -> Self {
                iter.fold($one, |a, b| (a * b) % modulo)
            }
        }

        impl<'a> ModProduct<&'a $t> for $t {
            fn mod_product<I: Iterator<Item = &'a Self>>(iter: I, modulo: Self) -> Self {
                iter.fold($one, |a, b| (a * b) % modulo)
            }
        }
    };
}

macro_rules! int_mod_sum_product {
    ($($t:ty)*) => {$(
        mod_sum_product! { 0, 1, $t }
        mod_sum_product! { Wrapping(0), Wrapping(1), Wrapping<$t> }
    )*};
}

macro_rules! float_mod_sum_product {
    ($($t:ty)*) => {$(
        mod_sum_product! { 0.0, 1.1, $t }
    )*};
}

int_mod_sum_product! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }
float_mod_sum_product! { f32 f64 }

pub struct CustomMap<I, F> {
    iter: I,
    f: F,
}

pub trait CustomMapFn<T> {
    type Output;
    fn call(&mut self, t: T) -> Self::Output;
}

impl<I: Iterator, F: CustomMapFn<I::Item>> Iterator for CustomMap<I, F> {
    type Item = F::Output;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|t| self.f.call(t))
    }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.iter.count()
    }
}

impl<I: DoubleEndedIterator, F: CustomMapFn<I::Item>> DoubleEndedIterator for CustomMap<I, F> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|t| self.f.call(t))
    }
}

impl<I: ExactSizeIterator, F: CustomMapFn<I::Item>> ExactSizeIterator for CustomMap<I, F> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<I: FusedIterator, F: CustomMapFn<I::Item>> FusedIterator for CustomMap<I, F> {}

pub struct CustomMapFnToDigit {
    radix: u32,
}

impl CustomMapFn<char> for CustomMapFnToDigit {
    type Output = u32;

    fn call(&mut self, t: char) -> Self::Output {
        t.to_digit(self.radix)
            .unwrap_or_else(|| panic!("failed to convert to digit from `{}`", t))
    }
}

pub type ToDigit<I> = CustomMap<I, CustomMapFnToDigit>;

pub fn to_digit<I>(iter: I, radix: u32) -> ToDigit<I> {
    ToDigit {
        iter,
        f: CustomMapFnToDigit { radix },
    }
}

pub struct CustomMapFnAsUsize;
pub struct CustomMapFnAsIsize;

macro_rules! as_iter {
    ($fn:ident, $to:ty: $($t:ty)*) => {$(
        impl CustomMapFn<$t> for $fn {
            type Output = $to;

            fn call(&mut self, t: $t) -> Self::Output { t as $to }
        }
    )*};
}

as_iter! { CustomMapFnAsUsize, usize: u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64 }
as_iter! { CustomMapFnAsIsize, isize: u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64 }

pub type AsUsize<I> = CustomMap<I, CustomMapFnAsUsize>;
pub type AsIsize<I> = CustomMap<I, CustomMapFnAsIsize>;

pub fn as_usize<I>(iter: I) -> AsUsize<I> {
    AsUsize {
        iter,
        f: CustomMapFnAsUsize,
    }
}

pub fn as_isize<I>(iter: I) -> AsIsize<I> {
    AsIsize {
        iter,
        f: CustomMapFnAsIsize,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mod_sum() {
        let sum = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
            .iter()
            .mod_sum::<usize, _>(13);
        assert_eq!(sum, 3);
    }

    #[test]
    fn mod_product() {
        let prod = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
            .iter()
            .mod_product::<usize, _>(13);
        assert_eq!(prod, 6);
    }

    #[test]
    fn to_digit() {
        let v = "1234567890".chars().to_digit(10).collect::<Vec<_>>();
        assert_eq!(v, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);
    }

    #[test]
    fn collect_string() {
        let s = "test_string".to_string();
        let v = s.chars().collect_string();
        assert_eq!(v, s);
    }

    #[test]
    fn counts() {
        let v = vec![2, 1, 4, 7, 4, 8, 3, 6, 4, 7];
        let counts = v.into_iter().counts();
        let ans = vec![(1, 1), (2, 1), (3, 1), (4, 3), (6, 1), (7, 2), (8, 1)]
            .into_iter()
            .collect::<HashMap<_, _>>();
        assert_eq!(counts, ans);
    }

    #[test]
    fn as_usize() {
        let v = vec![1., 2., 3., 4., 5., 6., 7., 8., 9., 0.]
            .into_iter()
            .as_usize()
            .collect::<Vec<_>>();
        assert_eq!(v, vec![1usize, 2, 3, 4, 5, 6, 7, 8, 9, 0]);
    }

    #[test]
    fn as_isize() {
        let v = vec![-1., -2., -3., -4., -5., -6., -7., -8., -9., -0.]
            .into_iter()
            .as_isize()
            .collect::<Vec<_>>();
        assert_eq!(v, vec![-1isize, -2, -3, -4, -5, -6, -7, -8, -9, -0]);
    }
}
