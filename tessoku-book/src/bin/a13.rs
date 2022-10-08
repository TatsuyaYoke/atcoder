use proconio::{fastout, input};

#[allow(non_snake_case)]
#[allow(clippy::needless_range_loop)]
#[fastout]

fn main() {
    input! {
        N: usize,
        K: usize,
        A: [usize; N]
    }

    let mut r = 0;
    let mut ans = 0;
    for l in 0..N - 1 {
        while r + 1 < N && A[r + 1] - A[l] <= K {
            r += 1;
        }
        ans += r - l;
    }

    println!("{:?}", ans);
}
