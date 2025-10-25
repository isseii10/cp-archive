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
        a: [usize; n],
    }
    let mut counter = Map::new();
    for &e in a.iter() {
        if let Some(c) = counter.get_mut(&e) {
            *c += 1
        } else {
            counter.push((e, 1))
        }
    }
    let mut ans = 0;
    for (_, &v) in counter.iter() {
        let others = n - v;
        ans += (v * (v - 1) / 2) * others
    }
    println!("{}", ans)
}
