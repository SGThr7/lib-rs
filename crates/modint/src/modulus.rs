use crate::Set;

/// The trait is used to define modulo number for [`ModInt`].
///
/// [`MOD`] must be greater than 1.
///
/// [`ModInt`]: super::ModInt
/// [`MOD`]: Modulus::MOD
///
/// # Examples
///
/// ```
/// use modint::{ModInt, define_modulus};
///
/// define_modulus!(M = 7);
/// type MI = ModInt<M>;
///
/// let a: MI = 3.into();
/// let b: MI = 4.into();
///
/// assert_eq!(a * b, 5);
/// ```
pub trait Modulus: Copy {
    /// Modulus integer.
    const MOD: Set;
}

/// Convenient macro to define [`Modulus`].
///
/// # Examples
///
/// ```
/// use modint::{ModInt, define_modulus};
///
/// let x = 13;
///
/// define_modulus!(M1 = 7);
/// assert_eq!(ModInt::<M1>::from(x), 6);
///
/// define_modulus!(M2 = 5);
/// assert_eq!(ModInt::<M2>::from(x), 3);
/// ```
#[macro_export]
macro_rules! define_modulus {
    ($name:ident = $mod:expr) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $name {}

        impl $crate::Modulus for $name {
            const MOD: $crate::Set = $mod;
        }
    };
}

define_modulus! { Modulus1e9_7 = 1_000_000_007 }
define_modulus! { Modulus998_244_353 = 998_244_353 }
