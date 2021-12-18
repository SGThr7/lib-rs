use super::{ModInt, Modulo};

type Set = usize;

#[derive(Clone, Debug)]
pub struct ModIntFactMap<M> {
    len: usize,
    recip: Vec<ModInt<M>>,
    factorial: Vec<ModInt<M>>,
    recip_fact: Vec<ModInt<M>>,
}

impl<M> ModIntFactMap<M> {
    const MIN_LEN: usize = 1;

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn get_recip(&self, x: usize) -> Option<&ModInt<M>> {
        self.recip.get(x)
    }

    pub fn get_factorial(&self, x: usize) -> Option<&ModInt<M>> {
        self.factorial.get(x)
    }

    pub fn get_recip_fact(&self, x: usize) -> Option<&ModInt<M>> {
        self.recip_fact.get(x)
    }
}

impl<M: Clone + Modulo<Set = Set>> ModIntFactMap<M> {
    pub fn new(size: usize) -> Self {
        let len = size.max(Self::MIN_LEN) + 1;

        let mut recip = Vec::with_capacity(len);
        recip.push(ModInt::<M>::zero());
        recip.push(ModInt::<M>::one());

        let mut factorial = Vec::with_capacity(len);
        factorial.push(ModInt::<M>::one());
        factorial.push(ModInt::<M>::one());

        let mut recip_fact = Vec::with_capacity(len);
        recip_fact.push(ModInt::<M>::one());
        recip_fact.push(ModInt::<M>::one());

        let mut ret = Self {
            len: 2,
            recip,
            factorial,
            recip_fact,
        };
        ret.increase(len - 2);
        ret
    }

    pub fn increase(&mut self, additional: usize) {
        assert!(self.len() + additional - 1 <= M::MOD);
        self.recip.reserve_exact(additional);
        self.factorial.reserve_exact(additional);
        self.recip_fact.reserve_exact(additional);

        for d in 0..additional {
            let i = self.len() + d;
            // P = ⌊P/i⌋×i + P%i
            // 0 ≡ ⌊P/i⌋×i + P%i (mod P)
            // 0 ≡ ⌊P/i⌋ + (P%i)×i⁻¹ (mod P)
            // -⌊P/i⌋ ≡ (P%i)×i⁻¹ (mod P)
            // i⁻¹ ≡ -⌊P/i⌋ × (P%i)⁻¹ (mod P)
            // i⁻¹ ≡ -(P%i)⁻¹ × ⌊P/i⌋ (mod P)
            self.recip
                .push(-self.recip[M::MOD % i].clone() * (M::MOD / i));
            self.factorial.push(self.factorial[i - 1].clone() * i);
            self.recip_fact
                .push(self.recip_fact[i - 1].clone() * self.recip[i].clone());
        }
        self.len += additional;
    }

    pub fn increase_to(&mut self, x: usize) {
        if x >= self.len() {
            self.increase(x + 1 - self.len())
        }
    }

    pub fn recip(&mut self, x: usize) -> ModInt<M> {
        if x < self.len() {
            self.recip[x].clone()
        } else {
            self.increase_to(x);
            self.recip[x].clone()
        }
    }

    pub fn factorial(&mut self, x: usize) -> ModInt<M> {
        if x < self.len() {
            self.factorial[x].clone()
        } else {
            self.increase_to(x);
            self.factorial[x].clone()
        }
    }

    pub fn recip_fact(&mut self, x: usize) -> ModInt<M> {
        if x < self.len() {
            self.recip_fact[x].clone()
        } else {
            self.increase_to(x);
            self.recip_fact[x].clone()
        }
    }

    pub fn get_combination(&self, n: usize, k: usize) -> Option<ModInt<M>> {
        if n < k {
            None
        } else if k == 0 || n == k {
            Some(ModInt::<M>::one())
        } else {
            let ni = self.get_factorial(n);
            let kii = self.get_recip_fact(k);
            let nkii = self.get_recip_fact(n - k);
            if let (Some(ni), Some(kii), Some(nkii)) = (ni.cloned(), kii.cloned(), nkii.cloned()) {
                Some(ni * kii * nkii)
            } else {
                None
            }
        }
    }

    pub fn combination(&mut self, n: usize, k: usize) -> ModInt<M> {
        assert!(n >= k);
        let ni = self.factorial(n);
        let kii = self.recip_fact(k);
        let nkii = self.recip_fact(n - k);

        ni * kii * nkii
    }

    pub fn get_multi_choose(&self, n: usize, k: usize) -> Option<ModInt<M>> {
        self.get_combination(n + k - 1, k)
    }

    pub fn multi_choose(&mut self, n: usize, k: usize) -> ModInt<M> {
        self.combination(n + k - 1, k)
    }
}

#[cfg(test)]
mod tests {
    use super::{ModIntFactMap, Modulo};

    #[derive(Clone, Copy, Debug)]
    enum ModuloTest {}
    impl Modulo for ModuloTest {
        type Set = usize;
        const MOD: Self::Set = 13;
    }
    type Map = ModIntFactMap<ModuloTest>;

    fn fact(x: usize) -> usize {
        (1..=x).product()
    }

    fn comb(n: usize, k: usize) -> usize {
        fact(n) / fact(k) / fact(n - k)
    }

    #[test]
    fn combination() {
        let max = 11;
        let map = Map::new(max);

        for i in 0..=max {
            assert_eq!(
                map.get_combination(max, i).unwrap(),
                comb(max, i) % ModuloTest::MOD,
                "{}C{}",
                max,
                i
            );
        }
    }
}
