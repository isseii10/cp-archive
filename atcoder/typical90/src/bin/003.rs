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
        ab: [(usize, usize); n-1],
    }
    let mut g = vec![vec![]; n];
    for &(a, b) in ab.iter() {
        g[a - 1].push(b - 1);
        g[b - 1].push(a - 1);
    }

    // 木の直径を求める
    // 1. 適当な点から最も遠い点uを求める
    let mut dist_0 = vec![MAX; n];
    dist_0[0] = 0;
    dfs(&g, &mut dist_0, 0, None);
    let mut max_dist = 0;
    let mut u = 0;
    for i in 0..n {
        if dist_0[i] > max_dist {
            max_dist = dist_0[i];
            u = i;
        }
    }
    // 2. uから最も遠い点までの距離が木の直径となる
    let mut dist_u = vec![MAX; n];
    dist_u[u] = 0;
    dfs(&g, &mut dist_u, u, None);

    // 直径+1が答え
    println!("{}", dist_u.iter().max().unwrap() + 1);
}

fn dfs(g: &[Vec<usize>], dist: &mut [usize], cur: usize, pre: Option<usize>) {
    for &nxt in g[cur].iter() {
        if Some(nxt) == pre {
            continue;
        }
        if dist[nxt] < dist[cur] + 1 {
            continue;
        }
        dist[nxt] = dist[cur] + 1;
        dfs(g, dist, nxt, Some(cur));
    }
}
