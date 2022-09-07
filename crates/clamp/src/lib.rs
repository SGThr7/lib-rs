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
    ///
    /// ```
    /// use clamp::Clamp;
    ///
    /// assert_eq!((-3).clamp1(-2, 1), -2);
    /// assert_eq!(0.clamp1(-2, 1), 0);
    /// assert_eq!(2.clamp1(-2, 1), 1);
    /// ```
    ///
    /// If `self` is in between `min` and `max`,
    /// it returns `self` exactly.
    ///
    /// ```
    /// use clamp::Clamp;
    /// # use std::cmp::Ordering;
    ///
    /// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    /// struct Hoge(isize, isize);
    ///
    /// impl PartialOrd for Hoge {
    ///     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    ///         Some(self.cmp(&other))
    ///     }
    /// }
    ///
    /// impl Ord for Hoge {
    ///     fn cmp(&self, other: &Self) -> Ordering {
    ///         self.0.cmp(&other.0)
    ///     }
    /// }
    ///
    /// let a = Hoge(0, 0);
    /// let b = Hoge(0, 1);
    /// let c = Hoge(0, -1);
    ///
    /// assert_eq!(a.clamp1(b, c), a);
    /// assert_eq!(a.clamp1(c, b), a);
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
