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

    fn unite(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            return;
        }
        let size_x = -self.parent[root_x];
        let size_y = -self.parent[root_y];
        if size_x >= size_y {
            self.parent[root_x] -= size_y;
            self.parent[root_y] = root_x as isize;
        } else {
            self.parent[root_y] -= size_x;
            self.parent[root_x] = root_y as isize;
        }
        self.size -= 1;
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

#[fastout]
fn main() {
    input! {
        S: [Chars; 9]
    }

    let mut ans = 0;

    for i1 in 0..9 {
        for j1 in 0..9 {
            for i2 in 0..9 {
                for j2 in 0..9 {
                    if i2 > i1 && j2 >= j1 && S[i1][j1] == '#' && S[i2][j2] == '#' {
                        let di = i2 as isize - i1 as isize;
                        let dj = j2 as isize - j1 as isize;
                        let i3 = i2 as isize + dj;
                        let j3 = j2 as isize - di;
                        let i4 = i3 as isize - di;
                        let j4 = j3 as isize - dj;
                        if j3 >= 0
                            && i4 >= 0
                            && j4 >= 0
                            && i3 < 9
                            && S[i3 as usize][j3 as usize] == '#'
                            && S[i4 as usize][j4 as usize] == '#'
                        {
                            ans += 1;
                        }
                    }
                }
            }
        }
    }
    println!("{}", ans);
}
