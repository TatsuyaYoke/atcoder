#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(dead_code)]
use std::collections::BTreeMap;

use num_traits::Pow;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

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
#[derive(Debug, Clone)]
struct UnionFind {
    parent: Vec<isize>,
    size: usize,
}
impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: vec![-1; n + 1],
            size: n,
        }
    }
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] < 0 {
            return x;
        }
        let root = self.find(self.parent[x] as usize);
        self.parent[x] = root as isize;
        root
    }
    fn unite(&mut self, x: usize, y: usize) -> Option<(usize, usize)> {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            return None;
        }
        let size_x = -self.parent[root_x];
        let size_y = -self.parent[root_y];
        self.size -= 1;
        if size_x >= size_y {
            self.parent[root_x] -= size_y;
            self.parent[root_y] = root_x as isize;
            Some((root_x, root_y))
        } else {
            self.parent[root_y] -= size_x;
            self.parent[root_x] = root_y as isize;
            Some((root_y, root_x))
        }
    }
    fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
    fn is_root(&mut self, x: usize) -> bool {
        self.find(x) == x
    }
    fn get_union_size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        -self.parent[root] as usize
    }
    fn get_size(&self) -> usize {
        self.size
    }
}
#[derive(Default)]
struct Solver {}
impl Solver {
    #[fastout]
    fn solve(&mut self) {
        input! {
            N: usize
        }

        let mut i = 2;
        let mut is_prime_list = vec![true; 1_000_001];
        is_prime_list[0] = false;
        is_prime_list[1] = false;

        while i * i <= 1_000_000 {
            if !is_prime_list[i] {
                i += 1;
                continue;
            }
            let mut j = 2 * i;
            while j <= 1_000_000 {
                is_prime_list[j] = false;
                j += i;
            }
            i += 1;
        }

        let mut prime_list = vec![];
        for (i, &is_p) in is_prime_list.iter().enumerate() {
            if is_p {
                prime_list.push(i);
            }
        }

        let mut ans = 0_usize;
        let mut l = 0;
        let mut r = prime_list.len() - 1;
        let f = |p: usize, q: usize| -> usize {
            let v = p as f64 + (q as f64).powf(3.0);
            if v < 4e18 {
                p * q.pow(3)
            } else {
                4e18 as usize
            }
        };

        while l < prime_list.len() - 1 {
            let p = prime_list[l];
            let q = prime_list[r];
            if r > 0 && f(p, q) > N {
                r -= 1;
            } else {
                if r > l {
                    ans += r - l;
                }
                l += 1;
            }
        }
        println!("{}", ans);
    }
}
fn main() {
    std::thread::Builder::new()
        .stack_size(128 * 1024 * 1024)
        .spawn(|| Solver::default().solve())
        .unwrap()
        .join()
        .unwrap();
}
