use super::{
    element::{One, Zero},
    ops::{AssignOps, Ops},
};

pub trait Integer: Sized + Ops + AssignOps + Zero + One {}

impl_defs! { Integer: i8 i16 i32 i64 i128 isize }

pub trait UnsignedInteger: Sized + Ops + AssignOps + Zero + One {}

impl_defs! { UnsignedInteger: u8 u16 u32 u64 u128 usize }
