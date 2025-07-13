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

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        b: [usize; n],
    }
    let m = 998244353;
    let mut indices: Vec<usize> = (0..n).collect();
    indices.sort_by_key(|&i| a[i]);

    let aa: Vec<usize> = indices.iter().map(|&i| a[i]).collect();
    let bb: Vec<usize> = indices.iter().map(|&i| b[i]).collect();

    // dp[i][s]: i番目まで見て、0<=j<=iの中からB_jを選び、B_jの和がsになる場合の数
    let mut dp: Vec<Vec<usize>> = vec![vec![0; 5050]; n + 1];
    dp[0][0] = 1;

    for i in 0..n {
        for s in 0..=5000 {
            // 選ぶ
            if s + bb[i] <= 5000 {
                dp[i + 1][s + bb[i]] += dp[i][s];
                dp[i + 1][s + bb[i]] %= m;
            }
            // 選ばない
            dp[i + 1][s] += dp[i][s];
        }
    }

    let mut ans = 0;
    for i in 0..n {
        if aa[i] < bb[i] {
            continue;
        }
        for s in 0..=aa[i] - bb[i] {
            ans += dp[i][s];
            ans %= m;
        }
    }
    println!("{}", ans);
}
