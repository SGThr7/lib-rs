#[codesnip::entry]
pub fn matrix_modmut(
    lhs: &Vec<Vec<usize>>,
    rhs: &Vec<Vec<usize>>,
    modulo: usize,
) -> Vec<Vec<usize>> {
    let row_len = lhs.len();
    let col_len = rhs[0].len();
    let calc_len = rhs.len();
    assert!(lhs.iter().all(|v| v.len() == calc_len));
    let mut ret = vec![vec![0; row_len]; col_len];
    for r in 0..row_len {
        for i in 0..calc_len {
            for c in 0..col_len {
                ret[r][c] += (lhs[r][i] * rhs[i][c]) % modulo;
                ret[r][c] %= modulo;
            }
        }
    }
    ret
}

#[codesnip::entry(include("matrix_modmut"))]
pub fn matrix_modpow(
    base: Vec<Vec<usize>>,
    size: usize,
    mut exp: usize,
    modulo: usize,
) -> Vec<Vec<usize>> {
    assert_eq!(base.len(), size);
    assert!(base.iter().all(|v| v.len() == size));
    let id = {
        use core::iter::{once, repeat};
        let base = once(1).chain(repeat(0));
        (0..size)
            .map(|i| repeat(0).take(i).chain(base.clone()).take(size).collect())
            .collect()
    };
    if exp == 0 {
        id
    } else {
        let mut acc = id;
        let mut base = base;
        while exp > 1 {
            if (exp & 1) == 1 {
                acc = matrix_modmut(&acc, &base, modulo);
            }
            exp >>= 1;
            base = matrix_modmut(&base, &base, modulo);
        }
        matrix_modmut(&acc, &base, modulo)
    }
}
