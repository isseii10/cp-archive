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
    }
    let mut graph = vec![vec![]; n];
    // 出次数の配列
    let mut deg = vec![0; n];
    for _ in 0..m {
        input! {
            k: usize,
            a: [usize; k],
        }
        for j in 0..k - 1 {
            let from = a[j] - 1;
            let to = a[j + 1] - 1;
            // 逆向きに有向辺を貼る
            graph[to].push(from);
            deg[from] += 1
        }
    }
    // 有向グラフのループ検出
    let mut queue = Deque::new();
    for i in 0..n {
        if deg[i] == 0 {
            // 出次数0のノードを格納しておく
            queue.push_back(i);
        }
    }

    // トポロジカルソートの逆順
    let mut order_rev: Vec<usize> = vec![];
    while !queue.is_empty() {
        let v = queue.pop_front().unwrap();
        order_rev.push(v);
        for next in graph[v].iter() {
            deg[*next] -= 1;
            if deg[*next] == 0 {
                queue.push_back(*next);
            }
        }
    }
    println!("{}", if order_rev.len() == n { "Yes" } else { "No" })
}
