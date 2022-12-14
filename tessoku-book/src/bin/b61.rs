#![allow(non_snake_case)]
#![allow(unused_imports)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        AB: [(usize, usize); M]
    }

    let mut G = vec![vec![]; N + 1];

    for &(A, B) in &AB {
        G[A].push(B);
        G[B].push(A);
    }

    let mut ans = 1;
    for i in 2..=N {
        if G[i].len() > G[ans].len() {
            ans = i;
        }
    }
    println!("{:?}", ans);
}
