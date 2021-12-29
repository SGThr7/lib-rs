/// Creates a multi dimention [`Vec`].
///
/// # Examples
///
/// ```
/// use macro_matrix::mat;
///
/// let m = mat![1; 2, 3];
///
/// assert_eq!(m.len(), 2);
/// assert!(m.iter().all(|v| v.len() == 3));
/// assert!(m.iter().flatten().all(|&x| x == 1));
/// ```
#[macro_export]
macro_rules! mat {
    () => {
        vec![]
    };
    ($e:expr; $n:expr $(,)?) => {
        vec![$e; $n]
    };
    ($e:expr; $nhead:expr, $($ntail:expr),* $(,)?) => {
        vec![$crate::mat![$e; $($ntail),*]; $nhead]
    };
}
