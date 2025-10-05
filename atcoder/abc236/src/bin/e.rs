#[allow(unused_imports)]
use amplify::confinement::Collection;
use itertools::Itertools;
use ordered_float::OrderedFloat;
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
        a: [i64; n],
    }
    // 平均値

    // 中央値
    println!("{}", max_med(n, &a));
}

fn max_ave(n: usize, a: &[i64]) -> f64 {
    let mut ok = 0;
    let mut ng = a.iter().sum::<i64>() + 1;
    while ng - ok > 1 {
        let mid = (ok + ng) / 2;
        // mid以上の合計値を取れるか？
        let b = a.iter().map(|&v| v - mid).collect_vec();
        if check(n, &b) > 0 {
            ok = mid
        } else {
            ng = mid
        }
    }
    0.0
}
fn max_med(n: usize, a: &[i64]) -> i64 {
    let mut ok = 0;
    let mut ng = a.iter().sum::<i64>() + 1;
    while ng - ok > 1 {
        let mid = (ok + ng) / 2;
        // mid以上の中央値を取れるか？
        let b = a
            .iter()
            .map(|&v| if v - mid >= 0 { 1 } else { -1 })
            .collect_vec();
        if check(n, &b) > 0 {
            ok = mid
        } else {
            ng = mid
        }
    }
    return ok;
}

fn check(n: usize, b: &[i64]) -> i64 {
    let mut dp = vec![vec![0; 2]; n + 1];
    for i in 0..n {
        // 選ばない
        dp[i + 1][0] = max(dp[i + 1][0], dp[i][1]);
        // 選ぶ
        dp[i + 1][1] = max(dp[i + 1][1], dp[i][0] + b[i]);
        dp[i + 1][1] = max(dp[i + 1][1], dp[i][1] + b[i]);
    }

    return max(dp[n][0], dp[n][1]);
}
