#[codesnip::entry]
pub trait MinMax
where
    Self: Sized,
{
    fn min_max(self, other: Self) -> (Self, Self);
}

#[codesnip::entry("MinMax")]
impl<T: core::cmp::PartialOrd> MinMax for T {
    fn min_max(self, other: Self) -> (Self, Self) {
        if self > other {
            (other, self)
        } else {
            (self, other)
        }
    }
}
