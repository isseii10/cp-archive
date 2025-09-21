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
        t: usize,
    }
    for _ in 0..t {
        solve();
    }
}

fn solve() {
    input! {
        h: usize,
        w: usize,
        mut s: [Chars; h],
    }
    // 状態数
    let k = 1 << w;

    // 遷移可能性テーブル allow[prev][curr]
    let mut allow = vec![vec![true; k]; k];
    for i in 0..k {
        for j in 0..k {
            for ii in 0..w - 1 {
                if ((i >> ii) & 3) == 3 && ((j >> ii) & 3) == 3 {
                    allow[i][j] = false;
                    break;
                }
            }
        }
    }

    let inf: usize = 1_000_000_000;
    let mut dp = vec![inf; k];
    dp[0] = 0;

    for i in 0..h {
        let mut field_state = 0;
        for (j, &ch) in s[i].iter().enumerate() {
            if ch == '#' {
                field_state |= 1 << j;
            }
        }

        let mut ndp = vec![inf; k];
        for state in 0..k {
            // 白マス(0)が黒マス(1)になることはできない
            // -> stateはfield_stateの1を0に変えたものでないといけない
            // -> orをとるとfield_stateに等しくなるのが条件
            if (state | field_state) != field_state {
                continue;
            }
            for prev_state in 0..k {
                if allow[prev_state][state] {
                    let cost = (state ^ field_state).count_ones() as usize;
                    ndp[state] = min(ndp[state], dp[prev_state] + cost);
                }
            }
        }
        dp = ndp;
    }

    let ans = dp.into_iter().min().unwrap();
    println!("{}", ans);
}
