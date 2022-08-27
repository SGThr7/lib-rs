use bisect::RangeBisect;
use itertools::Itertools;
use proconio::input;
use std::collections::BTreeSet;

fn main() {
    const STATUS_LEN: usize = 5;
    const LIM: usize = 1e9 as usize;
    input!(ar: [[usize; STATUS_LEN]]);

    let ng = (0..=LIM).partition_point(|x| {
        let status_set = ar
            .iter()
            .map(|status| {
                status
                    .iter()
                    .copied()
                    .enumerate()
                    .filter_map(|(i, v)| if v >= x { Some(i) } else { None })
                    .collect::<BTreeSet<_>>()
            })
            .collect::<BTreeSet<_>>();
        if status_set.len() < 3 {
            status_set
                .into_iter()
                .fold(BTreeSet::new(), |acc, set| &acc | &set)
                .len()
                == STATUS_LEN
        } else {
            status_set
                .iter()
                .tuple_combinations()
                .any(|(a, b, c)| (&(a | b) | c).len() == STATUS_LEN)
        }
    });
    // collect range (closed): 0..ng
    let ans = ng - 1;
    println!("{}", ans);
}
