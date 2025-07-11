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

use ac_library::Dsu;

fn main() {
    input! {
        n: usize,
        vvw: [(usize, usize, usize); n-1],
    }
    let mut edges: Vec<(usize, usize, usize)> = vvw.iter().map(|&(u, v, w)| (w, u, v)).collect();
    edges.sort();

    let mut dsu = Dsu::new(n + 1);
    let mut ans = 0;
    for (w, u, v) in edges {
        ans += dsu.size(u) * dsu.size(v) * w;
        dsu.merge(u, v);
    }
    println!("{}", ans);
}
