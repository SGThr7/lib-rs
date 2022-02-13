pub trait SaturatingOps: SaturatingAdd + SaturatingSub + SaturatingMul + SaturatingPow {}

impl_defs! { SaturatingOps }

impl_binop! { SaturatingAdd, saturating_add -> Self }
impl_binop! { SaturatingSub, saturating_sub -> Self }
impl_binop! { SaturatingMul, saturating_mul -> Self }
// impl_binop! { SaturatingDiv, saturating_div -> Self }
impl_binop! { SaturatingPow, saturating_pow, u32, -> Self }
