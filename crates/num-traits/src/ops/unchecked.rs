pub trait UncheckedOps:
    UncheckedAdd + UncheckedSub + UncheckedMul + UncheckedShl + UncheckedShr
{
}

impl_defs! { UncheckedOps }

macro_rules! impl_unchecked_binop {
    ($trait:ident, $fn:ident -> $ret:ty) => {
        pub trait $trait: Sized {
            unsafe fn $fn(self, rhs: Self) -> $ret;
        }

        impl_unchecked_binop! { $trait, $fn -> $ret, u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }
    };
    ($trait:ident, $fn:ident -> $ret:ty, $($t:ty)*) => {$(
        impl $trait for $t {
            unsafe fn $fn(self, rhs: Self) -> $ret {
                <$t>::$fn(self, rhs)
            }
        }
    )*};
}

// impl_unchecked_binop! { UncheckedAdd, unchecked_add -> Self }
// impl_unchecked_binop! { UncheckedSub, unchecked_sub -> Self }
// impl_unchecked_binop! { UncheckedMul, unchecked_mul -> Self }
// impl_unchecked_binop! { UncheckedShl, unchecked_shl -> Self }
// impl_unchecked_binop! { UncheckedShr, unchecked_shr -> Self }
