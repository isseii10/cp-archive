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
        ab: [(usize, usize); n-1],
    }
    let mut g = vec![vec![]; n];
    for &(a, b) in ab.iter() {
        g[a - 1].push(b - 1);
        g[b - 1].push(a - 1);
    }

    // 木の直径

    let tmp_dist = bfs(&g, 0);

    let mut v = None;
    let mut max_d = 0;
    for (i, &d) in tmp_dist.iter().enumerate() {
        if d >= max_d {
            max_d = d;
            v = Some(i);
        }
    }
    let v = v.unwrap();

    let dist_from_v = bfs(&g, v);
    // println!("v:{}, dist_from_v:{:?}", v + 1, dist_from_v);
    let mut u = None;
    let mut max_d = 0;
    for (i, &d) in dist_from_v.iter().enumerate() {
        if d >= max_d {
            max_d = d;
            u = Some(i);
        }
    }
    let u = u.unwrap();
    let dist_from_u = bfs(&g, u);
    // println!("u:{}, dist_from_u:{:?}", u + 1, dist_from_u);

    for i in 0..n {
        let a = max_d;
        let b = dist_from_v[i];
        let c = dist_from_u[i];
        let x = (a + b - c) / 2;
        let y = (a + c - b) / 2;
        // let z = (b + c - a) / 2;
        if x > y {
            println!("{}", v + 1);
        } else if x < y {
            println!("{}", u + 1);
        } else {
            println!("{}", if u <= v { v + 1 } else { u + 1 });
        }
    }
}

fn bfs(g: &[Vec<usize>], start: usize) -> Vec<usize> {
    let n = g.len();
    let mut dist = vec![usize::MAX; n];
    let mut que = VecDeque::new();
    dist[start] = 0;
    que.push_back(start);
    while let Some(v) = que.pop_front() {
        for &nv in g[v].iter() {
            if dist[nv] == usize::MAX {
                dist[nv] = dist[v] + 1;
                que.push_back(nv);
            }
        }
    }
    dist
}
