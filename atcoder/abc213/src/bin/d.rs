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

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n-1],
    }
    let mut edges = vec![vec![]; n];
    for (a, b) in ab {
        edges[a - 1].push(b - 1);
        edges[b - 1].push(a - 1);
    }
    for i in 0..n {
        edges[i].sort();
    }
    let mut visited = Set::new();
    let mut ans: Vec<usize> = vec![];
    dfs(&edges, &mut visited, 0, &mut ans);

    for i in 0..ans.len() {
        if i == ans.len() - 1 {
            print!("{}\n", ans[i]);
        } else {
            print!("{} ", ans[i]);
        }
    }
}

fn dfs(edges: &Vec<Vec<usize>>, visited: &mut Set<usize>, current: usize, ans: &mut Vec<usize>) {
    visited.insert(current);
    ans.push(current + 1);

    for next in &edges[current] {
        if visited.contains(&next) {
            continue;
        }
        dfs(edges, visited, *next, ans);
        ans.push(current + 1);
    }
}
