#[codesnip::entry(include("Monoid", "Semigroup"))]
pub trait ActMonoid {
    type Monoid: Monoid;
    type Act: Clone;

    fn identity() -> <Self::Monoid as Semigroup>::Set {
        Self::Monoid::identity()
    }

    fn operate(
        lhs: &<Self::Monoid as Semigroup>::Set,
        rhs: &<Self::Monoid as Semigroup>::Set,
    ) -> <Self::Monoid as Semigroup>::Set {
        Self::Monoid::operate(lhs, rhs)
    }

    fn identity_act() -> Self::Act;
    fn act(s: &<Self::Monoid as Semigroup>::Set, a: &Self::Act)
        -> <Self::Monoid as Semigroup>::Set;
    fn merge_act(lhs: &Self::Act, rhs: &Self::Act) -> Self::Act;
}

#[codesnip::entry]
pub struct ReplaceAct<M>(core::marker::PhantomData<M>);

#[codesnip::entry("ReplaceAct", include("ActMonoid", "Monoid"))]
impl<M> ActMonoid for ReplaceAct<M>
where
    M: Monoid,
{
    type Monoid = M;
    type Act = Option<M::Set>;

    fn identity_act() -> Option<M::Set> {
        None
    }

    fn act(s: &M::Set, a: &Option<M::Set>) -> M::Set {
        if let Some(rhs) = a {
            rhs.clone()
        } else {
            s.clone()
        }
    }

    fn merge_act(lhs: &Option<M::Set>, rhs: &Option<M::Set>) -> Option<M::Set> {
        match (lhs, rhs) {
            (_, Some(rhs)) => Some(rhs.clone()),
            (Some(lhs), None) => Some(lhs.clone()),
            (None, None) => None,
        }
    }
}

// require tree range
#[codesnip::entry(include("ReplaceAct", "AddMonoid"))]
pub type ReplaceSum<T> = ReplaceAct<AddMonoid<T>>;

#[codesnip::entry(include("ReplaceAct", "MulMonoid"))]
pub type ReplaceProd<T> = ReplaceAct<MulMonoid<T>>;

#[codesnip::entry(include("ReplaceAct", "MaxMonoid"))]
pub type ReplaceMax<T> = ReplaceAct<MaxMonoid<T>>;

#[codesnip::entry(include("ReplaceAct", "MinMonoid"))]
pub type ReplaceMin<T> = ReplaceAct<MinMonoid<T>>;

#[codesnip::entry(include("ReplaceAct", "XorMonoid"))]
pub type ReplaceXor<T> = ReplaceAct<XorMonoid<T>>;
