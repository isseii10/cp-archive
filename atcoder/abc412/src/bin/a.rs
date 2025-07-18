// -*- coding:utf-8-unix -*-

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }
    let ans = ab.iter().filter(|&(a, b)| a < b).count();
    println!("{}", ans);
}
