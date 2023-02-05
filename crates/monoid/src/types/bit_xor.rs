use std::ops::BitXor;

use num_traits_zero::Zero;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitXorAlge<T>(PhantomData<T>);

impl<T> Semigroup for BitXorAlge<T>
where
    for<'a> &'a T: BitXor<Output = T>,
{
    type Set = T;

    fn operate(lhs: &Self::Set, rhs: &Self::Set) -> Self::Set {
        lhs ^ rhs
    }
}

impl<T> Monoid for BitXorAlge<T>
where
    T: Zero,
    for<'a> &'a T: BitXor<Output = T>,
{
    fn id() -> Self::Set {
        T::ZERO
    }
}

impl<T> PartialGroup for BitXorAlge<T>
where
    T: Zero,
    for<'a> &'a T: BitXor<Output = T>,
{
    fn inverse_operate(lhs: &Self::Set, rhs: &Self::Set) -> Self::Set {
        lhs ^ rhs
    }
}

impl<T> MathGroup for BitXorAlge<T>
where
    T: Zero + Clone,
    for<'a> &'a T: BitXor<Output = T>,
{
    fn inverse(x: &Self::Set) -> Self::Set {
        x.clone()
    }
}
