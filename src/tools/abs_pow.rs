use crate::math::num::One;

#[codesnip::entry("AbstractPow", include("One"))]
pub fn abstract_pow<T: Clone + core::ops::Mul<Output = T> + One>(mut base: T, mut exp: u32) -> T {
    if exp == 0 {
        T::one()
    } else {
        let mut acc = T::one();
        while exp > 1 {
            if exp & 1 == 1 {
                acc = acc * base.clone();
            }
            exp >>= 1;
            base = base.clone() * base;
        }

        acc * base
    }
}
