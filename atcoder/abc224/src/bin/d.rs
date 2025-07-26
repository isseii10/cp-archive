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
        m: usize,
        uv: [(usize, usize); m],
        mut p: [usize; 8],
    }
    // 空頂点にコマ9を置く
    let mut piece9_pos = 1;
    let mut p2 = p.clone();
    p2.sort();
    for i in 0..8 {
        if p2[i] == piece9_pos {
            piece9_pos += 1;
        }
    }
    p.push(piece9_pos);
    // 0-indexedにする
    for i in 0..9 {
        p[i] -= 1
    }
    // println!("{:?}", p);

    // グラフ準備
    let mut graph = vec![vec![]; 10];
    for &(u, v) in uv.iter() {
        graph[u - 1].push(v - 1);
        graph[v - 1].push(u - 1);
    }
    let dist = bfs(&graph, p);

    if let Some(ans) = dist.get(&vec![0, 1, 2, 3, 4, 5, 6, 7, 8]) {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

fn bfs(graph: &Vec<Vec<usize>>, start_state: Vec<usize>) -> HashMap<Vec<usize>, usize> {
    let mut dist = Map::new();
    dist.push((start_state.clone(), 0));
    let mut queue = Deque::new();
    queue.push_back(start_state);
    while let Some(curr_state) = queue.pop_front() {
        let curr_node = curr_state[8];
        for &next_node in graph[curr_node].iter() {
            let swap_piece = curr_state.iter().position(|&x| x == next_node).unwrap(); // 次に空になる頂点に置かれているコマ
            let mut next_state = curr_state.clone();
            next_state.swap(8, swap_piece);
            if let Some(&d) = dist.get(&next_state) {
                if d <= dist[&curr_state] + 1 {
                    continue;
                }
            }
            dist.push((next_state.clone(), dist[&curr_state] + 1));
            queue.push_back(next_state);
        }
    }
    dist
}
