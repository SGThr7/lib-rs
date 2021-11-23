#[codesnip::entry]
/// Compute `base^exp mod m`
pub fn mod_pow(base: usize, mut exp: usize, m: usize) -> usize {
    let mut acc = 1;
    let mut base = base % m;
    while exp > 0 {
        if (exp & 1) == 1 {
            acc = (acc * base) % m;
        }
        exp >>= 1;
        base = (base * base) % m;
    }
    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_cases() {
        for m in vec![2, 3, 4, 5, 12] {
            for base in 0..=10 {
                for exp in 0..=10 {
                    assert_eq!(
                        mod_pow(base, exp, m),
                        base.pow(exp as u32) % m,
                        "{}^{} % {}",
                        base,
                        exp,
                        m
                    )
                }
            }
        }
    }

    #[test]
    fn overflow() {
        let x = 1e18 as usize;
        let m = 1e9 as usize + 7;
        assert_eq!(mod_pow(x, x, m), 504853526)
    }
}
