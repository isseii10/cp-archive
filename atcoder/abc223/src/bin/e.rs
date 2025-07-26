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
        m: usize,
        uv: [(usize, usize); m],
        mut p: [usize; 8],
    }
    // 空頂点にコマ9を置く
    let mut empty_node = 0;
    let mut p2 = p.clone();
    p2.sort();
    for i in 0..7 {
        if p2[i] + 1 != p2[i + 1] {
            empty_node = p2[i] + 1;
        }
    }
    p.push(empty_node);
    // 0-indexedにする
    for i in 0..9 {
        p[i] -= 1
    }

    // [0, 1, ..., 8]の順列に対して状態番号を割り当てる
    let mut to_state = Map::new();
    let mut to_state_idx = Map::new();

    let mut state = (0..9).collect_vec();
    let mut ok = true;
    let mut state_idx = 0;
    while ok {
        to_state.push((state_idx, state.clone()));
        to_state_idx.push((state.clone(), state_idx));
        state_idx += 1;
        ok = next_permutation(&mut state);
    }
    let mut graph = vec![vec![]; 10];
    for &(u, v) in uv.iter() {
        graph[u - 1].push(v - 1);
        graph[v - 1].push(u - 1);
    }
    let mut dist = vec![MAX; state_idx];
    dist[to_state_idx[&p]] = 0;
    dfs(state_idx, &graph, &mut dist, &p, p[8], &to_state_idx);

    let ans = dist[to_state_idx[&p]];
    if ans == MAX {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}

fn dfs(
    n: usize,
    graph: &Vec<Vec<usize>>,
    dist: &mut Vec<usize>,
    curr_state: &Vec<usize>,
    curr: usize,
    to_state_idx: &Map<Vec<usize>, usize>,
) {
    for &next in graph[curr].iter() {
        let mut next_state = curr_state.clone();
        next_state.swap(curr, next);
        if dist[to_state_idx[&next_state]] <= dist[to_state_idx[curr_state]] + 1 {
            continue;
        }
        dist[to_state_idx[&next_state]] = dist[to_state_idx[curr_state]] + 1;
        dfs(n, graph, dist, &next_state, next, to_state_idx);
    }
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
