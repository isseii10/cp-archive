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
        uv: [(usize,usize); m],
        s: Chars,
    }
    let mut g = vec![vec![]; n];
    for &(u, v) in uv.iter() {
        g[u - 1].push(v - 1);
        g[v - 1].push(u - 1);
    }
    let mut starts = vec![];
    for i in 0..n {
        if s[i] == 'S' {
            starts.push(i);
        }
    }
    let dists = multi_source_bfs(starts, &g, n);

    for i in 0..n {
        if s[i] == 'D' {
            println!("{}", dists[i].iter().map(|(d, _)| d).sum::<usize>())
        }
    }
}

fn multi_source_bfs(starts: Vec<usize>, g: &[Vec<usize>], n: usize) -> Vec<Vec<(usize, usize)>> {
    let mut dists = vec![vec![]; n];
    let mut deque = Deque::new();
    for &s in starts.iter() {
        dists[s].push((0, s));
        deque.push_back((s, s, 0));
    }
    while let Some((v, sv, d)) = deque.pop_front() {
        for &nv in g[v].iter() {
            if ok(&dists[nv], sv) {
                dists[nv].push((d + 1, sv));
                deque.push_back((nv, sv, d + 1));
            }
        }
    }
    dists
}

fn ok(dists: &[(usize, usize)], s: usize) -> bool {
    if dists.len() >= 2 {
        return false;
    }
    // distsには異なる2点からの最短距離が格納されてほしいので、同じ始点から来るケースを省く
    for &(_, sv) in dists.iter() {
        if sv == s {
            return false;
        }
    }
    true
}
