use super::*;
use num_traits_bounded::BoundedAbove;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MinAlge<T>(PhantomData<T>);

impl<T> Semigroup for MinAlge<T>
where
    T: Ord + Clone,
{
    type Set = T;

    fn operate(lhs: &Self::Set, rhs: &Self::Set) -> Self::Set {
        lhs.min(rhs).clone()
    }
}

impl<T> Monoid for MinAlge<T>
where
    T: Ord + Clone + BoundedAbove,
{
    fn id() -> Self::Set {
        T::SUPREMUM
    }
}
