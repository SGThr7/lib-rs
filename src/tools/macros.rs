#[codesnip::entry]
#[allow(unused_macros)]
macro_rules! mat {
    () => {
        vec![]
    };
    ($e:expr; $n:expr) => {
        vec![$e; $n]
    };
    ($e:expr; $nhead:expr $(, $ntail:expr)*) => {
        vec![mat![$e; $($ntail),*]; $nhead]
    };
}
#[codesnip::entry("mat")]
#[allow(unused_imports)]
pub(crate) use mat;

#[codesnip::entry]
#[allow(unused_macros)]
macro_rules! dir4 {
    (($x:expr, $y:expr) in $range:expr) => {
        vec![
            ($x.wrapping_add(1), $y),
            ($x, $y.wrapping_add(1)),
            ($x.wrapping_sub(1), $y),
            ($x, $y.wrapping_sub(1)),
        ]
        .into_iter()
        .filter(|(x, y)| $range.contains(&x) && $range.contains(&y))
    };
}
#[codesnip::entry("dir4")]
#[allow(unused_imports)]
pub(crate) use dir4;

#[codesnip::entry]
#[allow(unused_macros)]
macro_rules! dir_around {
    (@1dim $x:expr, $range:expr) => {
        vec![($x as usize).wrapping_sub(1), $x, ($x as usize).wrapping_add(1)]
            .into_iter()
            .filter(|x| $range.contains(x))
    };
    (($x:expr, $y:expr) in ($rangex:expr, $rangey:expr)) => {
        dir_around!(@1dim $x, $rangex)
            .map(|x| core::iter::repeat(x).zip(dir_around!(@1dim $y, $rangey)).collect::<Vec<_>>())
            .flatten()
            .filter(|pos| pos != &($x, $y))
    };
    (($x:expr, $y:expr) in $range:expr) => {
        dir_around!(($x, $y) in ($range, $range))
    };
}
#[codesnip::entry("dir_around")]
#[allow(unused_imports)]
pub(crate) use dir_around;

#[codesnip::entry("chmax")]
#[allow(unused_macros)]
macro_rules! chmax {
    ($lhs:expr, $rhs:expr) => {
        $lhs = $lhs.max($rhs)
    };
}
#[codesnip::entry("chmax")]
#[allow(unused_imports)]
pub(crate) use chmax;

#[codesnip::entry("chmin")]
#[allow(unused_macros)]
macro_rules! chmin {
    ($lhs:expr, $rhs:expr) => {
        $lhs = $lhs.min($rhs)
    };
}
#[codesnip::entry("chmin")]
#[allow(unused_imports)]
pub(crate) use chmin;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mat() {
        let t = mat![3; 3, 3];
        assert_eq!(t.len(), 3);
        assert!(t.iter().all(|v| v.len() == 3));
        assert!(t.iter().all(|v| v.iter().all(|x| *x == 3)));
    }

    #[test]
    fn dir_around() {
        assert_eq!(
            vec![
                (0, 0),
                (0, 1),
                (0, 2),
                (1, 0),
                (1, 2),
                (2, 0),
                (2, 1),
                (2, 2)
            ],
            dir_around!((1, 1) in 0..10).collect::<Vec<_>>()
        );
        assert_eq!(
            vec![(1, 2), (2, 1), (2, 2)],
            dir_around!((1, 1) in 1..10).collect::<Vec<_>>()
        );
        assert_eq!(
            vec![(0, 1), (0, 2), (1, 2), (2, 1), (2, 2)],
            dir_around!((1, 1) in (0..10, 1..10)).collect::<Vec<_>>()
        );
    }

    #[test]
    fn chmax() {
        let mut a = 10;
        chmax!(a, 5);
        assert_eq!(a, 10);
        chmax!(a, 12);
        assert_eq!(a, 12);
    }

    #[test]
    fn chmin() {
        let mut a = 10;
        chmin!(a, 12);
        assert_eq!(a, 10);
        chmin!(a, 5);
        assert_eq!(a, 5);
    }
}
