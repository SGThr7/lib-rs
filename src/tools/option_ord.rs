#[codesnip::entry]
pub fn option_min<T: Ord>(v1: Option<T>, v2: Option<T>) -> Option<T> {
    match (v1, v2) {
        (None, None) => None,
        (None, Some(v2)) => Some(v2),
        (Some(v1), None) => Some(v1),
        (Some(v1), Some(v2)) => Some(v1.min(v2)),
    }
}

#[codesnip::entry]
pub fn option_max<T: Ord>(v1: Option<T>, v2: Option<T>) -> Option<T> {
    v1.max(v2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn option_min_test() {
        assert_eq!(Option::<i32>::None, option_min(None, None));
        let pattern = vec![
            (None, Some(1)),
            (Some(1), None),
            (Some(1), Some(2)),
            (Some(2), Some(1)),
            (Some(1), Some(1)),
        ];
        for (a, b) in pattern {
            assert_eq!(Some(1), option_min(a, b));
        }
    }

    #[test]
    fn option_max_test() {
        assert_eq!(Option::<i32>::None, option_max(None, None));
        let pattern = vec![
            (None, Some(2)),
            (Some(2), None),
            (Some(1), Some(2)),
            (Some(2), Some(1)),
            (Some(2), Some(2)),
        ];
        for (a, b) in pattern {
            assert_eq!(Some(2), option_max(a, b));
        }
    }
}
