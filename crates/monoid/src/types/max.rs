use super::*;
use num_traits_bounded::BoundedBelow;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaxAlge<T>(PhantomData<T>);

impl<T> Semigroup for MaxAlge<T>
where
    T: Clone + Ord,
{
    type Set = T;

    fn operate(lhs: Self::Set, rhs: Self::Set) -> Self::Set {
        lhs.max(rhs)
    }
}

impl<T> Monoid for MaxAlge<T>
where
    T: Clone + Ord + BoundedBelow,
{
    fn id() -> Self::Set {
        T::INFIMUM
    }
}
