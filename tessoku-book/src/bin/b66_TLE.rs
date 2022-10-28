#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::comparison_chain)]
#![allow(dead_code)]
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
#[macro_export]
macro_rules! abs {
    ($x: expr) => {
        if $x < 0_isize {
            $x * (-1)
        } else {
            $x
        }
    };
}
#[macro_export]
macro_rules! absf {
    ($x: expr) => {
        if $x < 0.0 {
            $x * (-1.0)
        } else {
            $x
        }
    };
}

#[derive(Debug, Clone)]
struct UnionFind {
    parent: Vec<isize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: vec![-1; n + 1],
            size: vec![1; n + 1],
        }
    }

    fn find(&self, x: usize) -> usize {
        if self.parent[x] == -1 {
            return x;
        }
        self.find(self.parent[x] as usize)
    }

    fn unite(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            return;
        }
        if self.size[root_x] < self.size[root_y] {
            self.parent[root_x] = root_y as isize;
            self.size[root_y] += self.size[root_x];
        } else {
            self.parent[root_y] = root_x as isize;
            self.size[root_x] += self.size[root_y];
        }
    }

    fn is_same(&self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}

#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        AB: [(usize, usize); M],
        Q: usize,
    }
    let mut Query = vec![];
    for _ in 0..Q {
        input! {
            t: usize
        }
        if t == 1 {
            input! {
                x: usize
            }
            Query.push((t, x, 0))
        } else {
            input! {
                u: usize,
                v: usize
            }
            Query.push((t, u, v))
        }
    }

    let mut connected = vec![true; M];
    let mut uf = UnionFind::new(N);
    for &(a, b) in &AB {
        uf.unite(a, b);
    }

    for &q in &Query {
        match q {
            (1, x, _) => {
                uf = UnionFind::new(N);
                connected[x - 1] = false;
                for (&(a, b), is_connected) in AB.iter().zip(&connected) {
                    if *is_connected {
                        uf.unite(a, b);
                    }
                }
            }
            (2, u, v) => {
                if uf.is_same(u, v) {
                    println!("Yes");
                } else {
                    println!("No");
                }
            }
            (_, _, _) => unreachable!(),
        }
    }
}
