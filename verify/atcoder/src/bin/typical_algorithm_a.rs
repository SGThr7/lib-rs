use bisect::Bisect;
use proconio::input;

fn main() {
    input!(n: usize, k: usize, ar: [usize; n]);

    // let i = ar.partition_point(|ai| ai < &k);
    let i = Bisect::partition_point(&*ar, |ai| ai < &k);

    if i == n {
        println!("-1");
    } else {
        println!("{}", i);
    }
}
