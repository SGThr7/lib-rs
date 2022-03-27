use num_traits_macro_impl_id::impl_id;

/// The trait that implements zero.
///
/// # Examples
///
/// ```
/// use num_traits_zero::Zero;
///
/// let zero = usize::ZERO;
///
/// assert_eq!(zero, 0);
/// assert_eq!(zero * 7, 0);
/// ```
pub trait Zero {
    const ZERO: Self;
}

impl_id! { Zero, ZERO, 0, u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }
impl_id! { Zero, ZERO, 0., f32 f64 }
