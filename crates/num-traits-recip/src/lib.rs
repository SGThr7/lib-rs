/// The trait that implements reciprocal.
///
/// # Examples
///
/// ```
/// use num_traits_recip::Recip;
///
/// let recip = 2.0.recip();
///
/// assert_eq!(recip, 0.5);
/// assert_eq!(recip * 2.0, 1.0);
/// ```
pub trait Recip {
    type Output;

    fn recip(self) -> Self::Output;
}

macro_rules! impl_recip {
    ($($t:tt)*) => {$(
        impl Recip for $t {
            type Output = $t;

            #[inline]
            fn recip(self) -> Self {
                $t::recip(self)
            }
        }
    )*};
}

impl_recip! { f32 f64 }
