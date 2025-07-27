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
        k: usize,
        mut a: [usize; n],
    }
    a.sort();
    let mut acc_a = vec![0; n + 1];
    for i in 0..n {
        acc_a[i + 1] += acc_a[i] + a[i];
    }

    let mut ok = 0;
    let mut ng = 1_000_000_000_000_000_000;
    while ng - ok > 1 {
        let mid = (ok + ng) / 2;
        if is_ok(mid, &a, &acc_a, n, k) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}

// x個のプロジェクトを作れるか
fn is_ok(x: usize, a: &[usize], acc_a: &[usize], n: usize, k: usize) -> bool {
    let idx = a.partition_point(|&v| v < x);
    // println!("x: {}, idx: {}", x, idx);
    if n - idx >= k {
        return true;
    }

    if acc_a[idx] >= x * (idx - (n - k)) {
        return true;
    }
    false
}
