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
        k: usize,
        h: [i64; n],
    }
    // dp[i]:= i番目の足場にいる時の最小コスト
    let inf: i64 = 1_000_000_000_000_000_000;
    let mut dp = vec![inf; n];
    dp[0] = 0;
    for i in 0..n {
        for j in 1..=k {
            if i + j < n {
                dp[i + j] = min(dp[i + j], dp[i] + (h[i] - h[i + j]).abs());
            }
        }
    }
    println!("{}", dp[n - 1]);
}
