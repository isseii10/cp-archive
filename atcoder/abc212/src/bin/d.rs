// -*- coding:utf-8-unix -*-

use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        q: usize,
    }
    let mut ans: Vec<i64> = vec![];
    let mut min_heap = BinaryHeap::new();
    let mut thresholds = 0;
    for _ in 0..q {
        input! {
            query: Chars,
        }
        if query[0] == '1' {
            input! {
            x: i64,
            }
            min_heap.push(Reverse(x - thresholds));
        } else if query[0] == '2' {
            input! {
                x: i64,
            }
            thresholds += x
        } else {
            let min = min_heap.pop();
            ans.push(min.unwrap().0 + thresholds);
        }
    }
    for v in ans {
        println!("{}", v)
    }
}
