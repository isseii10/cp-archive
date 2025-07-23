use num::abs;
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
        k: isize,
        a: [usize; m],
        uv: [(usize, usize); n-1],
    }
    let mut graph = vec![vec![]; n];
    for (i, &(u, v)) in uv.iter().enumerate() {
        graph[u - 1].push((v - 1, i));
        graph[v - 1].push((u - 1, i));
    }
    let mut edge_count = vec![0; n - 1];
    for i in 0..m - 1 {
        let start = a[i] - 1;
        let goal = a[i + 1] - 1;
        let mut visited = vec![false; n];
        visited[start] = true;
        dfs(&graph, &mut visited, &mut edge_count, goal, start, None);
    }
    // println!("{:?}", edge_count);

    let total = edge_count.iter().sum::<usize>();
    // 赤と青は入れ替えても一般性は失われない
    let k = abs(k) as usize;

    // red - blue = k
    // red + blue = total
    // red = (k+total)/2
    if (k + total) % 2 != 0 {
        println!("0");
        return;
    }
    let s = ((k + total) / 2) as usize;
    // dp[i][j]: i番目の辺まで見て、辺通過回数がjになる場合の数
    let mut dp = vec![vec![Mint::new(0); s + 1]; n];
    dp[0][0] = Mint::new(1);
    for i in 0..n - 1 {
        // iの辺の通過回数
        let count = edge_count[i];
        for j in 0..=s {
            // 選ぶ
            if count + j <= s {
                dp[i + 1][count + j] = dp[i + 1][count + j] + dp[i][j];
            }
            // 選ばない
            dp[i + 1][j] = dp[i + 1][j] + dp[i][j];
        }
    }
    println!("{}", dp[n - 1][s])
}

fn dfs(
    graph: &Vec<Vec<(usize, usize)>>,
    visited: &mut [bool],
    edge_count: &mut Vec<usize>,
    goal: usize,
    curr: usize,
    par: Option<usize>,
) -> bool {
    for &(next, edge_index) in graph[curr].iter() {
        if visited[next] || Some(next) == par {
            continue;
        }
        visited[next] = true;
        if next == goal {
            edge_count[edge_index] += 1;
            return true;
        }
        if dfs(graph, visited, edge_count, goal, next, Some(curr)) {
            edge_count[edge_index] += 1;
            return true;
        }
    }
    false
}
