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
        x: usize,
        y: usize,
        ab: [(usize, usize); n],
    }
    // dp[i][j][k]: i番目まで見て、たこ焼きjこ、たい焼きk個買う時の最小個数
    let mut dp = vec![vec![vec![MAX; 310]; 310]; n + 1];
    dp[0][0][0] = 0;
    for i in 0..n {
        let (a, b) = ab[i];
        for j in 0..=300 {
            for k in 0..=300 {
                if dp[i][j][k] == MAX {
                    continue;
                }
                dp[i + 1][j][k] = min(dp[i + 1][j][k], dp[i][j][k]);
                dp[i + 1][min(300, j + a)][min(300, k + b)] =
                    min(dp[i + 1][min(300, j + a)][min(300, k + b)], dp[i][j][k] + 1);
            }
        }
    }
    let mut ans = MAX;
    for i in 1..=n {
        for j in x..=300 {
            for k in y..=300 {
                ans = min(ans, dp[i][j][k])
            }
        }
    }
    if ans != MAX {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}
