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
        h: usize,
        w: usize,
        k: usize,
        sx: usize,
        sy: usize,
        gx: usize,
        gy: usize,
    }
    // dp[i][j]:=i回目の移動で、j=00, 01, 10, 11の状態の場合の数。
    // 00: 行も列も合ってない
    // 01: 行は合ってないが、列は合ってる
    // 10: 行は合ってるが、列は合ってない
    // 11: 行も列も合ってる(目的地)
    let mut dp = vec![vec![Mint::new(0); 4]; k + 1];
    let mut init = 0;
    if sx == gx {
        init += 2;
    }
    if sy == gy {
        init += 1;
    }
    dp[0][init] = Mint::new(1);
    for i in 0..k {
        // 00 <- 00 (h-1)(w-1)通り
        // 00 <- 01 (w-1)通り
        // 00 <- 10 (h-1)通り
        // 00 <- 11 0通り
        dp[i + 1][0] = dp[i][0] * (h - 2 + w - 2) + dp[i][1] * (w - 1) + dp[i][2] * (h - 1);
        // 01 <- 00 1通り
        // 01 <- 01 (h-2)通り
        // 01 <- 10 0通り
        // 01 <- 11 (h-1)通り
        dp[i + 1][1] = dp[i][0] + dp[i][1] * (h - 2) + dp[i][3] * (h - 1);
        // 10 <- 00 1通り
        // 10 <- 01 0通り
        // 10 <- 10 (w-2)通り
        // 10 <- 11 (w-1)通り
        dp[i + 1][2] = dp[i][0] + dp[i][2] * (w - 2) + dp[i][3] * (w - 1);
        // 11 <- 00 0通り
        // 11 <- 01 1通り
        // 11 <- 10 1通り
        // 11 <- 11 0通り
        dp[i + 1][3] = dp[i][1] + dp[i][2];
    }
    println!("{}", dp[k][3].val())
}
