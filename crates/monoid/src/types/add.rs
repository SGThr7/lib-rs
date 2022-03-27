use super::*;
use num_traits_zero::Zero;
use std::ops::{Add, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AddAlge<T>(PhantomData<T>);

impl<T> Semigroup for AddAlge<T>
where
    T: Clone + Add<Output = T>,
{
    type Set = T;

    fn operate(lhs: Self::Set, rhs: Self::Set) -> Self::Set {
        lhs + rhs
    }
}

impl<T> Monoid for AddAlge<T>
where
    T: Clone + Add<Output = T> + Zero,
{
    fn id() -> Self::Set {
        T::ZERO
    }
}

impl<T> PartialGroup for AddAlge<T>
where
    T: Clone + Add<Output = T> + Zero + Sub<Output = T>,
{
    fn inverse_operate(lhs: Self::Set, rhs: Self::Set) -> Self::Set {
        lhs - rhs
    }
}

impl<T> MathGroup for AddAlge<T>
where
    T: Clone + Add<Output = T> + Zero + Sub<Output = T> + Neg<Output = T>,
{
    fn inverse(x: Self::Set) -> Self::Set {
        -x
    }
}
