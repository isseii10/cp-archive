#[allow(unused_imports)]
use amplify::confinement::Collection;
use ordered_float::OrderedFloat;
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
        a: [usize; n],
    }
    // dp[i][j] := i番目まで見た時の平均値の最大(jはi番目を選んだかどうか)
    let mut dp = vec![vec![(0, 0); 2]; n + 1];
    for i in 0..n {
        let (s0, l0) = dp[i][0];
        let (s1, l1) = dp[i][1];

        // i番目を選ばない
        dp[i + 1][0] = (s1, l1);

        // i番目を選ぶ
        if s0 * l1 > s1 * l0 {
            // s0/l0 > s1/l1
            dp[i + 1][1] = (s0 + a[i], l0 + 1);
        } else if s0 * l1 < s1 * l0 {
            // s0/l0 < s1/l1
            dp[i + 1][1] = (s1 + a[i], l1 + 1);
        } else {
            // 平均が同じなら、要素数が少ない方
            if l0 < l1 {
                dp[i + 1][1] = (s0 + a[i], l0 + 1);
            } else {
                dp[i + 1][1] = (s1 + a[i], l1 + 1);
            }
        }
    }
    println!("{:?}", dp);
    println!(
        "{:.10}",
        dp[n]
            .iter()
            .map(|&(s, l)| OrderedFloat(s as f64 / l as f64))
            .max()
            .unwrap()
    )
}
