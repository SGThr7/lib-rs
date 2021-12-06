#[cfg_attr(nightly, codesnip::entry)]
#[macro_export]
macro_rules! chmax {
    ($lhs:expr, $rhs:expr) => {
        $lhs = $lhs.max($rhs)
    };
}

#[cfg_attr(nightly, codesnip::entry)]
#[macro_export]
macro_rules! chmin {
    ($lhs:expr, $rhs:expr) => {
        $lhs = $lhs.min($rhs)
    };
}

#[cfg(test)]
mod tests {
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
