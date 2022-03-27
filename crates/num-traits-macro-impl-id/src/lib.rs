/// The macro that implements identity function.
///
/// # Examples
///
/// ```
/// use num_traits_macro_impl_id::impl_id;
///
/// trait FavoriteNumber {
///     const FAVORITE_NUMBER: Self;
/// }
///
/// impl_id! { FavoriteNumber, FAVORITE_NUMBER, 57, u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }
///
/// assert_eq!(usize::FAVORITE_NUMBER, 57);
/// ```
#[macro_export]
macro_rules! impl_id {
    ($trait:ident, $name:ident, $id:expr, $($t:ty)*) => {$(
        impl $trait for $t {
            const $name: $t = $id;
        }
    )*};
}
