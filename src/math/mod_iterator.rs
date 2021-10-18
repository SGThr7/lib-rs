#[codesnip::entry("ModIterator")]
pub trait ModIterator: Iterator {
    fn mod_sum<S>(self, modulo: Self::Item) -> S
    where
        Self: Sized,
        S: ModSum<Self::Item>,
    {
        ModSum::mod_sum(self, modulo)
    }
    fn mod_product<P>(self, modulo: Self::Item) -> P
    where
        Self: Sized,
        P: ModProduct<Self::Item>,
    {
        ModProduct::mod_product(self, modulo)
    }
}

#[codesnip::entry("ModIterator")]
pub trait ModSum<A = Self>: Sized {
    fn mod_sum<I: Iterator<Item = A>>(iter: I, modulo: A) -> Self;
}
#[codesnip::entry("ModIterator")]
pub trait ModProduct<A = Self>: Sized {
    fn mod_product<I: Iterator<Item = A>>(iter: I, modulo: A) -> Self;
}

#[codesnip::entry("ModIterator")]
mod mod_iterator_impl {
    use super::{ModIterator, ModProduct, ModSum};

    impl<I: ?Sized + Iterator> ModIterator for I {}

    #[macro_export]
    macro_rules! mod_sum_product {
        (@impls $zero:expr, $one:expr, $($a:ty)*) => ($(
            impl ModSum for $a {
                fn mod_sum<I: Iterator<Item = Self>>(iter: I, modulo: $a) -> Self {
                    iter.fold($zero, |a, b| (a + b) % modulo)
                }
            }
            impl ModProduct for $a {
                fn mod_product<I: Iterator<Item = Self>>(iter: I, modulo: $a) -> Self {
                    iter.fold($one, |a, b| (a * b) % modulo)
                }
            }
        )*);
        (int: $($a:ty)*) => (
            mod_sum_product!(@impls 0, 1, $($a)*);
            mod_sum_product!(@impls core::num::Wrapping(0), core::num::Wrapping(1), $(core::num::Wrapping<$a>)*);
        );
        (float: $($a:ty)*) => (
            mod_sum_product!(@impls 0.0, 1.0, $($a)*);
        );
    }

    mod_sum_product!(int: i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);
    mod_sum_product!(float: f32 f64);
}
