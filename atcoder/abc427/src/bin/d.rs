#[allow(unused_imports)]
use amplify::confinement::Collection;
use num::abs;
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
// type Mint = ac_library::ModInt1000000007;

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        solve()
    }
}

fn solve() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        s: Chars,
        uv: [(usize, usize); m],
    }
    let mut g = vec![vec![]; n];
    for &(u, v) in uv.iter() {
        g[u - 1].push(v - 1);
    }
    // dp[i][j] := 残り行動回数がi回でjにいる時の勝者(1:A, -1:B)
    let mut dp = vec![vec![0; n]; 2 * k + 1];
    for j in 0..n {
        dp[0][j] = if s[j] == 'A' { 1 } else { -1 };
    }

    for i in 0..2 * k {
        for from in 0..n {
            for &to in g[from].iter() {
                dp[i + 1][from] += dp[i][to];
            }
            // +1(アリス勝ち), -1(ボブ勝ち)なので、次に行く頂点が全てアリス/ボブ勝ちの時はg[from]のサイズと絶対値が一致する
            // 逆に、一致しない場合は操作者が自分が勝つようにできる
            if abs(dp[i + 1][from]) == g[from].len() as i64 {
                dp[i + 1][from] /= g[from].len() as i64;
            } else {
                dp[i + 1][from] = if (i + 1) % 2 == 0 { 1 } else { -1 };
            }
        }
    }
    println!("{}", if dp[k][0] == 1 { "Alice" } else { "Bob" })
}
