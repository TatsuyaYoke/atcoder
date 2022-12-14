#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
use std::collections::VecDeque;

use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

pub mod macros {
    #[macro_export]
    macro_rules !max {
        ($x: expr) => ($x);
        ($x: expr, $($y: expr), +) => {
            std::cmp::max($x, max!($($y), +))
        }
    }
    #[macro_export]
    macro_rules !min {
        ($x: expr) => ($x);
        ($x: expr, $($y: expr), +) => {
            std::cmp::min($x, min!($($y), +))
        }
    }
}

#[fastout]
fn main() {
    input! {
        N:usize,
        M:usize,
        AB: [(usize, usize); M]
    }

    let mut G = vec![vec![]; N + 1];
    for &(A, B) in &AB {
        G[A].push(B);
        G[B].push(A);
    }
    let mut dist = vec![-1; N + 1];
    let mut Q = VecDeque::new();
    dist[1] = 0;
    Q.push_back(1);

    while !Q.is_empty() {
        let pos = Q.pop_front().unwrap();
        for &n in &G[pos] {
            if dist[n] == -1 {
                dist[n] = dist[pos] + 1;
                Q.push_back(n);
            }
        }
    }

    for a in &dist[1..] {
        println!("{}", a);
    }
}
