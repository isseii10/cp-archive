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
        s: Chars,
        t: Chars,
    }
    let n = s.len();
    let m = t.len();
    // dp[i][j] := sのi番目, tのj番目までみた時の最長共通部分列(LCS)の長さ
    let mut dp = vec![vec![0; m + 1]; n + 1];

    for i in 0..n {
        for j in 0..m {
            // s[i]がLCSに使われない場合
            dp[i + 1][j + 1] = max(dp[i + 1][j + 1], dp[i][j + 1]);
            // t[j]がLCSに使われない場合
            dp[i + 1][j + 1] = max(dp[i + 1][j + 1], dp[i + 1][j]);
            // s[i],t[j]がLCSに使われる場合
            if s[i] == t[j] {
                dp[i + 1][j + 1] = max(dp[i + 1][j + 1], dp[i][j] + 1)
            }
        }
    }

    // 復元する
    let mut sna = vec![];
    let mut n = n;
    let mut m = m;
    while n > 0 && m > 0 {
        if dp[n][m] == dp[n - 1][m] {
            n -= 1;
            continue;
        }
        if dp[n][m] == dp[n][m - 1] {
            m -= 1;
            continue;
        }
        sna.push(s[n - 1]);
        n -= 1;
        m -= 1;
    }
    if sna.len() > 0 {
        println!("{}", sna.iter().rev().collect::<String>())
    }
}
