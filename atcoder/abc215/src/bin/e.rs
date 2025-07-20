use num::pow;
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
        s: Chars,
    }
    let pow2_10 = 1 << 10;

    // dp[i][j][k]: i番目まで見て、j(選んだコンテストの英字をbitで管理)のコンテストに出て、選んだ最後のコンテストがkの時の場合の数
    let mut dp = vec![vec![vec![Mint::new(0); 10]; pow2_10]; n + 1];

    let contests: Vec<usize> = s.iter().map(|&x| (x as u8 - b'A') as usize).collect();

    for i in 0..n {
        let next = contests[i];

        // 初回の出場
        dp[i + 1][1 << next][next] += 1;

        for j in 0..pow2_10 {
            for last in 0..10 {
                if (j >> last) & 1 == 0 {
                    // lastに出場しているのにjにlastが含まれていない場合はありえない
                    continue;
                }

                // nextに出場しない
                dp[i + 1][j][last] = dp[i + 1][j][last] + dp[i][j][last];

                // nextに出場する
                // 出場する1: 最後のコンテストが同じなら出場できる
                // 出場する2: 最後のコンテストとは異なるが、まだ出たことない種類のコンテストだから出場できる
                // つまり、出場できる条件は next == last || (j >> next) & 1 == 0
                if next != last && (j >> next) & 1 == 1 {
                    continue;
                }
                dp[i + 1][j | (1 << next)][next] =
                    dp[i + 1][j | (1 << next)][next] + dp[i][j][last];
            }
        }
    }

    let mut ans = Mint::new(0);
    for j in 0..pow2_10 {
        for k in 0..10 {
            ans += dp[n][j][k]
        }
    }
    println!("{}", ans)
}
