#[codesnip::entry("Prime")]
pub use prime_map_impl::Prime;

#[codesnip::entry("Prime")]
mod prime_map_impl {
    use core::iter::{self, FusedIterator};

    #[derive(Clone, Debug)]
    pub struct Prime {
        is_prime: Vec<bool>,
    }

    impl Prime {
        const MIN_SIZE: usize = 2;

        /// Creates a `Prime` for primes less than or equal to `n`.
        /// Minimum length is `2`.
        pub fn new(n: usize) -> Self {
            let mut ret = Self {
                is_prime: vec![false, false],
            };
            ret.extend_to(n.max(Self::MIN_SIZE));
            ret
        }

        /// Returns `true` if the value is a prime.
        ///
        /// # Examples
        ///
        /// ```
        /// # use lib_rust::math::prime::Prime;
        /// let map = Prime::new(15);
        ///
        /// let primes = vec![2, 3, 5, 7, 11, 13];
        /// assert!(primes.iter().all(|&i| map.check_prime(i)));
        ///
        /// let not_primes = vec![0, 1, 4, 6, 8, 9, 12, 14, 15];
        /// assert!(not_primes.iter().all(|&i| !map.check_prime(i)));
        /// ```
        ///
        /// # Panics
        ///
        /// Panics if the value is larger than `self.len()`.
        pub fn check_prime(&self, x: usize) -> bool {
            if x >= self.len() {
                panic!(
                    "number out of bounds: the {} is 1 but the number is {}",
                    self.len(),
                    x
                );
            }
            self.is_prime[x]
        }

        /// Returns the maximum number of handle range in the `Prime`.
        pub fn len(&self) -> usize {
            self.is_prime.len()
        }

        /// Extends the handle range of the `Prime` by `additional`.
        ///
        /// # Examples
        ///
        /// ```
        /// # use lib_rust::math::prime::Prime;
        /// let mut map = Prime::new(3);
        /// map.extend(2);
        ///
        /// // include zero
        /// assert_eq!(map.len(), 5 + 1);
        ///
        /// assert!(!map.check_prime(4));
        /// assert!(map.check_prime(5));
        /// ```
        pub fn extend(&mut self, additional: usize) {
            let old_len = self.len();
            let new_len = old_len + additional;
            self.is_prime.extend(iter::repeat(true).take(additional));
            for i in iter::once(2).chain((3..new_len).step_by(2)) {
                if self.is_prime[i] {
                    // Same as `(old_len / i).ceil() * i`;
                    let start = ((old_len + i - 1) / i * i).max(i + i);
                    (start..new_len)
                        .step_by(i)
                        .for_each(|k| self.is_prime[k] = false);
                }
            }
        }

        /// Extends the handle range of the `Prime` to `x`.
        ///
        /// # Examples
        ///
        /// ```
        /// # use lib_rust::math::prime::Prime;
        /// let mut map = Prime::new(3);
        /// map.extend_to(5);
        ///
        /// // include zero
        /// assert_eq!(map.len(), 5 + 1);
        ///
        /// assert!(!map.check_prime(4));
        /// assert!(map.check_prime(5));
        /// ```
        pub fn extend_to(&mut self, x: usize) {
            let additional = (x + 1).saturating_sub(self.len());
            self.extend(additional)
        }

        /// Gets an iterator over the prime factors of `x`.
        ///
        /// # Examples
        ///
        /// ```
        /// # use lib_rust::math::prime::Prime;
        /// let mut factors: Vec<usize> = Prime::prime_factor(57).collect();
        /// factors.sort();
        ///
        /// assert_eq!(factors, vec![3, 19]);
        /// ```
        pub fn prime_factor(x: usize) -> PrimeFactor {
            let map = Self::new(x);
            PrimeFactor::new(x, map)
        }
    }

    pub struct PrimeFactor {
        x: usize,
        p: usize,
        map: Prime,
    }

    impl PrimeFactor {
        pub fn new(x: usize, map: Prime) -> Self {
            Self { x, map, p: 2 }
        }
    }

    impl Iterator for PrimeFactor {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.x <= 1 || !(2..self.map.len()).contains(&self.p) {
                None
            } else if self.x % self.p == 0 {
                self.x /= self.p;
                Some(self.p)
            } else {
                if let Some(p) = (self.p + 1..self.map.len())
                    .find(|&i| self.map.check_prime(i) && self.x % i == 0)
                {
                    self.p = p;
                    self.x /= p;
                    Some(p)
                } else {
                    self.p = self.map.len();
                    None
                }
            }
        }
    }

    impl FusedIterator for PrimeFactor {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_prime() {
        let map = Prime::new(30);
        dbg!(&map);
        let ans = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
        assert!((0..30).all(|i| !(ans.contains(&i) ^ map.check_prime(i))));
    }

    #[test]
    fn prime_factor() {
        let p = 433_500;
        let mut prime_factors = Prime::prime_factor(p).collect::<Vec<_>>();
        prime_factors.sort();
        assert_eq!(prime_factors, vec![2, 2, 3, 5, 5, 5, 17, 17]);
    }
}
