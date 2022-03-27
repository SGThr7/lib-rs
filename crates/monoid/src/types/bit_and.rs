use std::ops::BitAnd;

use num_traits_all_bit_one::AllBitOne;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitAndAlge<T>(PhantomData<T>);

impl<T> Semigroup for BitAndAlge<T>
where
    T: Clone + BitAnd<Output = T>,
{
    type Set = T;

    fn operate(lhs: Self::Set, rhs: Self::Set) -> Self::Set {
        lhs & rhs
    }
}

impl<T> Monoid for BitAndAlge<T>
where
    T: Clone + BitAnd<Output = T> + AllBitOne,
{
    fn id() -> Self::Set {
        T::ALL_BIT_ONE
    }
}
