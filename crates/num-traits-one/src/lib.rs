use num_traits_macro_impl_id::impl_id;

/// The trait that implements one.
///
/// # Examples
///
/// ```
/// use num_traits_one::One;
///
/// let one = usize::ONE;
///
/// assert_eq!(one, 1);
/// assert_eq!(one * 7, 7);
/// ```
pub trait One {
    const ONE: Self;
}

impl_id! { One, ONE, 1, u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }
impl_id! { One, ONE, 1., f32 f64 }
