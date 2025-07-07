// -*- coding:utf-8-unix -*-

use proconio::{input, marker::Chars};
use std::collections::HashSet;

fn main() {
    input! {
        n: u64,
    }
    // let mut ans = 0;
    // for i in (1..).map(|x| 1 << x).take_while(|&x| x <= n) {
    //     // iは 2**1, 2**2, ..., 2**59
    //     let bb: u64 = n / i;
    //     let b = u64_floor_sqrt(bb);
    //     ans += (b + 1) / 2
    // }
    // 関数型で
    let ans: u64 = (1..)
        .map(|x| 1 << x) // 2^1, 2^2, ..., 2^59
        .take_while(|&i| i <= n)
        .map(|i| {
            let b = u64_floor_sqrt(n / i); // √(n / i)
            (b + 1) / 2
        })
        .sum();
    println!("{}", ans)
}

fn u64_floor_sqrt(n: u64) -> u64 {
    let tmp = (n as f64).sqrt() as u64;
    let tmp_m1 = tmp.saturating_sub(1);
    if tmp_m1 * (tmp_m1 + 2) < n {
        tmp
    } else {
        tmp_m1
    }
}
