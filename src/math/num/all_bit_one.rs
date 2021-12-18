pub trait AllBitOne {
    const ALL_BIT_ONE: Self;
}

macro_rules! impl_allone_uint {
    (for $($t:tt)*) => {$(
        impl AllBitOne for $t {
            const ALL_BIT_ONE: Self = core::$t::MAX;
        }
    )*};
}

macro_rules! impl_allone_int {
    (for $($t:ty)*) => {$(
        impl AllBitOne for $t {
            const ALL_BIT_ONE: Self = -1;
        }
    )*};
}

impl_allone_uint! { for u8 u16 u32 u64 u128 usize }
impl_allone_int! { for i8 i16 i32 i64 i128 isize }
