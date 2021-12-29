/// Replace with maximum one.
///
/// # Examples
///
/// ```
/// use chmax::chmax;
///
/// let mut x = 10;
///
/// chmax!(x, 5);
/// assert_eq!(x, 10);
///
/// chmax!(x, 20);
/// assert_eq!(x, 20);
/// ```
#[macro_export]
macro_rules! chmax {
    ($lhs:expr, $rhs:expr) => {
        $lhs = $lhs.max($rhs)
    };
}
