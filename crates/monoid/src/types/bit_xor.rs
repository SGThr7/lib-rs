use std::ops::BitXor;

use num_traits_zero::Zero;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitXorAlge<T>(PhantomData<T>);

impl<T> Semigroup for BitXorAlge<T>
where
    T: Clone + BitXor<Output = T>,
{
    type Set = T;

    fn operate(lhs: Self::Set, rhs: Self::Set) -> Self::Set {
        lhs ^ rhs
    }
}

impl<T> Monoid for BitXorAlge<T>
where
    T: Clone + BitXor<Output = T> + Zero,
{
    fn id() -> Self::Set {
        T::ZERO
    }
}

impl<T> PartialGroup for BitXorAlge<T>
where
    T: Clone + BitXor<Output = T> + Zero,
{
    fn inverse_operate(lhs: T, rhs: T) -> T {
        lhs ^ rhs
    }
}

impl<T> MathGroup for BitXorAlge<T>
where
    T: Clone + BitXor<Output = T> + Zero,
{
    fn inverse(x: Self::Set) -> Self::Set {
        x
    }
}
