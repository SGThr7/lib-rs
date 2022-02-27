use macro_forward_ref_binop::forward_ref_binop;

/// The trait to implement [Greatest Common Divisor (GCD)](https://en.wikipedia.org/wiki/Greatest_common_divisor).
pub trait Gcd<Rhs = Self> {
    /// The resulting type
    type Output;

    /// Calculates the GCD.
    ///
    /// The result is always a positive number.
    ///
    /// # Examples
    ///
    /// ```
    /// use gcd_lcm::Gcd;
    ///
    /// let a = 4;
    /// let b = 6;
    ///
    /// assert_eq!(a.gcd(b), 2);
    /// assert_eq!(b.gcd(a), 2);
    /// assert_eq!((-a).gcd(b), 2);
    /// assert_eq!(a.gcd(-b), 2);
    /// assert_eq!((-a).gcd(-b), 2);
    ///
    /// assert_eq!(0.gcd(0), 0);
    /// ```
    fn gcd(self, rhs: Rhs) -> Self::Output;
}

/// The trait to implement [Least Common Multiple (LCM)](https://en.wikipedia.org/wiki/Least_common_multiple).
pub trait Lcm<Rhs = Self> {
    /// The resulting type.
    type Output;

    /// Calculates the LCM.
    ///
    /// The result is always a positive number.
    ///
    /// # Examples
    ///
    /// ```
    /// use gcd_lcm::Lcm;
    ///
    /// let a = 4;
    /// let b = 6;
    ///
    /// assert_eq!(a.lcm(b), 12);
    /// assert_eq!(b.lcm(a), 12);
    /// assert_eq!((-a).lcm(b), 12);
    /// assert_eq!(a.lcm(-b), 12);
    /// assert_eq!((-a).lcm(-b), 12);
    ///
    /// assert_eq!(0.lcm(0), 0);
    /// ```
    fn lcm(self, rhs: Rhs) -> Self::Output;
}

/// The convenient trait to get [`Gcd`] and [`Lcm`] together.
pub trait GcdLcm<Rhs = Self> {
    /// The resulting gcd and lcm.
    type Output;

    /// Calculates the [`Gcd`] and [`Lcm`] together.
    ///
    /// # Examples
    ///
    /// ```
    /// use gcd_lcm::GcdLcm;
    ///
    /// let (gcd, lcm) = 4.gcd_lcm(6);
    ///
    /// assert_eq!(gcd, 2);
    /// assert_eq!(lcm, 12);
    /// ```
    fn gcd_lcm(self, rhs: Rhs) -> Self::Output;
}

macro_rules! impl_gcd {
    ($($t:ty)*) => {$(
        impl Gcd for $t {
            type Output = $t;

            fn gcd(self, rhs: $t) -> $t {
                let (mut min, mut max) = if self <= rhs {
                    (self, rhs)
                } else {
                    (rhs, self)
                };
                while min != 0 {
                    let rem = max.rem_euclid(min);
                    max = min;
                    min = rem;
                }
                max
            }
        }

        forward_ref_binop! { impl Gcd, gcd for $t, $t }

        impl Lcm for $t {
            type Output = $t;

            fn lcm(self, rhs: $t) -> $t {
                self.gcd_lcm(rhs).1
            }
        }

        forward_ref_binop! { impl Lcm, lcm for $t, $t }

        forward_ref_binop! { impl GcdLcm, gcd_lcm for $t, $t }
    )*};
}

impl_gcd! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }

macro_rules! impl_gcd_lcm_uint {
    ($($t:ty)*) => {$(
        impl GcdLcm for $t {
            type Output = ($t, $t);

            fn gcd_lcm(self, rhs: $t) -> ($t, $t) {
                if self == 0 && rhs == 0 {
                    return (0, 0);
                }

                let gcd = self.gcd(rhs);
                let lcm = self / gcd * rhs;
                (gcd, lcm)
            }
        }
    )*};
}

macro_rules! impl_gcd_lcm_int {
    ($($t:ty)*) => {$(
        impl GcdLcm for $t {
            type Output = ($t, $t);

            fn gcd_lcm(self, rhs: $t) -> ($t, $t) {
                if self == 0 && rhs == 0 {
                    return (0, 0);
                }

                let gcd = self.gcd(rhs);
                let lcm = self.abs() / gcd * rhs.abs();
                (gcd, lcm)
            }
        }
    )*};
}

impl_gcd_lcm_uint! { u8 u16 u32 u64 u128 usize }
impl_gcd_lcm_int! { i8 i16 i32 i64 i128 isize }

/// Calculates the [`Gcd`] of multiple numbers.
///
/// # Examples
///
/// ```
/// use gcd_lcm::gcd;
///
/// assert_eq!(gcd!(6, 9, 21), 3);
/// ```
#[macro_export]
macro_rules! gcd {
    () => {
        0
    };
    ($a:expr $(,)?) => {
        $a
    };
    ($a:expr, $b:expr $(,)?) => {
        $crate::Gcd::gcd($a, $b)
    };
    ($a1:expr, $($an:expr),* $(,)?) => {
        $crate::Gcd::gcd($a1, $crate::gcd!( $($an),* ))
    }
}

/// Calculates the [`Lcm`] of multiple numbers.
///
/// # Examples
///
/// ```
/// use gcd_lcm::lcm;
///
/// assert_eq!(lcm!(6, 9, 21), 126);
/// ```
#[macro_export]
macro_rules! lcm {
    () => {
        0
    };
    ($a:expr $(,)?) => {
        $a
    };
    ($a:expr, $b:expr $(,)?) => {
        $crate::Lcm::lcm($a, $b)
    };
    ($a1:expr, $($an:expr),* $(,)?) => {
        $crate::Lcm::lcm($a1, $crate::lcm!( $($an),* ))
    }
}
