use num_traits_macro_impl_id::impl_id;
use std::{u128, u16, u32, u64, u8, usize};

/// The trait that implements the all bit one value.
///
/// # Example
///
/// ```
/// use num_traits_all_bit_one::AllBitOne;
///
/// let uint = u8::ALL_BIT_ONE;
///
/// assert_eq!(uint, 0b1111_1111);
/// assert_eq!(uint.count_ones(), 8);
///
/// let int = i8::ALL_BIT_ONE;
///
/// assert_eq!(int, 0b1111_1111_u8 as i8);
/// assert_eq!(int.count_ones(), 8);
/// ```
pub trait AllBitOne {
    const ALL_BIT_ONE: Self;
}

macro_rules! impl_all_bit_one {
    ($id:tt, $($t:tt)*) => {$(
        impl_id! { AllBitOne, ALL_BIT_ONE, $t::$id, $t }
    )*};
}

impl_all_bit_one! { MAX, u8 u16 u32 u64 u128 usize }
impl_id! { AllBitOne, ALL_BIT_ONE, -1, i8 i16 i32 i64 i128 isize }
