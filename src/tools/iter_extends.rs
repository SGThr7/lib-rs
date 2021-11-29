#[codesnip::entry("IterExtends")]
pub use iter_extends_impl::IterExtends;

#[codesnip::entry("IterExtends")]
mod iter_extends_impl {
    use core::{
        hash::Hash,
        iter::{FromIterator, FusedIterator},
    };
    use std::collections::HashMap;

    pub trait IterExtends: Iterator {
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
    }

    impl<I: Iterator> IterExtends for I {}

    pub struct CustomMap<I, F> {
        iter: I,
        f: F,
    }

    impl<I: Iterator, F: CustomMapFn<I::Item>> Iterator for CustomMap<I, F> {
        type Item = F::Output;

        fn next(&mut self) -> Option<Self::Item> {
            self.iter.next().map(|t| self.f.call(t))
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

    pub trait CustomMapFn<T> {
        type Output;
        fn call(&mut self, t: T) -> Self::Output;
    }

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

    type ToDigit<I> = CustomMap<I, CustomMapFnToDigit>;

    pub fn to_digit<I>(iter: I, radix: u32) -> ToDigit<I> {
        ToDigit {
            iter,
            f: CustomMapFnToDigit { radix },
        }
    }

    pub struct CustomMapFnAsUsize;
    pub struct CustomMapFnAsIsize;

    macro_rules! as_impl {
        ($struct:ident, $type:ty :$($t:ty)*) => {$(
            impl CustomMapFn<$t> for $struct {
                type Output = $type;

                fn call(&mut self, t: $t) -> Self::Output {
                    t as $type
                }
            }
        )*};
    }

    as_impl! { CustomMapFnAsUsize, usize: u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64 }
    as_impl! { CustomMapFnAsIsize, isize: u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64 }

    type AsUsize<I> = CustomMap<I, CustomMapFnAsUsize>;
    type AsIsize<I> = CustomMap<I, CustomMapFnAsIsize>;

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
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::IterExtends;

    #[test]
    fn to_digit() {
        let v = "1234567890".chars().to_digit(10).collect::<Vec<_>>();
        assert_eq!(v, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);
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
}
