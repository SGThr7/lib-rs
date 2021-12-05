#[cfg_attr(nightly, codesnip::entry)]
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

#[cfg_attr(nightly, codesnip::entry)]
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

#[cfg(test)]
mod tests {
    #[test]
    fn max() {
        assert_eq!(max!(2, 1, 4, 7, 4, 8, 3, 6, 4, 7), 8);
    }

    #[test]
    fn min() {
        assert_eq!(min!(2, 1, 4, 7, 4, 8, 3, 6, 4, 7), 1);
    }
}
