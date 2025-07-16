use amplify::confinement::Collection;
use petgraph::graph;
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
        m: usize,
        k: usize,
        abd: [(usize, usize, usize); m],
        ct: [(usize, usize); k],
    }
    // 最短経路の準備
    let mut graph = vec![vec![]; n];
    for i in 0..m {
        let (mut a, mut b, d) = abd[i];
        a -= 1;
        b -= 1;
        graph[a].push((b, d));
        graph[b].push((a, d));
    }
    let mut dist = vec![vec![]; n];
    for i in 0..n {
        let dist_i = bfs(n, &graph, i);
        dist[i] = dist_i
    }

    // イベントの準備
    let mut events_on = vec![vec![]; n];
    for (c, t) in ct {
        events_on[c-1].push(t);
    }



    // dp[i][j]: j個のイベントに参加してきて、今iにいる時の時間
    let mut dp = vec![vec![MAX; k+1]; n+1]
    dp[0][0] = 0;

    for i in 0..n {
        for j in 0..k {
            // iにいるので、iで起きるイベントを取るか取らないか
            dp[i]

        }
    }





}

fn bfs(n: usize, graph: &Vec<Vec<(usize, usize)>>, start: usize) -> Vec<usize> {
    let mut dist = vec![MAX; n];
    dist[start] = 0;
    let mut queue = Deque::new();
    queue.push(start);
    while !queue.is_empty() {
        let p = queue.pop_front().unwrap();
        for &(c, d) in graph[p].iter() {
            if dist[c] <= dist[p] + d {
                continue;
            }
            dist[c] = dist[p] + d;
            queue.push_back(c);
        }
    }

    dist
}
