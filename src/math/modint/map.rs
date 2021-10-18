use super::{ModInt, ModInt1000000007, ModInt998244353, Modulo};

#[codesnip::entry(
    "ModIntMap1000000007",
    include("define_modint_map", "ModInt1000000007")
)]
define_modint_map!(ModIntMap1000000007, ModInt1000000007);
#[codesnip::entry("ModIntMap998244353", include("define_modint_map", "ModInt998244353"))]
define_modint_map!(ModIntMap998244353, ModInt998244353);

#[codesnip::entry(include("ModInt", "Modulo"))]
#[allow(unused_macros)]
macro_rules! define_modint_map {
    ($name:ident, $modint:ident) => {
        pub struct $name {
            len: usize,
            inverse: Vec<$modint>,
            factorial: Vec<$modint>,
            inv_fac: Vec<$modint>,
        }

        impl $name {
            const MOD: usize = <$modint as ModInt>::Modulo::MOD;

            pub fn new(x: usize) -> Self {
                let len = 1.max(x) + 1;
                let mut inverse = Vec::with_capacity(len);
                inverse.push(0.into());
                inverse.push(1.into());
                let mut factorial = Vec::with_capacity(len);
                factorial.push(1.into());
                factorial.push(1.into());
                let mut inv_fac = Vec::with_capacity(len);
                inv_fac.push(1.into());
                inv_fac.push(1.into());

                let mut res = Self {
                    len: 2,
                    inverse,
                    factorial,
                    inv_fac,
                };
                res.increase(len - 2);
                res
            }

            pub fn increase(&mut self, len: usize) {
                self.inverse.reserve_exact(len);
                self.factorial.reserve_exact(len);
                self.inv_fac.reserve_exact(len);

                for i in self.len()..self.len() + len {
                    self.inverse
                        .push(Self::MOD - ((Self::MOD / i) * self.inverse[Self::MOD % i]));
                    self.factorial.push(self.factorial[i - 1] * i);
                    self.inv_fac.push(self.inv_fac[i - 1] * self.inverse[i]);
                }
                self.len += len;
            }

            pub fn increase_to(&mut self, x: usize) {
                if x >= self.len() {
                    self.increase(x + 1 - self.len())
                }
            }

            pub fn len(&self) -> usize {
                self.len
            }

            pub fn get_inverse(&self, x: usize) -> Option<&$modint> {
                self.inverse.get(x)
            }

            pub fn inverse(&mut self, x: usize) -> $modint {
                if x < self.len() {
                    self.inverse[x]
                } else {
                    self.increase_to(x);
                    self.inverse[x]
                }
            }

            pub fn get_factorial(&self, x: usize) -> Option<&$modint> {
                self.factorial.get(x)
            }

            pub fn factorial(&mut self, x: usize) -> $modint {
                if x < self.len() {
                    self.factorial[x]
                } else {
                    self.increase_to(x);
                    self.factorial[x]
                }
            }

            pub fn get_inverse_factorial(&self, x: usize) -> Option<&$modint> {
                self.inv_fac.get(x)
            }

            pub fn inverse_factorial(&mut self, x: usize) -> $modint {
                if x < self.len() {
                    self.inv_fac[x]
                } else {
                    self.increase_to(x);
                    self.inv_fac[x]
                }
            }

            pub fn get_combination(&self, n: usize, k: usize) -> Option<$modint> {
                if n < k {
                    return None;
                }

                let ni = self.get_factorial(n);
                let ki = self.get_inverse_factorial(k);
                let nki = self.get_inverse_factorial(n - k);

                if let (Some(ni), Some(ki), Some(nki)) = (ni, ki, nki) {
                    Some(ni * ki * nki)
                } else {
                    None
                }
            }

            pub fn combination(&mut self, n: usize, k: usize) -> $modint {
                assert!(n >= k);
                let ni = self.factorial(n);
                let ki = self.inverse_factorial(k);
                let nki = self.inverse_factorial(n - k);

                ni * ki * nki
            }

            pub fn get_multi_choose(&self, n: usize, k: usize) -> Option<$modint> {
                self.get_combination(n + k - 1, k)
            }

            pub fn multi_choose(&mut self, n: usize, k: usize) -> $modint {
                self.combination(n + k - 1, k)
            }
        }
    };
}
#[codesnip::entry("define_modint_map")]
#[allow(unused_imports)]
pub(crate) use define_modint_map;

#[cfg(test)]
mod tests {
    use super::*;

    fn fact(x: usize) -> usize {
        (1..=x).product()
    }

    fn comb(n: usize, k: usize) -> usize {
        fact(n) / fact(k) / fact(n - k)
    }

    macro_rules! map_tests {
        ($map:ident) => {
            #[test]
            fn combination() {
                let max = 10;
                let map = $map::new(max);

                for i in 0..=max {
                    assert_eq!(
                        super::comb(max, i),
                        map.get_combination(max, i).unwrap(),
                        "{}C{}",
                        max,
                        i
                    );
                }
            }
        };
    }

    mod map_1000000007 {
        use super::ModIntMap1000000007;
        map_tests!(ModIntMap1000000007);
    }

    mod map_998244353 {
        use super::ModIntMap998244353;
        map_tests!(ModIntMap998244353);
    }
}
