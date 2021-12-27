/// Implements binary operations `&T, U`, `T, &U` and `&T, &U`.
///
/// Where `T` and `U` must be implemented `$trait` and [`Copy`].
///
/// [`Copy`]: std::marker::Copy
#[macro_export]
macro_rules! forward_ref_binop {
    (impl $trait:ident, $fn:ident for $t:ty, $u:ty) => {
        impl $trait<$u> for &$t {
            type Output = <$t as $trait<$u>>::Output;

            fn $fn(self, rhs: $u) -> Self::Output {
                $trait::$fn(*self, rhs)
            }
        }

        impl $trait<&$u> for $t {
            type Output = <$t as $trait<$u>>::Output;

            fn $fn(self, rhs: &$u) -> Self::Output {
                $trait::$fn(self, *rhs)
            }
        }

        impl $trait<&$u> for &$t {
            type Output = <$t as $trait<$u>>::Output;

            fn $fn(self, rhs: &$u) -> Self::Output {
                $trait::$fn(*self, *rhs)
            }
        }
    };
}
