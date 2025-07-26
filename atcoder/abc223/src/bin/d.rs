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
        ab: [(usize, usize); m],
    }
    let mut graph = vec![vec![]; n];
    let mut indeg = vec![0; n]; // 入次数
    for &(a, b) in ab.iter() {
        indeg[b - 1] += 1;
        graph[a - 1].push(b - 1);
    }
    if let Some(ans) = topological_sort(n, &graph, &mut indeg) {
        println!(
            "{}",
            ans.iter()
                .map(|&x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    } else {
        println!("-1")
    }
}

fn topological_sort(n: usize, graph: &Vec<Vec<usize>>, deg: &mut Vec<usize>) -> Option<Vec<usize>> {
    let mut heapq = Heap::new();
    for (v, &d) in deg.iter().enumerate() {
        if d == 0 {
            heapq.push(Reverse(v));
        }
    }
    // println!("graph: {:?}", graph);
    // println!("deg: {:?}", deg);

    let mut order = vec![];

    while let Some(Reverse(p)) = heapq.pop() {
        order.push(p + 1);
        for &c in graph[p].iter() {
            deg[c] -= 1;
            if deg[c] == 0 {
                heapq.push(Reverse(c));
            }
        }
    }
    if order.len() == n {
        Some(order)
    } else {
        // サイクルがあり、トポロジカルソートできなかった
        None
    }
}
