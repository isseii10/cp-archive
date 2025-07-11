// -*- coding:utf-8-unix -*-

#[allow(unused_imports)]
use proconio::{input, marker::Chars};

fn main() {
    input! {
        _h: usize,
        _w: usize,
        n: usize,
        cards: [(usize, usize); n],
    }
    // 残るx軸のみ集める
    let mut xs: Vec<_> = cards.iter().map(|&(x, _)| x).collect();
    xs.sort();
    xs.dedup();
    // 残るy軸のみ集める
    let mut ys: Vec<_> = cards.iter().map(|&(_, y)| y).collect();
    ys.sort();
    ys.dedup();

    for (x, y) in cards {
        let ans_x = xs.partition_point(|&a| a < x) + 1;
        let ans_y = ys.partition_point(|&a| a < y) + 1;
        println!("{} {}", ans_x, ans_y);
    }
}
