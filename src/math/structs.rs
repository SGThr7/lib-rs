use super::num::{BoundedAbove, BoundedBelow, One, Zero};

#[codesnip::entry]
pub trait Semigroup {
    type Set: Clone;
    fn operate(lhs: &Self::Set, rhs: &Self::Set) -> Self::Set;
}

#[codesnip::entry(include("Semigroup"))]
pub trait Monoid: Semigroup {
    fn identity() -> Self::Set;
}

#[codesnip::entry(include("Monoid", "Semigroup"))]
#[macro_export]
macro_rules! define_monoid {
    ($monoid:ident<T: $($bounds:path),*>, |$lhs:ident, $rhs:ident| $operate:expr, $identity:expr) => {
        pub struct $monoid<S>(core::marker::PhantomData<S>);
        impl<T: Clone + $($bounds+)*> Monoid for $monoid<T> {
            fn identity() -> Self::Set { $identity }
        }
        impl<T: Clone + $($bounds+)*> Semigroup for $monoid<T> {
            type Set = T;
            fn operate(lhs: &Self::Set, rhs: &Self::Set) -> Self::Set {
                let $lhs = lhs.clone();
                let $rhs = rhs.clone();
                $operate
            }
        }
    };
}

#[codesnip::entry("AddMonoid", include("define_monoid", "Zero"))]
define_monoid!(AddMonoid<T: core::ops::Add<Output = T>, Zero>, |lhs,rhs| T::add(lhs,rhs), T::zero());

#[codesnip::entry("MulMonoid", include("define_monoid", "One"))]
define_monoid!(MulMonoid<T: core::ops::Mul<Output = T>, One>, |lhs,rhs| T::mul(lhs,rhs), T::one());

#[codesnip::entry("MaxMonoid", include("define_monoid", "BoundedBelow"))]
define_monoid!(MaxMonoid<T: Ord, BoundedBelow>, |lhs,rhs| T::max(lhs,rhs), T::lower_bound());

#[codesnip::entry("MinMonoid", include("define_monoid", "BoundedAbove"))]
define_monoid!(MinMonoid<T: Ord, BoundedAbove>, |lhs,rhs| T::min(lhs,rhs), T::upper_bound());

#[codesnip::entry("XorMonoid", include("define_monoid", "Zero"))]
define_monoid!(XorMonoid<T: core::ops::BitXor<Output = T>, Zero>, |lhs,rhs| T::bitxor(lhs,rhs), T::zero());

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
// #[codesnip::entry(include("ReplaceAct", "AddMonoid"))]
// pub type ReplaceSum<T> = ReplaceAct<AddMonoid<T>>;

// #[codesnip::entry(include("ReplaceAct", "MulMonoid"))]
// pub type ReplaceProd<T> = ReplaceAct<MulMonoid<T>>;

#[codesnip::entry(include("ReplaceAct", "MaxMonoid"))]
pub type ReplaceMax<T> = ReplaceAct<MaxMonoid<T>>;

#[codesnip::entry(include("ReplaceAct", "MinMonoid"))]
pub type ReplaceMin<T> = ReplaceAct<MinMonoid<T>>;

// #[codesnip::entry(include("ReplaceAct", "XorMonoid"))]
// pub type ReplaceXor<T> = ReplaceAct<XorMonoid<T>>;
