#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::comparison_chain)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

pub mod macros {
    #[macro_export]
    macro_rules! max {
        ($x: expr) => ($x);
        ($x: expr, $( $y: expr ),+) => {
            std::cmp::max($x, max!($( $y ),+))
        }
    }
    #[macro_export]
    macro_rules! min {
        ($x: expr) => ($x);
        ($x: expr, $( $y: expr ),+) => {
            std::cmp::min($x, min!($( $y ),+))
        }
    }
}

#[fastout]
fn main() {
    input! {
        N:usize
    }

    println!("{}", N);
    for i in 1..=N {
        println!("{} {}", i, i % N + 1);
    }
}
