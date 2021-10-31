use crate::math::num::{One, Zero};

#[codesnip::entry("SquareMatrix")]
pub use square_matrix_impl::SquareMatrix;

#[codesnip::entry("SquareMatrix", include("One", "Zero"))]
mod square_matrix_impl {
    use super::{One, Zero};
    use core::{
        iter::{self, FromIterator},
        ops::{Add, Index, IndexMut, Mul, Neg, Sub},
        slice::{self},
    };
    use std::{iter::Product, vec};

    /// This is square matrix.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct SquareMatrix<T> {
        size: usize,
        raw: Vec<T>,
    }

    impl<T> SquareMatrix<T> {
        pub fn new(size: usize) -> Self
        where
            T: Default,
        {
            Self::new_with(size, Default::default)
        }

        pub fn new_with<F>(size: usize, f: F) -> Self
        where
            F: FnMut() -> T,
        {
            Self {
                size,
                raw: iter::repeat_with(f).take(size * size).collect(),
            }
        }

        pub fn new_with_value(size: usize, value: T) -> Self
        where
            T: Clone,
        {
            Self::new_with(size, || value.clone())
        }

        pub const fn size(&self) -> usize {
            self.size
        }

        const fn into_index(&self, index: (usize, usize)) -> usize {
            index.0 * self.size + index.1
        }

        pub fn get(&self, index: (usize, usize)) -> Option<&T> {
            self.raw.get(self.into_index(index))
        }

        pub fn get_mut(&mut self, index: (usize, usize)) -> Option<&mut T> {
            let index = self.into_index(index);
            self.raw.get_mut(index)
        }

        pub fn iter(&self) -> slice::Iter<T> {
            self.raw.iter()
        }

        pub fn iter_mut(&mut self) -> slice::IterMut<T> {
            self.raw.iter_mut()
        }

        pub fn zeros(size: usize) -> Self
        where
            T: Zero,
        {
            Self::new_with(size, Zero::zero)
        }

        pub fn id(size: usize) -> Self
        where
            T: Zero + One,
        {
            let mut mat = Self::zeros(size);
            (0..size).for_each(|i| mat[(i, i)] = One::one());
            mat
        }

        pub fn pow(self, mut exp: usize) -> Self
        where
            T: Clone + Zero + One + Mul<Output = T> + Add<Output = T>,
        {
            if exp == 0 {
                Self::id(self.size)
            } else {
                let mut acc = Self::id(self.size);
                let mut base = self;
                while exp > 1 {
                    if exp & 1 == 1 {
                        acc = &acc * &base;
                    }
                    exp >>= 1;
                    base = &base * &base;
                }

                acc * base
            }
        }

        pub fn transpose(&mut self) {
            for row in 0..self.size() {
                for col in row + 1..self.size() {
                    let i = self.into_index((row, col));
                    let k = self.into_index((col, row));
                    self.raw.swap(i, k);
                }
            }
        }

        pub fn determinant(&self) -> T {
            todo!()
        }

        pub fn rank(&self) -> T {
            todo!()
        }

        pub fn trace(&self) -> T
        where
            T: Clone + Product,
        {
            (0..self.size()).map(|i| self[(i, i)].clone()).product()
        }

        // pub fn iter_transposed(&self) -> Transposed<'_, T> {
        //     Transposed::new(self)
        // }

        // pub fn for_each_row(&self) -> ForEachRow<T> {
        //     ForEachRow::new(self)
        // }

        // pub fn for_each_col(&self) {
        //     todo!()
        // }
    }

    impl<T> Index<(usize, usize)> for SquareMatrix<T> {
        type Output = T;

        fn index(&self, index: (usize, usize)) -> &Self::Output {
            self.raw.index(self.into_index(index))
        }
    }

    impl<T> IndexMut<(usize, usize)> for SquareMatrix<T> {
        fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
            self.raw.index_mut(self.into_index(index))
        }
    }

    impl<T: Add<Output = T>> Add for SquareMatrix<T> {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            assert!(self.size() == rhs.size());
            self.into_iter()
                .zip(rhs.into_iter())
                .map(|(a, b)| a + b)
                .collect()
        }
    }

    impl<T: Clone + Add<Output = T>> Add for &SquareMatrix<T> {
        type Output = SquareMatrix<T>;

        fn add(self, rhs: Self) -> Self::Output {
            assert!(self.size() == rhs.size());
            self.iter()
                .zip(rhs.iter())
                .map(|(a, b)| a.clone() + b.clone())
                .collect()
        }
    }

    impl<T: Neg<Output = T>> Neg for SquareMatrix<T> {
        type Output = Self;

        fn neg(self) -> Self::Output {
            self.into_iter().map(|a| -a).collect()
        }
    }

    impl<T: Clone + Neg<Output = T>> Neg for &SquareMatrix<T> {
        type Output = SquareMatrix<T>;

        fn neg(self) -> Self::Output {
            self.iter().map(|a| -a.clone()).collect()
        }
    }

    impl<T: Sub<Output = T>> Sub for SquareMatrix<T> {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            self.into_iter()
                .zip(rhs.into_iter())
                .map(|(a, b)| a - b)
                .collect()
        }
    }

    impl<T: Clone + Sub<Output = T>> Sub for &SquareMatrix<T> {
        type Output = SquareMatrix<T>;

        fn sub(self, rhs: Self) -> Self::Output {
            self.iter()
                .zip(rhs.iter())
                .map(|(a, b)| a.clone() - b.clone())
                .collect()
        }
    }

    impl<T: Clone + Zero + Mul<Output = T> + Add<Output = T>> Mul for SquareMatrix<T> {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self::Output {
            Mul::mul(&self, &rhs)
        }
    }

    impl<T: Clone + Zero + Add<Output = T> + Mul<Output = T>> Mul for &SquareMatrix<T> {
        type Output = SquareMatrix<T>;

        fn mul(self, rhs: Self) -> Self::Output {
            assert_eq!(self.size(), rhs.size());
            let mut ret = SquareMatrix::<T>::zeros(self.size);
            for row in 0..self.size {
                for d in 0..self.size {
                    for col in 0..self.size {
                        ret[(row, col)] = ret[(row, col)].clone()
                            + self[(row, d)].clone() * rhs[(d, col)].clone();
                    }
                }
            }
            ret
        }
    }

    impl<T: Clone + Mul<Output = T>> Mul<T> for SquareMatrix<T> {
        type Output = Self;

        fn mul(self, rhs: T) -> Self::Output {
            self.into_iter().map(|a| a * rhs.clone()).collect()
        }
    }

    impl<T: Clone + Mul<Output = T>> Mul<T> for &SquareMatrix<T> {
        type Output = SquareMatrix<T>;

        fn mul(self, rhs: T) -> Self::Output {
            self.iter().map(|a| a.clone() * rhs.clone()).collect()
        }
    }

    // impl<T: Mul<Output = T>> Mul<SquareMatrix<T>> for T {
    //     type Output = SquareMatrix<T>;
    //
    //     fn mul(self, rhs: SquareMatrix<T>) -> Self::Output {
    //         rhs.into_iter().map(|a| self.clone() * a).collect()
    //     }
    // }

    macro_rules! impl_matrix_mul {
        (for $($t:ty)*) => {$(
            impl Mul<SquareMatrix<$t>> for $t {
                type Output = SquareMatrix<$t>;

                fn mul(self, rhs: SquareMatrix<$t>) -> Self::Output {
                    rhs.into_iter().map(|a| self * a).collect()
                }
            }
            impl Mul<&SquareMatrix<$t>> for $t {
                type Output = SquareMatrix<$t>;

                fn mul(self, rhs: &SquareMatrix<$t>) -> Self::Output {
                    rhs.iter().map(|a| self * a).collect()
                }
            }
        )*};
    }
    pub(crate) use impl_matrix_mul;

    impl_matrix_mul! {for u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }

    impl<T> From<Vec<Vec<T>>> for SquareMatrix<T> {
        fn from(m: Vec<Vec<T>>) -> Self {
            let size = m.len();
            assert!(m.iter().all(|v| v.len() == size));
            Self {
                size,
                raw: m.into_iter().flatten().collect(),
            }
        }
    }

    impl<T> FromIterator<T> for SquareMatrix<T> {
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
            let raw = iter.into_iter().collect::<Vec<T>>();
            let size = (raw.len() as f64).sqrt();
            assert_eq!(
                size.floor(),
                size,
                "Matrix size must be square number (found {}).",
                raw.len()
            );
            let size = size as usize;
            Self { size, raw }
        }
    }

    impl<T> IntoIterator for SquareMatrix<T> {
        type Item = T;

        type IntoIter = vec::IntoIter<T>;

        fn into_iter(self) -> Self::IntoIter {
            self.raw.into_iter()
        }
    }

    // #[derive(Debug, Clone)]
    // pub struct Transposed<'a, T> {
    //     slice: &'a [T],
    //     size: usize,
    //     i: usize,
    // }

    // impl<'a, T> Transposed<'a, T> {
    //     pub fn new(m: &'a SquareMatrix<T>) -> Self {
    //         Self {
    //             slice: m.raw.as_slice(),
    //             size: m.size(),
    //             i: 0,
    //         }
    //     }
    // }

    // impl<'a, T> Iterator for Transposed<'a, T> {
    //     type Item = &'a T;

    //     fn next(&mut self) -> Option<Self::Item> {
    //         if self.i < self.slice.len() {
    //             let index = self.i % self.size + self.i / self.size;
    //             self.slice.get(index)
    //         } else {
    //             None
    //         }
    //     }
    // }

    // #[derive(Debug, Clone)]
    // pub struct ForEachRow<'a, T> {
    //     chunks: slice::Chunks<'a, T>,
    // }

    // impl<'a, T> ForEachRow<'a, T> {
    //     pub(super) fn new(matrix: &'a SquareMatrix<T>) -> Self {
    //         let slice = &*matrix.raw;
    //         let chunks = slice.chunks(matrix.size());
    //         Self { chunks }
    //     }
    // }

    // impl<'a, T> Iterator for ForEachRow<'a, T> {
    //     type Item = <slice::Chunks<'a, T> as Iterator>::Item;
    //     #[inline]
    //     fn next(&mut self) -> Option<Self::Item> {
    //         self.chunks.next()
    //     }

    //     #[inline]
    //     fn size_hint(&self) -> (usize, Option<usize>) {
    //         self.chunks.size_hint()
    //     }

    //     #[inline]
    //     fn count(self) -> usize {
    //         self.chunks.count()
    //     }

    //     #[inline]
    //     fn nth(&mut self, n: usize) -> Option<Self::Item> {
    //         self.chunks.nth(n)
    //     }

    //     #[inline]
    //     fn last(self) -> Option<Self::Item> {
    //         self.chunks.last()
    //     }
    // }

    // impl<T> DoubleEndedIterator for ForEachRow<'_, T> {
    //     #[inline]
    //     fn next_back(&mut self) -> Option<Self::Item> {
    //         self.chunks.next_back()
    //     }

    //     #[inline]
    //     fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
    //         self.chunks.nth_back(n)
    //     }
    // }

    // impl<T> ExactSizeIterator for ForEachRow<'_, T> {}
    // impl<T> FusedIterator for ForEachRow<'_, T> {}

    // pub struct ForEachColumn {}

    // impl Iterator for ForEachColumn {
    //     type Item = ();

    //     fn next(&mut self) -> Option<Self::Item> {
    //         todo!()
    //     }
    // }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn to_mat<T>(v: Vec<T>) -> SquareMatrix<T> {
            v.into_iter().collect()
        }

        #[test]
        fn add() {
            let a = to_mat(vec![1, 2, 3, 4]);
            let b = to_mat(vec![5, 6, 7, 8]);
            let ans = to_mat(vec![6, 8, 10, 12]);
            assert_eq!(a + b, ans);
        }

        #[test]
        fn pow() {
            let a = to_mat::<usize>((1..=9).collect());
            // julia:
            // for i=0:15 t^i |> transpose |> vec |> (v->println("vec!$v,")) end
            let ans = vec![
                vec![1, 0, 0, 0, 1, 0, 0, 0, 1],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![30, 36, 42, 66, 81, 96, 102, 126, 150],
                vec![468, 576, 684, 1062, 1305, 1548, 1656, 2034, 2412],
                vec![7560, 9288, 11016, 17118, 21033, 24948, 26676, 32778, 38880],
                vec![
                    121824, 149688, 177552, 275886, 338985, 402084, 429948, 528282, 626616,
                ],
                vec![
                    1963440, 2412504, 2861568, 4446414, 5463369, 6480324, 6929388, 8514234,
                    10099080,
                ],
                vec![
                    31644432, 38881944, 46119456, 71662158, 88052265, 104442372, 111679884,
                    137222586, 162765288,
                ],
                vec![
                    510008400, 626654232, 743300064, 1154967822, 1419124617, 1683281412,
                    1799927244, 2211595002, 2623262760,
                ],
                vec![
                    8219725776,
                    10099688472,
                    11979651168,
                    18614436174,
                    22871810025,
                    27129183876,
                    29009146572,
                    35643931578,
                    42278716584,
                ],
                vec![
                    132476037840,
                    162775103256,
                    193074168672,
                    300005963406,
                    368621393481,
                    437236823556,
                    467535888972,
                    574467683706,
                    681399478440,
                ],
                vec![
                    2135095631568,
                    2623420941336,
                    3111746251104,
                    4835149302222,
                    5941013482665,
                    7046877663108,
                    7535202972876,
                    9258606023994,
                    10982009075112,
                ],
                vec![
                    34411003154640,
                    42281265978648,
                    50151528802656,
                    77927346874638,
                    95750387322633,
                    113573427770628,
                    121443690594636,
                    149219508666618,
                    176995326738600,
                ],
                vec![
                    554596768687824,
                    681440566623768,
                    808284364559712,
                    1255942890559566,
                    1543194052527465,
                    1830445214495364,
                    1957289012431308,
                    2404947538431162,
                    2852606064431016,
                ],
                vec![
                    8938349587100880,
                    10982671286972184,
                    13026992986843488,
                    20241835602136974,
                    24871417759719369,
                    29500999917301764,
                    31545321617173068,
                    38760164232466554,
                    45975006847760040,
                ],
                vec![
                    144057985642894032,
                    177005999503810584,
                    209954013364727136,
                    326234506062126798,
                    400848759341284905,
                    475463012620443012,
                    508411026481359564,
                    624691519178759226,
                    740972011876158888,
                ],
            ];
            for (i, ans) in ans.into_iter().enumerate() {
                let ans = to_mat(ans);
                assert_eq!(a.clone().pow(i), ans, "i={}", i);
            }
        }

        #[test]
        fn pow_modint() {
            use crate::math::modint::ModInt1e9_7;
            let a = to_mat::<ModInt1e9_7>((1..=9).map(|x| x.into()).collect());
            let ans = vec![
                vec![1, 0, 0, 0, 1, 0, 0, 0, 1],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![30, 36, 42, 66, 81, 96, 102, 126, 150],
                vec![468, 576, 684, 1062, 1305, 1548, 1656, 2034, 2412],
                vec![7560, 9288, 11016, 17118, 21033, 24948, 26676, 32778, 38880],
                vec![
                    121824, 149688, 177552, 275886, 338985, 402084, 429948, 528282, 626616,
                ],
                vec![
                    1963440, 2412504, 2861568, 4446414, 5463369, 6480324, 6929388, 8514234,
                    10099080,
                ],
                vec![
                    31644432, 38881944, 46119456, 71662158, 88052265, 104442372, 111679884,
                    137222586, 162765288,
                ],
                vec![
                    510008400, 626654232, 743300064, 154967815, 419124610, 683281405, 799927237,
                    211594988, 623262746,
                ],
                vec![
                    219725720, 99688402, 979651091, 614436048, 871809871, 129183687, 9146369,
                    643931333, 278716290,
                ],
                vec![
                    476036916, 775102122, 74167321, 5961306, 621390905, 236820497, 535885703,
                    467679688, 399473673,
                ],
                vec![
                    95616623, 420922975, 746229327, 149268377, 13441078, 877613786, 202920131,
                    605959188, 8998238,
                ],
                vec![
                    2913763, 265682681, 528451599, 346329149, 386652383, 426975617, 689744535,
                    507622085, 325499635,
                ],
                vec![
                    764805652, 561853688, 358901724, 881767972, 41725107, 201682249, 998730292,
                    521596533, 44462774,
                ],
                vec![
                    524532437, 210093487, 895654544, 460444129, 585619450, 710794771, 396355821,
                    961145413, 525934998,
                ],
                vec![
                    634488144, 264768598, 895049059, 778485277, 535343613, 292201949, 922482410,
                    805918628, 689354846,
                ],
            ];
            for (i, ans) in ans.into_iter().enumerate() {
                let ans = to_mat(ans.into_iter().map(|x| x.into()).collect());
                assert_eq!(a.clone().pow(i), ans, "i={}", i);
            }
        }
    }
}
