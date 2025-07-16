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

    // dp[i][j]: i番目までの要素を使って、総和がjになる時の場合の数
    let mut dp = vec![vec![0; 5050]; n + 1];
    dp[0][0] = 1;

    for i in 0..n {
        for j in 0..=5000 {
            // bb[i]を使う場合
            if bb[i] + j <= 5000 {
                dp[i + 1][bb[i] + j] += dp[i][j];
                dp[i + 1][bb[i] + j] %= m;
            }
            // bb[i]を使わない場合
            dp[i + 1][j] += dp[i][j];
            dp[i + 1][j] %= m;
        }
    }

    let mut ans = 0;
    for i in 0..n {
        for j in 0..=5000 {
            if j + bb[i] <= aa[i] {
                ans += dp[i][j];
                ans %= m;
            }
        }
    }

    println!("{}", ans);
}
