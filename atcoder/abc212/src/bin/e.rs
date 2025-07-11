// -*- coding:utf-8-unix -*-

#[allow(unused_imports)]
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        uv: [(usize, usize); m],
    }
    let _mod: usize = 998244353;
    // graphは使えない辺
    let mut graph = vec![vec![]; n];
    for i in 0..n {
        graph[i].push(i);
    }
    for (u, v) in uv {
        graph[u - 1].push(v - 1);
        graph[v - 1].push(u - 1);
    }

    // dp[i][j]: i日目にjにいる場合の数
    let mut dp = vec![vec![0; n]; k + 1];
    dp[0][0] = 1;
    for now in 0..k {
        // println!("now: {}", now);
        let mut total = 0;
        for v in 0..n {
            total = (total + dp[now][v]) % _mod
        }
        // 一見O(n^2)の計算量に見えるが、実際はO(m)で済む(ならし計算量)
        for to in 0..n {
            dp[now + 1][to] = total;
            for from in graph[to].iter() {
                dp[now + 1][to] = (dp[now + 1][to] + _mod - dp[now][*from]) % _mod;
            }
        }
    }

    println!("{}", dp[k][0]);
}
