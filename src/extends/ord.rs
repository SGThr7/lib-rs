pub trait OrdEx {
    fn clamp(self, min: Self, max: Self) -> Self;
}

impl<T: Ord> OrdEx for T {
    fn clamp(self, min: Self, max: Self) -> Self {
        assert!(min <= max);
        if self < min {
            min
        } else if self > max {
            max
        } else {
            self
        }
    }
}
