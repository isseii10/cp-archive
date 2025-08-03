#[allow(unused_imports)]
use amplify::confinement::Collection;
#[allow(unused_imports)]
use proconio::{input, marker::Chars};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::f64::consts::PI;
#[allow(unused_imports)]
use std::io::{self, Write};
#[allow(unused_imports)]
use std::isize::MIN;
#[allow(unused_imports)]
use std::usize::MAX;

#[allow(dead_code)]
type Map<K, V> = HashMap<K, V>;
#[allow(dead_code)]
type Set<T> = HashSet<T>;
#[allow(dead_code)]
type Deque<T> = VecDeque<T>;
#[allow(dead_code)]
type Heap<T> = BinaryHeap<T>;
#[allow(dead_code)]
type Mint = ac_library::ModInt998244353;

fn main() {
    input! {
        n: usize,
        pab: [(usize, usize, usize); n],
        q: usize,
    }
    // dp[i][j] := i番目の操作を行った後にテンションがjだった場合の最終的なテンションの値
    let mut dp = vec![vec![0; 1001]; n + 1];
    for j in 0..1001 {
        dp[n][j] = j
    }
    for i in (0..n).rev() {
        let (p, a, b) = pab[i];
        for j in 0..=1000 {
            let nxt_j = if j <= p {
                j + a // p<=500なのでj<=500. またa<=500なのでj+a<=1000
            } else {
                j.saturating_sub(b)
            };
            dp[i][j] = dp[i+1][nxt_j]
        
    }

    let mut acc_b = vec![0; n + 1];
    for i in 0..n {
        let (_, _, b) = pab[i];
        acc_b[i + 1] = acc_b[i] + b;
    }

    for _ in 0..q {
        input! {
            x: usize,
        }
        if x <= 1000 {
            println!("{}", dp[0][x]);
            continue;
        }
        // NOTE: partition_pointにはtrueからfalseに切り替わる条件を渡す。(false -> trueではだめ)
        // 今回は、初めて x - acc_b[idx] <= 1000 がtrueになる境界を知りたいので、
        // x - 1000 > acc_b[idx] と条件を逆にすればいい
        let idx = acc_b.partition_point(|&v| v < x - 1000);
        // println!("acc_b: {:?}", acc_b);
        // println!("idx: {}", idx);
        if idx == acc_b.len() {
            // println!("x:{}, acc_b[n]:{}", x, acc_b[n]);
            println!("{}", x - acc_b[n]);
            continue;
        }
        println!("{}", dp[idx][x - acc_b[idx]]);
    }
}
