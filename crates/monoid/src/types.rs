use super::{Monoid, Semigroup};
use num_traits::element::{AllBitOne, BoundedAbove, BoundedBelow, One, Zero};
use std::{
    marker::PhantomData,
    ops::{Add, BitAnd, BitOr, BitXor, Mul},
};

macro_rules! define_monoid {
    (@impl $monoid:ident @type $t:tt @bounds [$($bounds:tt)*] @rest) => {};
    (@impl $monoid:ident @type $t:tt @bounds [$($bounds:tt)*] @rest operate: |$lhs:ident, $rhs:ident| $op:expr , $($rest:tt)*) => {
        impl<$t: Clone + $($bounds)*> Semigroup for $monoid<$t> {
            type Set = $t;

            fn operate($lhs: $t, $rhs: $t) -> $t { $op }
        }
        define_monoid! {
            @impl $monoid
            @type $t
            @bounds [$($bounds)*]
            @rest $($rest)*
        }
    };
    (@impl $monoid:ident @type $t:tt @bounds [$($bounds:tt)*] @rest id: $id:expr , $($rest:tt)*) => {
        impl<$t: Clone + $($bounds)*> Monoid for $monoid<$t> {
            fn id() -> $t { $id }
        }
        define_monoid! {
            @impl $monoid
            @type $t
            @bounds [$($bounds)*]
            @rest $($rest)*
        }
    };
    (impl<$t:tt $(: $($bounds:path),*)?> $monoid:ident { $($rest:tt)* } ) => {
        pub struct $monoid<$t: Clone + $($($bounds+)*)? >(PhantomData<$t>);

        define_monoid! {
            @impl $monoid
            @type $t
            @bounds [$($($bounds+)*)?]
            @rest $($rest)*
        }
    };
}

define_monoid! {
    impl<T: Zero, Add<Output = T>> AddMonoid {
        operate: |lhs, rhs| lhs + rhs,
        id: Zero::zero(),
    }
}

define_monoid! {
    impl<T: One, Mul<Output = T>> MulMonoid {
        operate: |lhs, rhs| lhs * rhs,
        id: One::one(),
    }
}

define_monoid! {
    impl<T: Ord, BoundedBelow> MaxMonoid {
        operate: |lhs, rhs| Ord::max(lhs, rhs),
        id: BoundedBelow::lower_bound(),
    }
}

define_monoid! {
    impl<T: Ord, BoundedAbove> MinMonoid {
        operate: |lhs, rhs| Ord::min(lhs, rhs),
        id: BoundedAbove::upper_bound(),
    }
}

define_monoid! {
    impl<T: Zero, BitOr<Output = T>> BitOrMonoid {
        operate: |lhs, rhs| BitOr::bitor(lhs, rhs),
        id: Zero::zero(),
    }
}

define_monoid! {
    impl<T: AllBitOne, BitAnd<Output = T>> BitAndMonoid {
        operate: |lhs, rhs| BitAnd::bitand(lhs, rhs),
        id: AllBitOne::all_bit_one(),
    }
}

define_monoid! {
    impl<T: Zero, BitXor<Output = T>> BitXorMonoid {
        operate: |lhs, rhs| BitXor::bitxor(lhs, rhs),
        id: Zero::zero(),
    }
}
