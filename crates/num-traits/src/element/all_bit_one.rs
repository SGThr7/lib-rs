use std::{i128, i16, i32, i64, i8, isize, u128, u16, u32, u64, u8, usize};

macro_rules! impl_all_bit_one {
    ($id:tt, $($t:tt)*) => {$(
        impl_id! { AllBitOne, all_bit_one, $t::$id, $t }
    )*};
}

pub trait AllBitOne {
    fn all_bit_one() -> Self;
}

impl_all_bit_one! { MAX, u8 u16 u32 u64 u128 usize }
impl_all_bit_one! { MIN, i8 i16 i32 i64 i128 isize }
