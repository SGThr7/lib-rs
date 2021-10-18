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
