pub trait Zero {
    fn zero() -> Self;
}

impl_id! { Zero, zero, 0 , u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }
impl_id! { Zero, zero, 0., f32 f64 }
