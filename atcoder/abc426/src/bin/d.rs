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
// type Mint = ac_library::ModInt1000000007;

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
        n: usize,
        mut s: Chars,
    }
    let mut total = vec![0; 2];

    for i in 0..n {
        if s[i] == '0' {
            total[0] += 1;
        } else {
            total[1] += 1;
        }
    }
    // ランレングスのための番兵
    if s[n - 1] == '0' {
        s.push('1')
    } else {
        s.push('0')
    }

    // ランレングス計算
    let mut run_length = vec![];
    let mut now_len = 0;
    for i in 0..n {
        now_len += 1;
        if s[i] != s[i + 1] {
            run_length.push((s[i], now_len));
            now_len = 0;
        }
    }

    let mut ans = MAX;
    for &(c, l) in run_length.iter() {
        // このrunを固定する
        if c == '0' {
            // 全て0にする
            let zero = total[0] - l;
            let one = total[1];
            ans = min(ans, zero * 2 + one)
        } else {
            // 全て1にする
            let zero = total[0];
            let one = total[1] - l;
            ans = min(ans, zero + one * 2)
        }
    }

    println!("{}", ans)
}
