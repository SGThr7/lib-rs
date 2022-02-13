pub trait Recip {
    fn recip(self) -> Self;
}

macro_rules! impl_recip {
    ($($t:tt)*) => {$(
        impl Recip for $t {
            fn recip(self) -> Self {
                $t::recip(self)
            }
        }
    )*};
}

impl_recip! { f32 f64 }
