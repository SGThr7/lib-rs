#[codesnip::entry(inline, "Reciprocal")]
mod reciprocal {
    pub trait Reciprocal {
        type Output;
        fn recip(self) -> Self::Output;
    }

    macro_rules! impl_reciprocal {
        ($one:expr, for $($t:ty)*) => {$(
            impl Reciprocal for $t {
                type Output = $t;
                fn recip(self) -> Self::Output {
                    $one / self
                }
            }
            impl<'a> Reciprocal for &'a $t {
                type Output = $t;
                fn recip(self) -> Self::Output {
                    Reciprocal::recip(*self)
                }
            }
        )*};
    }

    impl_reciprocal! { 1.0, for f32 f64 }
}
pub use reciprocal::*;
