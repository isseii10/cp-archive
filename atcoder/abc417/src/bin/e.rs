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
        t: usize,
    }
    for _ in 0..t {
        solve();
    }
}

fn solve() {
    input! {
        n: usize,
        m: usize,
        x: usize,
        y: usize,
        uv: [(usize, usize); m],
    }
    let x = x - 1;
    let y = y - 1;
    let mut graph = vec![vec![]; n];
    for &(u, v) in uv.iter() {
        let u = u - 1;
        let v = v - 1;
        graph[u].push(v);
        graph[v].push(u);
    }
    for i in 0..n {
        graph[i].sort();
    }
    let mut visited = vec![false; n];
    let sna = dfs(&graph, &mut visited, y, x);
    for i in (0..sna.len()).rev() {
        if i != 0 {
            print!("{} ", sna[i] + 1);
        } else {
            println!("{}", sna[i] + 1);
        }
    }
}

fn dfs(graph: &[Vec<usize>], visited: &mut [bool], goal: usize, curr: usize) -> Vec<usize> {
    visited[curr] = true;

    if curr == goal {
        return vec![curr];
    }

    for &next in graph[curr].iter() {
        if visited[next] {
            continue;
        }
        let mut ret = dfs(graph, visited, goal, next);
        if ret.len() > 0 {
            ret.push(curr);
            return ret;
        }
    }
    return vec![];
}
