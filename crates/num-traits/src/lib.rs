macro_rules! impl_defs {
    ($trait:ident: $($t:ty)*) => {$(
        impl $trait for $t {}
    )*};
    ($trait:ident) => {
        impl_defs! { $trait: u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }
    };
}

pub mod element;
pub mod integer;
pub mod ops;
