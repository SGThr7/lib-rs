use super::*;
use num_traits_zero::Zero;
use std::ops::{Add, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AddAlge<T>(PhantomData<T>);

impl<T> Semigroup for AddAlge<T>
where
    for<'a> &'a T: Add<Output = T>,
{
    type Set = T;

    fn operate(lhs: &Self::Set, rhs: &Self::Set) -> Self::Set {
        lhs + rhs
    }
}

impl<T> Monoid for AddAlge<T>
where
    T: Zero,
    for<'a> &'a T: Add<Output = T>,
{
    fn id() -> Self::Set {
        T::ZERO
    }
}

impl<T> PartialGroup for AddAlge<T>
where
    T: Zero,
    for<'a> &'a T: Add<Output = T> + Sub<Output = T>,
{
    fn inverse_operate(lhs: &Self::Set, rhs: &Self::Set) -> Self::Set {
        lhs - rhs
    }
}

impl<T> MathGroup for AddAlge<T>
where
    T: Zero,
    for<'a> &'a T: Add<Output = T> + Sub<Output = T> + Neg<Output = T>,
{
    fn inverse(x: &Self::Set) -> Self::Set {
        -x
    }
}
