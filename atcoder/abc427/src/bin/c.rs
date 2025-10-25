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
        n: usize,
        m: usize,
        uv: [(usize, usize); m],
    }
    let mut ans = m;
    for i in 1..n {
        if i > n - i {
            break;
        }
        // i個とn-i個に分けた
        let mut groups = vec![0; n];
        for g in groups.iter_mut().rev().take(i) {
            *g = 1;
        }
        let mut ok = true;
        while ok {
            let mut res = 0;
            for &(u, v) in uv.iter() {
                if groups[u - 1] == groups[v - 1] {
                    res += 1;
                }
            }
            ans = min(ans, res);
            ok = next_permutation(&mut groups)
        }
    }
    println!("{}", ans)
}

fn next_permutation<T: Ord>(a: &mut [T]) -> bool {
    let n = a.len();
    if n < 2 {
        return false;
    }
    let mut i = n - 2;
    while i != usize::MAX && a[i] >= a[i + 1] {
        if i == 0 {
            break;
        }
        i -= 1;
    }
    if i == 0 && a[0] >= a[1] {
        return false;
    }
    let mut j = n - 1;
    while a[j] <= a[i] {
        j -= 1;
    }
    a.swap(i, j);
    a[i + 1..].reverse();
    true
}
