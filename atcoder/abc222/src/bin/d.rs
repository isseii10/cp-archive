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
        b: [usize; n],
    }
    // dp[i][j]: iまで見た時に、j以下の数をciとした時の場合の数
    let mut dp = vec![vec![Mint::new(0); 3030]; n + 1];
    for j in 0..3030 {
        dp[0][j] = Mint::new(1);
    }

    for i in 0..n {
        for ci in a[i]..=b[i] {
            dp[i + 1][ci] = dp[i + 1][ci] + dp[i][ci];
        }

        for j in 0..3000 {
            dp[i + 1][j + 1] = dp[i + 1][j + 1] + dp[i + 1][j];
        }
    }

    println!("{}", dp[n][3000]);
}
