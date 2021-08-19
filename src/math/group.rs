use super::{monoid::Monoid, semigroup::Semigroup};

#[codesnip::entry(include("Monoid"))]
pub trait Group: Monoid {
    fn inv(self) -> Self;
}

#[codesnip::entry(include("Group", "impl_monoid"))]
macro_rules! impl_group {
    ($name:ident <$($bounds:path),*>, $fn:path, $e:expr, $inv:path) => {
        impl<T: $($bounds+)*> Group for $name<T> {
            fn inv(self) -> Self {
                $inv(self.0).into()
            }
        }
        impl_monoid!($name <$($bounds),*>, $fn, $e);
    };
}

#[codesnip::entry(include("impl_group", "impl_traits"))]
macro_rules! define_primitive_group {
    ($name:ident <$($bounds:path),*>, $fn:path, $e:expr, $inv:path) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $name<T: $($bounds+)*>(T);
        impl_group!($name <$($bounds),*>, $fn, $e, $inv);
        impl_traits!($name <$($bounds),*>);
    };
}

#[codesnip::entry("AddGroup", include("define_primitive_group"))]
define_primitive_group!(AddGroup<num::Zero, core::ops::Neg<Output = T>>, T::add, T::zero(), T::neg);

#[codesnip::entry("MulGroup", include("define_primitive_group"))]
define_primitive_group!(MulGroup<num::One, num::traits::Inv<Output = T>>, T::mul, T::one(), T::inv);

macro_rules! test_group {
    ($($testname:ident: $struct:tt;)*) => {
        $(#[test]
        fn $testname() {
            // associativity
            let a = $struct(33.);
            let b = $struct(16.);
            let c = $struct(28.);
            assert_eq!((a.op(b)).op(c), a.op(b.op(c)));
            // identity
            let e = $struct::e();
            assert_eq!(e.op(a), a.op(e));
            // inverse
            let ia = a.inv();
            assert_eq!(a.op(ia), e);
            assert_eq!(ia.op(a), e);
        })*
    };
}

test_group! {
   add: AddGroup;
   mul: MulGroup;
}
