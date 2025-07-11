// -*- coding:utf-8-unix -*-

use num::abs;
#[allow(unused_imports)]
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,

        mut a: [isize; n],
        b: [isize; m],
    }
    a.sort();
    let mut ans = 1_000_000_001;
    for i in 0..m {
        let left = a.partition_point(|&x| x < b[i]);
        if left != n {
            ans = ans.min(abs(a[left] - b[i]));
        }
        if left != 0 {
            ans = ans.min(abs(a[left - 1] - b[i]));
        }
    }
    println!("{}", ans);
}
