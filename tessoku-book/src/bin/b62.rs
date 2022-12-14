#![allow(non_snake_case)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_traits::ops::checked;
use proconio::{fastout, input, marker::Chars};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        AB: [(usize, usize); M]
    }

    let mut G = vec![vec![]; N + 1];
    for &(A, B) in &AB {
        G[A].push(B);
        G[B].push(A);
    }

    let mut visited = vec![false; N + 1];
    let mut path = vec![1];
    let mut ans = vec![];
    dfs(1, &G, &mut visited, &mut path, &mut ans);
    println!("{}", ans.iter().join(" "));
}

fn dfs(
    pos: usize,
    G: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    path: &mut Vec<usize>,
    ans: &mut Vec<usize>,
) {
    if pos == visited.len() - 1 {
        *ans = path.clone();
        return;
    }
    visited[pos] = true;
    for &next in &G[pos] {
        if !visited[next] {
            path.push(next);
            dfs(next, G, visited, path, ans);
            path.pop();
        }
    }
}
