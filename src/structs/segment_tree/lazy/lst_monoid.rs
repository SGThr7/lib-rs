use crate::math::num::{
    alge_struct::monoid::{AddMonoid, BitOrMonoid, BitXorMonoid, MaxMonoid, MinMonoid, MulMonoid},
    Monoid,
};

#[codesnip::entry(include("Monoid"))]
pub trait LSTMonoid<M: Monoid> {
    fn id_act() -> Option<M::Set> {
        None
    }

    fn is_id_act(act: &Option<M::Set>) -> bool {
        act.is_none()
    }

    fn drain(acter: &mut Option<M::Set>) -> Option<M::Set> {
        let mut y = Self::id_act();
        core::mem::swap(acter, &mut y);
        y
    }

    fn act(set: &M::Set, acter: &Option<M::Set>, range: usize) -> M::Set;

    fn merge_act(lhs: &Option<M::Set>, rhs: &Option<M::Set>) -> Option<M::Set>;
}

#[codesnip::entry]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LMRep<M>(core::marker::PhantomData<M>);

#[codesnip::entry(include("LMRep", "AddMonoid"))]
pub type RepSum<T> = LMRep<AddMonoid<T>>;
#[codesnip::entry(include("LMRep", "MulMonoid"))]
pub type RepProd<T> = LMRep<MulMonoid<T>>;
#[codesnip::entry(include("LMRep", "MaxMonoid"))]
pub type RepMax<T> = LMRep<MaxMonoid<T>>;
#[codesnip::entry(include("LMRep", "MinMonoid"))]
pub type RepMin<T> = LMRep<MinMonoid<T>>;
#[codesnip::entry(include("LMRep", "BitXorMonoid"))]
pub type RepXor<T> = LMRep<BitXorMonoid<T>>;
#[codesnip::entry(include("LMRep", "BitOrMonoid"))]
pub type RepOr<T> = LMRep<BitOrMonoid<T>>;

#[codesnip::entry("LMRep", include("Monoid", "LSTMonoid"))]
impl<M: Monoid> LSTMonoid<M> for LMRep<M> {
    fn act(set: &M::Set, acter: &Option<M::Set>, range: usize) -> M::Set {
        match acter {
            Some(acter) => (0..range).fold(M::id(), |acc, _| M::operate(&acc, acter)),
            None => set.clone(),
        }
    }

    fn merge_act(lhs: &Option<M::Set>, rhs: &Option<M::Set>) -> Option<M::Set> {
        match rhs {
            Some(_) => rhs.clone(),
            None => lhs.clone(),
        }
    }
}

#[codesnip::entry]
pub struct LMAdd<M>(core::marker::PhantomData<M>);

#[codesnip::entry(include("LMAdd", "AddMonoid"))]
pub type AddSum<T> = LMAdd<AddMonoid<T>>;
#[codesnip::entry(include("LMAdd", "MulMonoid"))]
pub type AddProd<T> = LMAdd<MulMonoid<T>>;
#[codesnip::entry(include("LMAdd", "MaxMonoid"))]
pub type AddMax<T> = LMAdd<MaxMonoid<T>>;
#[codesnip::entry(include("LMAdd", "MinMonoid"))]
pub type AddMin<T> = LMAdd<MinMonoid<T>>;
#[codesnip::entry(include("LMAdd", "BitXorMonoid"))]
pub type AddXor<T> = LMAdd<BitXorMonoid<T>>;
#[codesnip::entry(include("LMAdd", "BitOrMonoid"))]
pub type AddOr<T> = LMAdd<BitOrMonoid<T>>;

#[codesnip::entry("LMAdd", include("Monoid", "LSTMonoid"))]
impl<M> LSTMonoid<M> for LMAdd<M>
where
    M: Monoid,
    M::Set: core::ops::Add<Output = M::Set>,
{
    fn act(set: &M::Set, acter: &Option<M::Set>, range: usize) -> M::Set {
        match acter {
            Some(acter) => (0..range).fold(set.clone(), |acc, _| M::operate(&acc, acter)),
            None => set.clone(),
        }
    }

    fn merge_act(lhs: &Option<M::Set>, rhs: &Option<M::Set>) -> Option<M::Set> {
        match (lhs, rhs) {
            (Some(lhs), Some(rhs)) => Some(lhs.clone() + rhs.clone()),
            (lhs, rhs) => lhs.clone().or(rhs.clone()),
        }
    }
}
