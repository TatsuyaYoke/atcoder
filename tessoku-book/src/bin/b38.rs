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
        S: Chars
    }

    let mut ret1 = vec![1; N];
    let mut ret2 = vec![1; N];

    for (i, &s) in S.iter().enumerate() {
        if s == 'A' {
            ret1[i + 1] = ret1[i] + 1;
        }
    }
    for (i, &s) in S.iter().enumerate().rev() {
        if s == 'B' {
            ret2[i] = ret2[i + 1] + 1;
        }
    }
    let ans = ret1
        .iter()
        .zip(ret2.iter())
        .map(|(x, y)| x.max(y))
        .sum::<usize>();
    println!("{}", ans);
}
