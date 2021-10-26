#[codesnip::entry("Modulo")]
pub trait Modulo {
    type Set: core::ops::Rem<Output = Self::Set>;
    const MOD: Self::Set;
}

#[codesnip::entry("Modulo1e9_7")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Modulo1e9_7 {}
#[codesnip::entry("Modulo1e9_7", include("Modulo"))]
impl Modulo for Modulo1e9_7 {
    type Set = usize;
    const MOD: Self::Set = 1_000_000_007;
}

#[codesnip::entry("Modulo998244353")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Modulo998244353 {}
#[codesnip::entry("Modulo998244353", include("Modulo"))]
impl Modulo for Modulo998244353 {
    type Set = usize;
    const MOD: Self::Set = 998_244_353;
}
