use super::{ModInt, ModIntFactMap, Modulo};

// 1e9 + 7
#[codesnip::entry("Modulo1e9_7", include("define_modulo"))]
define_modulo! { Modulo1e9_7: usize = 1_000_000_007 }

#[codesnip::entry("ModInt1e9_7", include("ModInt", "Modulo1e9_7"))]
pub type ModInt1e9_7 = ModInt<Modulo1e9_7>;

#[codesnip::entry("ModIntFactMap1e9_7", include("ModIntFactMap", "ModInt1e9_7"))]
pub type FactMap1e9_7 = ModIntFactMap<ModInt1e9_7>;

// 998244353
#[codesnip::entry("Modulo998244353", include("define_modulo"))]
define_modulo! { Modulo998244353: usize = 998_244_353 }

#[codesnip::entry("ModInt998244353", include("ModInt", "Modulo998244353"))]
pub type ModInt998244353 = ModInt<Modulo998244353>;

#[codesnip::entry("ModIntFactMap998244353", include("ModIntFactMap", "ModInt998244353"))]
pub type FactMap998244353 = ModIntFactMap<ModInt998244353>;
