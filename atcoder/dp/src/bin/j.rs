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
        a: [usize; n],
    }
    let cnt = a.iter().fold(vec![0; 4], |mut acc, &x| {
        acc[x] += 1;
        acc
    });
    // dp[i][j][k] := 3の皿がi個, 2の皿がj個, 1の皿がk個残っているときの期待値
    let mut dp = vec![vec![vec![0.0; n + 10]; n + 10]; n + 10];
    for i in 0..=n {
        for j in 0..=n {
            for k in 0..=n {
                let nonzero = (i + j + k) as f64;
                if nonzero == 0.0 {
                    dp[i][j][k] = 0.0;
                    continue;
                }
                dp[i][j][k] = (n as f64) / nonzero;
                if i > 0 {
                    dp[i][j][k] += dp[i - 1][j + 1][k] * (i as f64) / nonzero;
                }
                if j > 0 {
                    dp[i][j][k] += dp[i][j - 1][k + 1] * (j as f64) / nonzero;
                }
                if k > 0 {
                    dp[i][j][k] += dp[i][j][k - 1] * (k as f64) / nonzero;
                }
            }
        }
    }
    println!("{}", dp[cnt[3]][cnt[2]][cnt[1]])
}
