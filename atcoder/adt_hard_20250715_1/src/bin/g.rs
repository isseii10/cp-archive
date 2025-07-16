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
        mut ct: [(usize, usize); k],
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
    ct.push((1, 0));
    ct.sort_by(|(_, t1), (_, t2)| t1.cmp(t2));

    // // dp[i]: i番目のイベントに参加した場合の、参加イベントの最大値
    // let mut dp = vec![MIN; k + 1];
    // dp[0] = 0;
    //
    // for i in 0..k {
    //     let (mut c, t) = ct[i];
    //     c -= 1;
    //     for j in i + 1..k + 1 {
    //         let (mut nc, nt) = ct[j];
    //         nc -= 1;
    //         if dist[c][nc] < nt - t {
    //             dp[j] = max(dp[j], dp[i] + 1);
    //         }
    //     }
    // }
    // let ans = dp.iter().max().unwrap();

    // dp[i][j]: i番目のイベントまで見て、j番目を最後に参加したときの最大参加数
    let mut dp = vec![vec![MIN; k + 1]; k + 1];
    dp[0][0] = 0;

    for i in 0..k {
        for last in 0..=i {
            if dp[i][last] == MIN {
                continue;
            }
            // 次のイベントに参加しない場合
            dp[i + 1][last] = max(dp[i + 1][last], dp[i][last]);

            let (mut c1, t1) = ct[last];
            let (mut c2, t2) = ct[i + 1];
            c1 -= 1;
            c2 -= 1;

            // 間に合う場合は参加できる
            if dist[c1][c2] < t2 - t1 {
                dp[i + 1][i + 1] = max(dp[i + 1][i + 1], dp[i][last] + 1);
            }
        }
    }

    let ans = dp[k].iter().max().unwrap();
    println!("{}", ans)
}

fn bfs(n: usize, graph: &Vec<Vec<(usize, usize)>>, start: usize) -> Vec<usize> {
    let mut dist = vec![MAX; n];
    dist[start] = 0;
    let mut queue = Deque::new();
    queue.push_back(start);
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
