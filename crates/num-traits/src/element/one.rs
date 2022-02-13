pub trait One {
    fn one() -> Self;
}

impl_id! { One, one, 1 , u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }
impl_id! { One, one, 1., f32 f64 }
