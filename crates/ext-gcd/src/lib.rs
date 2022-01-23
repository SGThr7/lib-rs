/// It represent that extended Euclidean algorithm results.
///
/// # Examples
///
/// ```
/// use ext_gcd::{ExtGcd, ExtGcdResult};
///
/// let a = 13;
/// let b = 5;
/// let ExtGcdResult { x, y, gcd } = a.ext_gcd(b);
///
/// assert_eq!(x, 2);
/// assert_eq!(y, -5);
/// assert_eq!(gcd, 1);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct ExtGcdResult<T> {
    pub x: T,
    pub y: T,
    pub gcd: T,
}

/// The trait to implement [Extended Euclidean Algorithm](https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm).
pub trait ExtGcd: Sized {
    /// The resulting type
    type Output;

    /// Calculate extended euclidean algorithm.
    ///
    /// # Examples
    ///
    /// ```
    /// use ext_gcd::{ExtGcd, ExtGcdResult};
    ///
    /// let a = 13;
    /// let b = 5;
    /// let ExtGcdResult { x, y, gcd } = a.ext_gcd(b);
    ///
    /// assert_eq!(a * x + b * y, gcd);
    /// ```
    fn ext_gcd(self, b: Self) -> Self::Output;
}

macro_rules! impl_ext_gcd {
    ($($t:ty)*) => {$(
        impl ExtGcd for $t {
            type Output = ExtGcdResult<$t>;

            fn ext_gcd(self, b: $t) -> Self::Output {
                let mut x = vec![self, 1, 0];
                let mut y = vec![b, 0, 1];
                loop {
                    std::mem::swap(&mut x, &mut y);
                    if y[0] == 0 {
                        break;
                    }
                    let div = x[0].div_euclid(y[0]);
                    x = x.iter().zip(y.iter()).map(|(a, b)| a - div * b).collect();
                }

                ExtGcdResult {
                    gcd: x[0],
                    x: x[1],
                    y: x[2],
                }
            }
        }
    )*};
}

impl_ext_gcd! { i8 i16 i32 i64 isize }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ext_gcd() {
        let a = 57;
        let b = 1729;

        let ExtGcdResult { x, y, gcd } = a.ext_gcd(b);
        assert_eq!(a * x + b * y, gcd);

        let ExtGcdResult { x, y, gcd } = b.ext_gcd(a);
        assert_eq!(b * x + a * y, gcd);
    }
}
