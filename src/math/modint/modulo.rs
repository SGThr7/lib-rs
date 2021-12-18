#[codesnip::entry("Modulo")]
pub trait Modulo {
    type Set: core::ops::Rem<Output = Self::Set>;
    const MOD: Self::Set;
}

#[cfg_attr(nightly, codesnip::entry(include("Modulo")))]
#[macro_export]
macro_rules! define_modulo {
    ($name:ident: $set:ty = $mod:expr) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $name {}

        impl Modulo for $name {
            type Set = $set;
            const MOD: Self::Set = $mod;
        }
    };
}
