use ac_library::Dsu;
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
        m: usize,
        uv: [(usize, usize); m],
    }
    let mut dsu = Dsu::new(n);
    let mut edges = vec![];
    for (u, v) in uv {
        let u = u - 1;
        let v = v - 1;
        dsu.merge(u, v);
        edges.push((u, v));
    }
    let mut edge_count = vec![0; n];
    for &(u, _) in edges.iter() {
        let leader = dsu.leader(u);
        edge_count[leader] += 1;
    }
    let mut ans = Mint::new(1);

    for g in dsu.groups().iter() {
        if edge_count[dsu.leader(g[0])] != g.len() {
            ans *= 0;
        }
        ans *= 2;
    }
    println!("{}", ans.val());
}
