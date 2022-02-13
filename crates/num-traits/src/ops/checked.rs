pub trait CheckedOps:
    CheckedAdd
    + CheckedSub
    + CheckedMul
    + CheckedDiv
    + CheckedDivEuclid
    + CheckedRem
    + CheckedRemEuclid
    + CheckedShl
    + CheckedShr
    + CheckedPow
    + CheckedAbs
    + CheckedNeg
{
}

impl_defs! { CheckedOps }

impl_binop! { CheckedAdd, checked_add -> Option<Self> }
impl_binop! { CheckedSub, checked_sub -> Option<Self> }
impl_binop! { CheckedMul, checked_mul -> Option<Self> }
impl_binop! { CheckedDiv, checked_div -> Option<Self> }
impl_binop! { CheckedDivEuclid, checked_div_euclid -> Option<Self> }
impl_binop! { CheckedRem, checked_rem -> Option<Self> }
impl_binop! { CheckedRemEuclid, checked_rem_euclid -> Option<Self> }

impl_binop! { CheckedShl, checked_shl, u32, -> Option<Self> }
impl_binop! { CheckedShr, checked_shr, u32, -> Option<Self> }
impl_binop! { CheckedPow, checked_pow, u32, -> Option<Self> }

impl_unaryop! { CheckedAbs, checked_abs -> Option<Self>, i8 i16 i32 i64 i128 isize }
impl_uint_abs! { CheckedAbs, checked_abs -> Option<Self>, |self| Some(self) }
impl_unaryop! { CheckedNeg, checked_neg -> Option<Self> }
