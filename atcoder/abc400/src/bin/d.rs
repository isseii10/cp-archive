// -*- coding:utf-8-unix -*-

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: [usize;n],
    }
    // aを1-indexedにする操作
    // let aa: Vec<usize> = (0..n + 1)
    //     .map(|x| if x == 0 { 0 } else { a[x - 1] })
    //     .collect();
    let mut aa = vec![0];
    aa.extend_from_slice(&a[..n]);

    let mut ans = 0;
    for i in 1..=n {
        let mut sum = 0;
        for j in (i..=n).step_by(i) {
            sum += a[j];
        }
        if sum % 2 == a[i] {
            ans += 1
        }
    }
    println!("{}", ans)
}
