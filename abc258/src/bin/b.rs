#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
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
            N: usize,
            A: [Chars; N]
        }

        let mut P = vec![vec![0; N]; N];

        for i in 0..N {
            for j in 0..N {
                P[i][j] = A[i][j] as usize - '0' as usize;
            }
        }
        let mut ans = 0;
        for i in 0..N {
            for j in 0..N {
                let mut max = 0;
                let mut v = 0;
                let mut p = 1;
                for k in 0..N {
                    let x1 = (j + k) % N;
                    let y1 = i;
                    v += P[y1][x1] * p;
                    p *= 10;
                }
                max = max!(max, v);
                v = 0;
                p = 1;
                for k in 0..N {
                    let x1 = (N + j - k) % N;
                    let y1 = i;
                    v += P[y1][x1] * p;
                    p *= 10;
                }
                max = max!(max, v);
                v = 0;
                p = 1;
                for k in 0..N {
                    let x1 = j;
                    let y1 = (i + k) % N;
                    v += P[y1][x1] * p;
                    p *= 10;
                }
                max = max!(max, v);
                v = 0;
                p = 1;
                for k in 0..N {
                    let x1 = j;
                    let y1 = (N + i - k) % N;
                    v += P[y1][x1] * p;
                    p *= 10;
                }
                max = max!(max, v);
                v = 0;
                p = 1;
                for k in 0..N {
                    let x1 = (j + k) % N;
                    let y1 = (i + k) % N;
                    v += P[y1][x1] * p;
                    p *= 10;
                }
                max = max!(max, v);
                v = 0;
                p = 1;
                for k in 0..N {
                    let x1 = (N + j - k) % N;
                    let y1 = (N + i - k) % N;
                    v += P[y1][x1] * p;
                    p *= 10;
                }
                max = max!(max, v);
                v = 0;
                p = 1;
                for k in 0..N {
                    let x1 = (N + j - k) % N;
                    let y1 = (i + k) % N;
                    v += P[y1][x1] * p;
                    p *= 10;
                }
                max = max!(max, v);
                v = 0;
                p = 1;
                for k in 0..N {
                    let x1 = (j + k) % N;
                    let y1 = (N + i - k) % N;
                    v += P[y1][x1] * p;
                    p *= 10;
                }
                max = max!(max, v);
                ans = max!(ans, max);
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
