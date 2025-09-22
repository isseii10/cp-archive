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
        w: usize,
        wv: [(usize, usize); n],
    }
    // dp[i][w] := i番目までの品物で、重さwの時の価値の最大値
    let mut dp = vec![vec![0; w + 1]; n + 1];

    for i in 0..n {
        let (ww, vv) = wv[i];
        for j in 0..=w {
            // i番目を選ばない場合
            dp[i + 1][j] = max(dp[i + 1][j], dp[i][j]);
            // i番目を選ぶ場合
            if j + ww <= w {
                dp[i + 1][j + ww] = max(dp[i + 1][j + ww], dp[i][j] + vv);
            }
        }
    }

    println!("{}", dp[n].iter().max().unwrap())
}
