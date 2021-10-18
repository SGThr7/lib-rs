#[codesnip::entry("Modulo")]
pub trait Modulo {
    type Set: core::ops::Rem<Output = Self::Set>;
    const MOD: Self::Set;
}

#[codesnip::entry("Modulo1000000007")]
pub struct Modulo1000000007;
#[codesnip::entry("Modulo1000000007", include("Modulo"))]
impl Modulo for Modulo1000000007 {
    type Set = usize;
    const MOD: Self::Set = 1_000_000_007;
}

#[codesnip::entry("Modulo998244353")]
pub struct Modulo998244353;
#[codesnip::entry("Modulo998244353", include("Modulo"))]
impl Modulo for Modulo998244353 {
    type Set = usize;
    const MOD: Self::Set = 998_244_353;
}
