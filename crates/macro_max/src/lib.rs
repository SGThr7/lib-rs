/// Compute max of multiple elements.
///
/// # Examples
///
/// ```
/// use macro_max::max;
///
/// assert_eq!(max!(2, 1, 4, 7, 4, 8, 3, 6, 4, 7), 8);
/// ```
#[macro_export]
macro_rules! max {
    ($a:expr $(,)?) => {
        $a
    };
    ($a:expr, $b:expr $(,)?) => {
        Ord::max($a, $b)
    };
    ($a1:expr, $a2:expr, $($ai:expr),+ $(,)?) => {
        Ord::max($a1, $crate::max!($a2, $($ai),+))
    }
}
