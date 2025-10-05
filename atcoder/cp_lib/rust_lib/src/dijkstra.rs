use std::cmp::Reverse;
use std::collections::BinaryHeap;

use cargo_snippet::snippet;

/// ダイクストラ法
///
/// - `graph[v]` に `(to, cost)` のタプルを持つ隣接リスト形式
/// - 負のコストは非対応（必要ならBellman-Fordを使う）
///
/// 返り値: (dist, prev)
/// - dist[v]: 始点からvへの最短距離（i64::MAXなら到達不可）
/// - prev[v]: vに至る直前の頂点（Noneなら未到達）
#[snippet("dijkstra")]
pub fn dijkstra(graph: &Vec<Vec<(usize, i64)>>, start: usize) -> (Vec<i64>, Vec<Option<usize>>) {
    let n = graph.len();
    let mut dist = vec![i64::MAX; n];
    let mut prev = vec![None; n];
    // 到達できる頂点の中で短い順に取り出す優先度付きキュー
    let mut pq = BinaryHeap::new();

    dist[start] = 0;
    pq.push(Reverse((0, start)));

    while let Some(Reverse((d, v))) = pq.pop() {
        // すでにより良い距離が確定している場合はスキップ
        if d > dist[v] {
            continue;
        }
        for &(to, cost) in &graph[v] {
            let nd = d + cost;
            if nd < dist[to] {
                dist[to] = nd;
                prev[to] = Some(v);
                pq.push(Reverse((nd, to)));
            }
        }
    }

    (dist, prev)
}

/// 経路を復元する
/// - start: 始点
/// - goal: 終点
/// - prev: dijkstra() が返した prev ベクタ
#[snippet("dijkstra")]
pub fn reconstruct_path(start: usize, goal: usize, prev: &Vec<Option<usize>>) -> Vec<usize> {
    let mut path = vec![];
    let mut cur = Some(goal);
    while let Some(v) = cur {
        path.push(v);
        if v == start {
            break;
        }
        cur = prev[v];
    }
    if path.last() != Some(&start) {
        return vec![]; // 到達不可
    }
    path.reverse();
    path
}

fn main() {
    let n = 5;
    let mut graph = vec![vec![]; n];

    let edges = vec![
        (0, 1, 2),
        (0, 2, 4),
        (1, 2, 1),
        (1, 3, 7),
        (2, 4, 3),
        (3, 4, 1),
    ];

    // 無向グラフにしたい場合は両方向に追加
    for (a, b, c) in edges {
        graph[a].push((b, c));
        graph[b].push((a, c));
    }

    let start = 0;
    let goal = 4;
    let (dist, prev) = dijkstra(&graph, start);
    let path = reconstruct_path(start, goal, &prev);

    println!("最短距離: {:?}", dist);
    println!("経路: {:?}", path);
}
