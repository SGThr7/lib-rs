use super::num::{BoundedAbove, BoundedBelow, One, Reciprocal, Zero};

#[codesnip::entry("Semigroup")]
pub mod semigroup {
    /// [Semigroup](https://en.wikipedia.org/wiki/Semigroup) is an algebraic structure consisting of a set together with an associative binary operation.
    ///
    /// # Associativity
    ///
    /// ~~~text
    /// ∀ a, b, c ∈ Set, (a ∘ b) ∘ c = a ∘ (b ∘ c)
    /// ~~~
    pub trait Semigroup {
        type Set: Clone;

        /// It must be satisfy [associativity](https://en.wikipedia.org/wiki/Associative_property) `(a * b) * c = a * (b * c)`.
        fn operate(lhs: &Self::Set, rhs: &Self::Set) -> Self::Set;
    }

    #[macro_export]
    macro_rules! define_semigroup {
        (@impl $semigroup:ident<T: $($bounds:path),*>, |$lhs:ident,$rhs:ident| $expr:expr) => {
            impl<T: Clone + $($bounds+)*> Semigroup for $semigroup<T> {
                type Set = T;
                fn operate(lhs: &Self::Set, rhs: &Self::Set) -> Self::Set {
                    let $lhs = lhs.clone();
                    let $rhs = rhs.clone();
                    $expr
                }
            }
        };
    }
    pub(crate) use define_semigroup;
}

#[codesnip::entry(
    "Monoid",
    include("Semigroup", "BoundedAbove", "BoundedBelow", "Zero", "One")
)]
pub mod monoid {
    use super::{
        semigroup::{define_semigroup, Semigroup},
        BoundedAbove, BoundedBelow, One, Zero,
    };
    use core::{
        marker::PhantomData,
        ops::{Add, BitOr, BitXor, Mul},
    };

    /// [Monoid](https://en.wikipedia.org/wiki/Monoid) is an semigroup with identity element.
    ///
    /// # Identity element
    ///
    /// ~~~text
    /// ∃ e ∈ Set, ∀ a ∈ Set, e ◦ a = a ◦ e = a
    /// ~~~
    pub trait Monoid: Semigroup {
        fn identity() -> Self::Set;
    }

    #[macro_export]
    macro_rules! define_monoid {
        (@impl $monoid:ident<T:$($bounds:path),* $(,)?>, |$lhs:ident, $rhs:ident| $expr:expr, $identity:expr) => {
            define_semigroup!(@impl $monoid<T: $($bounds),*>, |$lhs,$rhs| $expr);
            impl<T: Clone + $($bounds+)*> Monoid for $monoid<T> {
                fn identity() -> Self::Set { $identity }
            }
        };
        ($($monoid:ident<T: $($bounds:path),* $(,)?>, |$lhs:ident, $rhs:ident| $expr:expr, $identity:expr);* $(;)?) => {$(
            pub struct $monoid<T>(PhantomData<T>);
            define_monoid!(@impl $monoid<T: $($bounds),*>, |$lhs,$rhs| $expr, $identity);
        )*};
    }
    pub(crate) use define_monoid;

    define_monoid! {
        AddMonoid<T: Add<Output = T>, Zero>, |lhs,rhs| lhs+rhs, T::zero();
        MulMonoid<T: Mul<Output = T>, One>, |lhs,rhs| lhs*rhs, T::one();
        MaxMonoid<T: Ord, BoundedBelow>, |lhs,rhs| T::max(lhs,rhs), T::lower_bound();
        MinMonoid<T: Ord, BoundedAbove>, |lhs,rhs| T::min(lhs,rhs), T::upper_bound();
        XorMonoid<T: BitXor<Output = T>, Zero>, |lhs,rhs| lhs^rhs, T::zero();
        OrMonoid<T: BitOr<Output = T>, Zero>, |lhs,rhs| lhs|rhs, T::zero();
    }
}

pub mod math_group {
    use super::{
        monoid::{define_monoid, Monoid},
        semigroup::{define_semigroup, Semigroup},
        One, Reciprocal, Zero,
    };
    use core::{
        marker::PhantomData,
        ops::{Add, BitXor, Div, Mul, Neg},
    };

    /// [Group (mathematics)](https://en.wikipedia.org/wiki/Group_(mathematics)) is an monoid with inverse element.
    ///
    /// # Inverse element
    ///
    /// When `e` is identity element,
    ///
    /// ~~~text
    /// ∀ a ∈ Set, ∃ b ∈ Set, a ◦ b = b ◦ a = e
    /// ~~~
    pub trait MathGroup: Monoid {
        fn inverse(x: &Self::Set) -> Self::Set;
        fn inv_operate(lhs: &Self::Set, rhs: &Self::Set) -> Self::Set {
            Self::operate(lhs, &Self::inverse(rhs))
        }
    }

