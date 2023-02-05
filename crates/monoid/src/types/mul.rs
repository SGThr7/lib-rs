use super::*;
use num_traits_one::One;
use num_traits_recip::Recip;
use std::ops::{Div, Mul};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MulAlge<T>(PhantomData<T>);

impl<T> Semigroup for MulAlge<T>
where
    for<'a> &'a T: Mul<Output = T>,
{
    type Set = T;

    fn operate(lhs: &Self::Set, rhs: &Self::Set) -> Self::Set {
        lhs * rhs
    }
}

impl<T> Monoid for MulAlge<T>
where
    T: One,
    for<'a> &'a T: Mul<Output = T>,
{
    fn id() -> Self::Set {
        T::ONE
    }
}

impl<T> PartialGroup for MulAlge<T>
where
    T: One,
    for<'a> &'a T: Mul<Output = T> + Div<Output = T>,
{
    fn inverse_operate(lhs: &Self::Set, rhs: &Self::Set) -> Self::Set {
        lhs / rhs
    }
}

impl<T> MathGroup for MulAlge<T>
where
    T: One,
    for<'a> &'a T: Mul<Output = T> + Div<Output = T> + Recip<Output = T>,
{
    fn inverse(x: &Self::Set) -> Self::Set {
        x.recip()
    }
}
