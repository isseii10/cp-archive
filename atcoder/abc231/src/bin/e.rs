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

const INF: usize = 1 << 60;

fn main() {
    input! {
        n: usize,
        mut x: usize,
        a: [usize; n],
    }
    // xを硬貨の進数みたいなものに変換する
    // (xがそれぞれの硬貨を何枚使って表現できるか)
    let mut xs = vec![0; n];
    for i in (0..n).rev() {
        xs[i] = x / a[i];
        x %= a[i];
    }
    // 何倍かを求めておく
    let mut d = vec![1; n];
    for i in 1..n {
        d[i] = a[i] / a[i - 1];
    }
    // 一番大きい硬貨は次の硬貨がないので倍率INFにしておく
    d.push(INF);

    // dp[i][j] := i桁目まで決めて、繰り下がりがj(0 or 1)の時の最小値
    let mut dp = vec![vec![INF; 2]; n + 1];
    dp[0][0] = 0; // 初期状態は繰り下がりなしで0

    for i in 0..n {
        // (1) i+1桁目を払う場合

        // i桁目で繰り下がりなし
        // xs[i]枚をピッタリ払う
        dp[i + 1][0] = min(dp[i + 1][0], dp[i][0] + xs[i]);
        // i桁目で繰り下がりあり
        // 繰り下がり分+1して、xs[i]+1枚をピッタリ払う
        dp[i + 1][0] = min(dp[i + 1][0], dp[i][1] + xs[i] + 1);

        // (2) i+1桁目を払わない場合(上位桁から繰り下げる)

        // i桁目で繰り下がりなし
        // d[i+1] - xs[i]枚
        dp[i + 1][1] = min(dp[i + 1][1], dp[i][0] + (d[i + 1] - xs[i]));
        // i桁目で繰り下がりあり
        // d[i+1] - (xs[i]+1)枚
        dp[i + 1][1] = min(dp[i + 1][1], dp[i][1] + (d[i + 1] - (xs[i] + 1)));
    }
    println!("{}", dp[n][0]);
}
