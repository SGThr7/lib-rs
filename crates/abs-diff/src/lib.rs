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
