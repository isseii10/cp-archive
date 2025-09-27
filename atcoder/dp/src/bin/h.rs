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
type Mint = ac_library::ModInt1000000007;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h],
    }
    let mut dp = vec![vec![Mint::new(0); w]; h];
    dp[0][0] = Mint::new(1);

    for i in 0..h {
        for j in 0..w {
            if i < h - 1 && a[i + 1][j] == '.' {
                dp[i + 1][j] = dp[i + 1][j] + dp[i][j];
            }
            if j < w - 1 && a[i][j + 1] == '.' {
                dp[i][j + 1] = dp[i][j + 1] + dp[i][j];
            }

        }
    }
    println!("{}", dp[h - 1][w - 1].val())
}
