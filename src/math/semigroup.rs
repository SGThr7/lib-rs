#[codesnip::entry]
pub trait Semigroup {
    fn op(self, rhs: Self) -> Self;
}

#[codesnip::entry(include("Semigroup"))]
#[macro_export]
macro_rules! impl_semigroup {
    ($name:ident <$($bounds:path),*>, $fn:path) => {
        impl<T: $($bounds+)*> Semigroup for $name<T> {
            fn op(self, rhs: Self) -> Self {
                $fn(self.0, rhs.0).into()
            }
        }
    };
}

#[codesnip::entry(include("impl_semigroup", "impl_traits"))]
#[macro_export]
macro_rules! define_primitive_semigroup {
    ($name:ident <$($bounds:path),*>, $fn:path) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name<T: $($bounds+)*>(T);
        impl_semigroup!($name <$($bounds),*>, $fn);
        impl_traits!($name <$($bounds),*>);
    };
}

#[codesnip::entry("AddSemigroup", include("define_primitive_semigroup"))]
define_primitive_semigroup!(AddSemigroup<core::ops::Add<Output = T>>, T::add);

#[codesnip::entry("MulSemigroup", include("define_primitive_semigroup"))]
define_primitive_semigroup!(MulSemigroup<core::ops::Mul<Output = T>>, T::mul);

#[codesnip::entry("MaxSemigroup", include("define_primitive_semigroup"))]
define_primitive_semigroup!(MaxSemigroup<core::cmp::Ord>, T::max);

#[codesnip::entry("MinSemigroup", include("define_primitive_semigroup"))]
define_primitive_semigroup!(MinSemigroup<core::cmp::Ord>, T::min);

#[codesnip::entry("XorSemigroup", include("define_primitive_semigroup"))]
define_primitive_semigroup!(XorSemigroup<core::ops::BitXor<Output = T>>, T::bitxor);

macro_rules! test_semigroups {
    ($($testname:ident: $struct:tt;)*) => {
        $(#[test]
        fn $testname() {
            // associativity
            let a = $struct(0b101001101);
            let b = $struct(0b100101011);
            let c = $struct(0b100011011);
            assert_eq!((a.op(b)).op(c), a.op(b.op(c)));
        })*
    };
}

test_semigroups! {
    add: AddSemigroup;
    mul: MulSemigroup;
    max: MaxSemigroup;
    min: MinSemigroup;
    xor: XorSemigroup;
}
