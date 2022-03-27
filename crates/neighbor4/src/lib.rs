/// Iterate 4 neighbor.
///
/// # Examples
///
/// ```
/// use neighbor4::neighbor4;
/// use std::collections::HashSet;
///
/// let mat = vec![
///     vec![0, 1, 2],
///     vec![3, 4, 5],
///     vec![6, 7, 8],
/// ];
///
/// let set = neighbor4!(1, 1)
///     .map(|(x, y)| mat[x][y])
///     .collect::<HashSet<_>>();
///
/// assert_eq!(set, vec![1, 3, 5, 7].into_iter().collect::<HashSet<_>>());
/// ```
#[macro_export]
macro_rules! neighbor4 {
    ($x:expr, $y:expr) => { $crate::neighbor4!(($x, $y)) };
    (($x:expr, $y:expr)) => { $crate::neighbor4!(($x, $y): usize) };
    (($x:expr, $y:expr): $t:tt) => { $crate::neighbor4!(($x, $y): $t in ..) };
    (($x:expr, $y:expr) in $range:expr) => { $crate::neighbor4!(($x, $y) in $range, $range) };
    (($x:expr, $y:expr) in $range_x:expr, $range_y:expr) => { $crate::neighbor4!(($x, $y): usize in $range_x, $range_y) };
    (($x:expr, $y:expr): $t:tt in $range:expr) => { $crate::neighbor4!(($x, $y): $t in $range, $range) };
    (($x:expr, $y:expr): $t:tt in $range_x:expr, $range_y:expr) => {{
        use core::convert::TryInto;
        use core::ops::RangeBounds;
        let x: $t = $x.try_into().unwrap();
        let y: $t = $y.try_into().unwrap();
        vec![
            (x.checked_sub(1), Some(y)),
            (Some(x), y.checked_sub(1)),
            (x.checked_add(1), Some(y)),
            (Some(x), y.checked_add(1)),
        ]
        .into_iter()
        .filter_map(|p| match p {
            (Some(x), Some(y))
                if RangeBounds::<$t>::contains(&$range_x, &x)
                    && RangeBounds::<$t>::contains(&$range_y, &y) =>
            {
                Some((x, y))
            }
            _ => None,
        })
    }};
}

#[cfg(test)]
mod tests {
    macro_rules! check_vec_pattern {
        ($a:expr, $b:expr) => {
            $a.iter()
                .for_each(|x| assert!($b.contains(x), "{:?} is not contained at {:?}", x, $b));
            assert_eq!($a.len(), $b.len());
        };
    }

    #[test]
    fn neighbor4() {
        let dir = neighbor4!((0,0): isize in ..=0, 0..).collect::<Vec<_>>();
        let sample = vec![(-1, 0), (0, 1)];
        check_vec_pattern!(dir, sample);
    }

    #[test]
    fn omit_type() {
        let dir = neighbor4!(0, 0).collect::<Vec<_>>();
        let sample = vec![(1, 0), (0, 1)];
        check_vec_pattern!(dir, sample);
    }

    #[test]
    fn specify_type() {
        let dir = neighbor4!((0, 0): isize).collect::<Vec<_>>();
        let sample = vec![(-1, 0), (0, -1), (1, 0), (0, 1)];
        check_vec_pattern!(dir, sample);
    }

    #[test]
    fn specify_both_bounds() {
        let dir = neighbor4!((1, 0) in ..=1).collect::<Vec<_>>();
        let sample = vec![(0, 0), (1, 1)];
        check_vec_pattern!(dir, sample);
    }

    #[test]
    fn specify_each_bounds() {
        let dir = neighbor4!((1, 1) in 1.., ..2).collect::<Vec<_>>();
        let sample = vec![(1, 0), (2, 1)];
        check_vec_pattern!(dir, sample);
    }
}
