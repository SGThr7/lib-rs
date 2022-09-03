/// The trait to implement [`abs_diff`].
///
/// [`abs_diff`]: usize::abs_diff
pub trait AbsDiff<Rhs = Self> {
    /// The resulting type.
    type Output;

    /// Compute the difference of two values.
    ///
    /// This function is equivalent to [`abs_diff`].
    ///
    /// [`abs_diff`]: usize::abs_diff
    ///
    /// # Examples
    ///
    /// ```
    /// use abs_diff::AbsDiff;
    ///
    /// assert_eq!(10.abs_diff1(20), 10);
    /// assert_eq!(20.abs_diff1(10), 10);
    /// assert_eq!(100.abs_diff1(20), 80);
    /// ```
    fn abs_diff1(self, other: Self) -> Self;
}

macro_rules! impl_abs_diff {
    ($($t:ty)*) => ($(
        impl AbsDiff for $t {
            type Output = Self;

            fn abs_diff1(self, other: Self) -> Self {
                if self >= other {
                    self - other
                } else {
                    other - self
                }
            }
        }
    )*)
}

impl_abs_diff! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64 }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounds() {
        let max = isize::MAX;
        let zero = 0;
        let min = isize::MIN;

        assert_eq!(max.abs_diff1(zero), max);
        assert_eq!(zero.abs_diff1(max), max);

        assert_eq!((min + 1).abs_diff1(zero), max);
        assert_eq!(zero.abs_diff1(min + 1), max);

        assert_eq!(min.abs_diff1(zero - 1), max);
        assert_eq!((zero - 1).abs_diff1(min), max);
    }

    #[test]
    fn plus() {
        assert_eq!(57.abs_diff1(7), 50);
        assert_eq!(7.abs_diff1(57), 50);
    }

    #[test]
    fn minus() {
        assert_eq!((-57).abs_diff1(-7), 50);
        assert_eq!((-7).abs_diff1(-57), 50);
    }

    #[test]
    fn plus_minus() {
        assert_eq!(10.abs_diff1(-10), 20);
        assert_eq!((-10).abs_diff1(10), 20);
    }
}
