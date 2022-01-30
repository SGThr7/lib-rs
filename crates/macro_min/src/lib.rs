/// Compute min of multiple elements.
///
/// # Examples
///
/// ```
/// use macro_min::min;
///
/// assert_eq!(min!(2, 1, 4, 7, 4, 8, 3, 6, 4, 7), 1);
/// ```
#[macro_export]
macro_rules! min {
    ($a:expr $(,)?) => {
        $a
    };
    ($a:expr, $b:expr $(,)?) => {
        Ord::min($a, $b)
    };
    ($a1:expr, $a2:expr, $($ai:expr),+ $(,)?) => {
        Ord::min($a1, $crate::min!($a2, $($ai),+))
    }
}
