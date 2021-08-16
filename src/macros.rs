#[codesnip::entry]
#[macro_export]
macro_rules! impl_traits {
    ($name:ident <$($bounds:path),*>) => {
        impl<T: $($bounds+)*> From<T> for $name<T> {
            fn from(v: T) -> Self {
                Self(v)
            }
        }
        impl<T: core::fmt::Display + $($bounds+)*> core::fmt::Display for $name<T> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}
