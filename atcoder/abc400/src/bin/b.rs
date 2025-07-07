// -*- coding:utf-8-unix -*-
#[allow(unused_imports)]
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut ans = 0;
    let mut ok = true;
    for i in 0..m + 1 {
        let mut ni = 1;
        for _j in 0..i {
            ni *= n;
            if ni > 1_000_000_000 {
                ok = false;
                break;
            }
        }
        ans += ni;
        if ans > 1_000_000_000 {
            ok = false;
            break;
        }
    }
    if ok {
        println!("{}", ans);
    } else {
        println!("inf")
    }
}
