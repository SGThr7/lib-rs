use super::*;
use num_traits_bounded::BoundedAbove;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MinAlge<T>(PhantomData<T>);

impl<T> Semigroup for MinAlge<T>
where
    T: Clone + Ord,
{
    type Set = T;

    fn operate(lhs: Self::Set, rhs: Self::Set) -> Self::Set {
        lhs.min(rhs)
    }
}

impl<T> Monoid for MinAlge<T>
where
    T: Clone + Ord + BoundedAbove,
{
    fn id() -> Self::Set {
        T::SUPREMUM
    }
}
