use proconio::input;
use sgthr7_lib_unionfind::UnionFind;

fn main() {
    input!(n: usize, query: [(u8, usize, usize)]);

    let mut uf = UnionFind::new(n);
    for (mode, a, b) in query {
        match mode {
            0 => uf.union(a, b),
            1 => {
                if uf.equiv(a, b) {
                    println!("Yes")
                } else {
                    println!("No")
                }
            }
            _ => unreachable!(),
        }
    }
}
