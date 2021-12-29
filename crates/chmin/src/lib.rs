/// Replace with minimam one.
///
/// # Examples
///
/// ```
/// use chmin::chmin;
///
/// let mut x = 10;
///
/// chmin!(x, 20);
/// assert_eq!(x, 10);
///
/// chmin!(x, 5);
/// assert_eq!(x, 5);
/// ```
#[macro_export]
macro_rules! chmin {
    ($lhs:expr, $rhs:expr) => {
        $lhs = $lhs.min($rhs)
    };
}
