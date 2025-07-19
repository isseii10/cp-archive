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
        a: [usize; n],
    }
    let m: usize = 998244353;
    // dp[i][j]: i番目まで見て、jになる時の場合の数
    let mut dp = vec![vec![0; 10]; n];
    dp[0][a[0]] = 1;

    for i in 1..n {
        for j in 0..10 {
            let nxt1 = (j + a[i]) % 10;
            let nxt2 = (j * a[i]) % 10;
            dp[i][nxt1] += dp[i - 1][j];
            dp[i][nxt1] %= m;
            dp[i][nxt2] += dp[i - 1][j];
            dp[i][nxt2] %= m;
        }
    }
    for k in 0..10 {
        println!("{}", dp[n - 1][k]);
    }
}
