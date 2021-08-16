use super::semigroup::Semigroup;

#[codesnip::entry(include("Semigroup"))]
pub trait Monoid: Semigroup {
    fn e() -> Self;
}

#[codesnip::entry(include("Monoid", "impl_semigroup"))]
#[macro_export]
macro_rules! impl_monoid {
    ($name:ident <$($bounds:path),*>, $fn:path, $e:expr) => {
        impl<T: $($bounds+)*> Monoid for $name<T> {
            fn e() -> Self {
                $e.into()
            }
        }
        impl<T: $($bounds+)*> Default for $name<T> {
            fn default() -> Self {
                Self::e()
            }
        }
        impl_semigroup!($name <$($bounds),*>, $fn);
    };
}

#[codesnip::entry(include("impl_monoid", "impl_traits"))]
#[macro_export]
macro_rules! define_primitive_monoid {
    ($name:ident <$($bounds:path),*>, $fn:path, $e:expr) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name<T: $($bounds+)*>(T);
        impl_monoid!($name <$($bounds),*>, $fn, $e);
        impl_traits!($name <$($bounds),*>);
    };
}

#[codesnip::entry("AddMonoid", include("define_primitive_monoid"))]
define_primitive_monoid!(AddMonoid<num::Zero>, T::add, T::zero());

#[codesnip::entry("MulMonoid", include("define_primitive_monoid"))]
define_primitive_monoid!(MulMonoid<num::One>, T::mul, T::one());

#[codesnip::entry("MaxMonoid", include("define_primitive_monoid"))]
define_primitive_monoid!(MaxMonoid<core::cmp::Ord, num::Bounded>, T::max, T::min_value());

#[codesnip::entry("MinMonoid", include("define_primitive_monoid"))]
define_primitive_monoid!(MinMonoid<core::cmp::Ord, num::Bounded>, T::min, T::max_value());

#[codesnip::entry("XorMonoid", include("define_primitive_monoid"))]
define_primitive_monoid!(
    XorMonoid<core::ops::BitXor<Output = T>, num::Zero>,
    T::bitxor,
    T::zero()
);

macro_rules! test_monoid {
    ($($testname:ident: $struct:tt;)*) => {
        $(#[test]
        fn $testname() {
            // associativity
            let a = $struct(33);
            let b = $struct(16);
            let c = $struct(28);
            assert_eq!((a.op(b)).op(c), a.op(b.op(c)));
            // identity
            let e = $struct::e();
            assert_eq!(e.op(a), a.op(e));
        })*
    };
}

test_monoid! {
    add: AddMonoid;
    mul: MulMonoid;
    max: MaxMonoid;
    min: MinMonoid;
    xor: XorMonoid;
}
