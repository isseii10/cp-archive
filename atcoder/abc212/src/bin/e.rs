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

type Mint = ac_library::ModInt998244353;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        uv: [(usize, usize); m],
    }
    // non_edgesは使えない辺
    let mut non_edges = vec![vec![]; n];
    for i in 0..n {
        non_edges[i].push(i);
    }
    for (u, v) in uv {
        non_edges[u - 1].push(v - 1);
        non_edges[v - 1].push(u - 1);
    }

    // dp[i][j]: i日目にjにいる場合の数
    let mut dp = vec![vec![Mint::new(0); n]; k + 1];
    dp[0][0] = Mint::new(1);
    for i in 0..k {
        let mut total = Mint::new(0);
        for v in 0..n {
            total += dp[i][v]
        }
        // 一見O(nm)の計算量に見えるが、たかだか辺の数しか処理しないので実際はO(m)で済む
        for v in 0..n {
            let mut to_v = total;
            for &u in non_edges[v].iter() {
                // uからvへはいけないので引く
                to_v -= dp[i][u];
            }
            dp[i + 1][v] = to_v;
        }
    }

    println!("{}", dp[k][0]);
}
