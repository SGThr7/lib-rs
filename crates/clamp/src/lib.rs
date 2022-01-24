/// The trait to implement clamp.
pub trait Clamp {
    /// Restrict a value to a certain interval.
    ///
    /// Returns `max` if `self` is greater than `max`, and `min` if `self` is less than `min`.
    /// Otherwise this returns `self`.
    ///
    /// This function is equivalent to [`Ord::clamp`].
    ///
    /// # Panics
    ///
    /// Panics if `min > max`.
    ///
    /// # Examples
    /// ```
    /// use clamp::Clamp;
    ///
    /// assert_eq!((-3).clamp(-2, 1), -2);
    /// assert_eq!(0.clamp(-2, 1), 0);
    /// assert_eq!(2.clamp(-2, 1), 1);
    /// ```
    fn clamp1(self, min: Self, max: Self) -> Self;
}

impl<T: Ord> Clamp for T {
    fn clamp1(self, min: Self, max: Self) -> Self {
        assert!(min <= max);
        if self < min {
            min
        } else if self > max {
            max
        } else {
            self
        }
    }
}
