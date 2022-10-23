#![allow(non_snake_case)]
#![allow(unused_imports)]

use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

#[fastout]
fn main() {
    input! {
        N: usize,
        S: usize,
        A: [usize; N]
    }

    let mut dp = vec![vec![false; S + 1]; N + 1];
    dp[0][0] = true;

    for i in 1..=N {
        let a = A[i - 1];
        for j in 0..=S {
            if j < a {
                dp[i][j] = dp[i - 1][j];
            } else {
                dp[i][j] = dp[i - 1][j] || dp[i - 1][j - a];
            }
        }
    }

    if dp[N][S] {
        println!("Yes");
    } else {
        println!("No");
    }
}
