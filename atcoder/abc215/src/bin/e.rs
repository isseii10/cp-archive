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

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let _mod: usize = 998244353;
    let pow2_10 = pow(2, 10);

    // dp[i][j][k]: i番目まで見て、j(選んだコンテストの英字をbitで管理)のコンテストに出て、選んだ最後のコンテストがkの時の場合の数
    let mut dp = vec![vec![vec![0; 10]; pow(2, 10)]; n + 1];
    dp[0][0][0] = 1;

    let contests: Vec<usize> = s
        .iter()
        .map(|&x| x as u8 - b'A')
        .map(|x| x as usize)
        .collect();

    for i in 0..n {
        let contest = contests[i];
        // 初回選ぶときの遷移
        dp[i + 1][1 << contest][contest] += (dp[i + 1][1 << contest][contest] + 1) % _mod;

        for j in 0..pow2_10 {
            for k in 0..10 {
                if (j >> k) & 1 == 0 {
                    // dp[i][j][k]でkが選ばれているのにjにkが含まれていない場合はありえない
                    continue;
                }
                // 選ばない
                dp[i + 1][j][k] = (dp[i + 1][j][k] + dp[i][j][k]) % _mod;

                // 選ぶ
                if k == contest {
                    // 選ぶ1: 出たことあるから最後のコンテストが同じなら選べる
                    dp[i + 1][j][k] = (dp[i + 1][j][k] + dp[i][j][k]) % _mod;
                } else if (j >> contest) & 1 == 0 {
                    // 選ぶ2: 出たことないから選べる
                    dp[i + 1][j | (1 << contest)][contest] =
                        (dp[i + 1][j | (1 << contest)][contest] + dp[i][j][k]) % _mod
                }
            }
        }
    }

    let mut ans = 0;

    for j in 0..pow2_10 {
        for k in 0..10 {
            ans = (ans + dp[n][j][k]) % _mod
        }
    }
    println!("{}", ans)
}
