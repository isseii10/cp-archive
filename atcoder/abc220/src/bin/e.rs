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
        d: usize,
    }
    let mut pow2 = vec![Mint::new(1); n + 1];
    for i in 1..=n {
        pow2[i] = pow2[i - 1] * 2;
    }
    // g[i]: 高さiの部分木における、根を通る距離dのパスの数
    // g[i] = g[i-1](そのまま根をシフトできる) + {leafを使うパスの数}
    let mut g = vec![Mint::new(0); n + 1];
    for i in 1..=n {
        let i = i as isize;
        let mut leaf = Mint::new(0);
        let left = i - 1; // 根からleafへの距離: i-1
        let right = d as isize - (i - 1);
        if 0 <= right && right <= i - 1 {
            leaf = pow2[max(left - 1, 0) as usize] * pow2[max(right - 1, 0) as usize];
            if left != right {
                // 左右が異なる場合は、左右を入れ替えた場合も数え上げる
                leaf *= 2;
            }
        }
        g[i as usize] = g[i as usize - 1] + leaf;
    }

    // f[i]: 高さiの部分木における、距離dのパスの数
    // f[i] = f[i-1]*2 + g[i]
    let mut f = vec![Mint::new(0); n + 1];
    for i in 1..=n {
        f[i] = f[i - 1] * 2 + g[i]
    }

    println!("{}", (f[n] * 2).val())
}
