// -*- coding:utf-8-unix -*-
#[allow(unused_imports)]
use proconio::{input, marker::Chars};

fn main() {
    input! {
        a: usize,
    }
    if 400 % a != 0 {
        println!("-1");
        return;
    }
    println!("{}", 400 / a);
}
