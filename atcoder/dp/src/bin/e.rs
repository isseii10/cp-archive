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
    // dp[i][v] := i番目までの品物から、価値がvになるように選んだ時の重さの最小値
    let m = 100010;
    let inf = w + 1;
    let mut dp = vec![vec![inf; m]; n + 1];
    dp[0][0] = 0;
    for i in 0..n {
        let (ww, vv) = wv[i];
        for v in 0..m {
            // 選ぶ
            if dp[i][v] + ww <= w {
                dp[i + 1][v + vv] = min(dp[i + 1][v + vv], dp[i][v] + ww)
            }
            // 選ばない
            dp[i + 1][v] = min(dp[i + 1][v], dp[i][v])
        }
    }
    for i in (0..m).rev() {
        if dp[n][i] != inf {
            println!("{}", i);
            return;
        }
    }
}
