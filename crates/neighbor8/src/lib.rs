#[macro_export]
macro_rules! neighbor8 {
    ($x:expr, $y:expr) => { $crate::neighbor8!(($x, $y)) };
    (($x:expr, $y:expr)) => { $crate::neighbor8!(($x, $y): usize) };
    (($x:expr, $y:expr): $t:tt) => { $crate::neighbor8!(($x, $y): $t in ..) };
    (($x:expr, $y:expr) in $range:expr) => { $crate::neighbor8!(($x, $y): usize in $range, $range) };
    (($x:expr, $y:expr) in $range_x:expr, $range_y:expr) => { $crate::neighbor8!(($x, $y): usize in $range_x, $range_y) };
    (($x:expr, $y:expr): $t:tt in $range:expr) => { $crate::neighbor8!(($x, $y): $t in $range, $range) };
    (($x:expr, $y:expr): $t:tt in $range_x:expr, $range_y:expr) => {{
        use core::convert::TryInto;
        use core::ops::RangeBounds;
        let x: $t = $x.try_into().unwrap();
        let y: $t = $y.try_into().unwrap();
        let ys = vec![y.checked_sub(1), Some(y), y.checked_add(1)];
        vec![x.checked_sub(1), Some(x), x.checked_add(1)]
            .into_iter()
            .map(move |x| core::iter::repeat(x).zip(ys.clone().into_iter()))
            .flatten()
            .filter_map(move |p| match p {
                (Some(x2), Some(y2))
                    if RangeBounds::<$t>::contains(&$range_x, &x2)
                        && RangeBounds::<$t>::contains(&$range_y, &y2)
                        && !(x2 == x && y2 == y) =>
                {
                    Some((x2, y2))
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
    fn neighbor8() {
        let dir = neighbor8!((0,0): isize in ..=0, 0..).collect::<Vec<_>>();
        let sample = vec![(-1, 0), (-1, 1), (0, 1)];
        check_vec_pattern!(dir, sample);
    }

    #[test]
    fn omit_type() {
        let dir = neighbor8!(0, 0).collect::<Vec<_>>();
        let sample = vec![(1, 0), (1, 1), (0, 1)];
        check_vec_pattern!(dir, sample);
    }

    #[test]
    fn specify_type() {
        let dir = neighbor8!((0, 0): isize).collect::<Vec<_>>();
        let sample = vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        check_vec_pattern!(dir, sample);
    }

    #[test]
    fn specify_both_bounds() {
        let dir = neighbor8!((1, 0) in ..=1).collect::<Vec<_>>();
        let sample = vec![(0, 1), (1, 1), (0, 0)];
        check_vec_pattern!(dir, sample);
    }

    #[test]
    fn specify_each_bounds() {
        let dir = neighbor8!((1, 1) in 1.., ..2).collect::<Vec<_>>();
        let sample = vec![(1, 0), (2, 0), (2, 1)];
        check_vec_pattern!(dir, sample);
    }
}
