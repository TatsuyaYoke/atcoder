#![allow(non_snake_case)]
#![allow(unused_imports)]
use std::collections::BTreeSet;

use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        Q: usize,
        Query: [(usize, usize); Q]
    }

    let mut set = BTreeSet::new();
    for q in &Query {
        match q {
            (1, x) => {
                set.insert(x);
            }
            (2, x) => {
                set.remove(x);
            }
            (3, x) => {
                let vec: Vec<_> = set.iter().cloned().collect();
                let index = vec.lower_bound(&x);
                if index != vec.len() {
                    println!("{}", vec[index]);
                } else {
                    println!("-1");
                }
            }
            (_, _) => unreachable!(),
        }
    }
}
