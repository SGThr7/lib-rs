pub trait OverflowingOps:
    OverflowingAdd
    + OverflowingSub
    + OverflowingMul
    + OverflowingDiv
    + OverflowingDivEuclid
    + OverflowingRem
    + OverflowingRemEuclid
    + OverflowingShl
    + OverflowingShr
    + OverflowingPow
    + OverflowingAbs
    + OverflowingNeg
{
}

impl_defs! { OverflowingOps }

impl_binop! { OverflowingAdd, overflowing_add -> (Self, bool) }
impl_binop! { OverflowingSub, overflowing_sub -> (Self, bool) }
impl_binop! { OverflowingMul, overflowing_mul -> (Self, bool) }
impl_binop! { OverflowingDiv, overflowing_div -> (Self, bool) }
impl_binop! { OverflowingDivEuclid, overflowing_div_euclid -> (Self, bool) }
impl_binop! { OverflowingRem, overflowing_rem -> (Self, bool) }
impl_binop! { OverflowingRemEuclid, overflowing_rem_euclid -> (Self, bool) }

impl_binop! { OverflowingShl, overflowing_shl, u32, -> (Self, bool) }
impl_binop! { OverflowingShr, overflowing_shr, u32, -> (Self, bool) }
impl_binop! { OverflowingPow, overflowing_pow, u32, -> (Self, bool) }

impl_unaryop! { OverflowingAbs, overflowing_abs -> (Self, bool), i8 i16 i32 i64 i128 isize }
impl_uint_abs! { OverflowingAbs, overflowing_abs -> (Self, bool), |self| (self, true) }
impl_unaryop! { OverflowingNeg, overflowing_neg -> (Self, bool) }