    #[macro_export]
    macro_rules! define_math_group {
        (@impl $group:ident<T: $($bounds:path),* $(,)?>, |$lhs:ident,$rhs:ident| $expr:expr, $identity:expr, |$x:ident| $neg:expr) => {
            define_monoid!(@impl $group<T: $($bounds),*>, |$lhs,$rhs| $expr, $identity);
            impl<T: Clone + $($bounds+)*> MathGroup for $group<T> {
                fn inverse(x: &Self::Set) -> Self::Set {
                    let $x = x.clone();
                    $neg
                }
            }
        };
        ($($group:ident<T: $($bounds:path),* $(,)?>, |$lhs:ident,$rhs:ident| $expr:expr, $identity:expr, |$x:ident| $neg:expr);* $(;)?) => {$(
            pub struct $group<T>(PhantomData<T>);
            define_math_group!(@impl $group<T: $($bounds),*>, |$lhs,$rhs| $expr, $identity, |$x| $neg);
        )*};
    }
    pub(crate) use define_math_group;

    define_math_group! {
        AddGroup<T: Add<Output = T>, Zero, Neg<Output = T>>, |lhs,rhs| lhs+rhs, T::zero(), |x| -x;
        MulGroup<T: Mul<Output = T>, One, Div<Output = T>, Reciprocal<Output = T>>, |lhs,rhs| lhs*rhs, T::one(), |x| x.reciprocal();
        XorGroup<T: BitXor<Output = T>, Zero>, |lhs,rhs| lhs^rhs, T::zero(), |x| x;
    }
}

// #[codesnip::entry(include("Monoid", "Semigroup"))]
// pub trait ActMonoid {
//     type Monoid: Monoid;
//     type Act: Clone;

//     fn identity() -> <Self::Monoid as Semigroup>::Set {
//         Self::Monoid::identity()
//     }

//     fn operate(
//         lhs: &<Self::Monoid as Semigroup>::Set,
//         rhs: &<Self::Monoid as Semigroup>::Set,
//     ) -> <Self::Monoid as Semigroup>::Set {
//         Self::Monoid::operate(lhs, rhs)
//     }

//     fn identity_act() -> Self::Act;
//     fn act(s: &<Self::Monoid as Semigroup>::Set, a: &Self::Act)
//         -> <Self::Monoid as Semigroup>::Set;
//     fn merge_act(lhs: &Self::Act, rhs: &Self::Act) -> Self::Act;
// }

// #[codesnip::entry]
// pub struct ReplaceAct<M>(core::marker::PhantomData<M>);

// #[codesnip::entry("ReplaceAct", include("ActMonoid", "Monoid"))]
// impl<M> ActMonoid for ReplaceAct<M>
// where
//     M: Monoid,
// {
//     type Monoid = M;
//     type Act = Option<M::Set>;

//     fn identity_act() -> Option<M::Set> {
//         None
//     }

//     fn act(s: &M::Set, a: &Option<M::Set>) -> M::Set {
//         if let Some(rhs) = a {
//             rhs.clone()
//         } else {
//             s.clone()
//         }
//     }

//     fn merge_act(lhs: &Option<M::Set>, rhs: &Option<M::Set>) -> Option<M::Set> {
//         match (lhs, rhs) {
//             (_, Some(rhs)) => Some(rhs.clone()),
//             (Some(lhs), None) => Some(lhs.clone()),
//             (None, None) => None,
//         }
//     }
// }

// require tree range
// #[codesnip::entry(include("ReplaceAct", "AddMonoid"))]
// pub type ReplaceSum<T> = ReplaceAct<AddMonoid<T>>;

// #[codesnip::entry(include("ReplaceAct", "MulMonoid"))]
// pub type ReplaceProd<T> = ReplaceAct<MulMonoid<T>>;

// #[codesnip::entry(include("ReplaceAct", "MaxMonoid"))]
// pub type ReplaceMax<T> = ReplaceAct<MaxMonoid<T>>;

// #[codesnip::entry(include("ReplaceAct", "MinMonoid"))]
// pub type ReplaceMin<T> = ReplaceAct<MinMonoid<T>>;

// #[codesnip::entry(include("ReplaceAct", "XorMonoid"))]
// pub type ReplaceXor<T> = ReplaceAct<XorMonoid<T>>;
