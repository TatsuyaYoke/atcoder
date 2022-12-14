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
            L: usize,
            R: usize
        }

        let MOD = 1_000_000_007;
        let L_order = L.to_string().len();
        let R_order = R.to_string().len();

        let mut ans = 0;
        for i in L_order..=R_order {
            let l = mod_power(10, i - 1, MOD);
            let r = mod_power(10, i, MOD) - 1;
            let mut sum = 1;
            sum = sum * (MOD + r - l + 1) % MOD;
            sum = ((sum % MOD) * (l + r) % MOD) % MOD;
            sum = sum * mod_power(2, MOD - 2, MOD) % MOD;
            sum = (sum * i) % MOD;
            ans = (ans + sum) % MOD;
        }

        let l = mod_power(10, L_order - 1, MOD);
        let mut sum = 1;
        sum = sum * (MOD + L % MOD - l) % MOD;
        sum = ((sum % MOD) * (MOD + l + L % MOD - 1) % MOD) % MOD;
        sum = sum * mod_power(2, MOD - 2, MOD) % MOD;
        sum = (sum * L_order) % MOD;
        ans = (MOD + ans - sum) % MOD;

        let r = mod_power(10, R_order, MOD) - 1;
        let mut sum = 1;
        sum = sum * (MOD + r - R % MOD) % MOD;
        sum = ((sum % MOD) * (R % MOD + r + 1) % MOD) % MOD;
        sum = (sum * R_order) % MOD;
        sum = sum * mod_power(2, MOD - 2, MOD) % MOD;
        ans = (MOD + ans - sum) % MOD;

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

fn mod_power(a: usize, b: usize, m: usize) -> usize {
    let mut p = a;
    let mut ans = 1;
    let mut n = b;

    while n > 0 {
        if n & 1 == 1 {
            ans = ans * p % m;
        }
        p = p * p % m;
        n >>= 1;
    }
    ans
}
