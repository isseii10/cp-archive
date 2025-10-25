#[allow(unused_imports)]
use amplify::confinement::Collection;
use itertools::Itertools;
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
    let ans = vec![-1; n];
    let mut start = 0;
    for i in 0..n {
        if s[i] == 'D' {
            start = i;
            break;
        }
    }
}

fn dfs(
    g: &[Vec<usize>],
    dist: &mut [usize],
    memo: &mut [usize],
    now: usize,
    parent: Option<usize>,
) {
    for &child in g[now].iter() {
        if parent.is_some() && parent.unwrap() == child {
            continue;
        }
        if dist[child] <= dist[now] + 1 {
            continue;
        }
        dfs(g, dist, memo, child, Some(now))
    }
}
