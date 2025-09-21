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
        n: usize,
        ab: [(usize, usize); n],
    }
    let mut g = vec![vec![]; n];
    let mut starts = vec![];
    for i in 0..n {
        let (a, b) = ab[i];
        if a == 0 && b == 0 {
            starts.push(i);
            continue;
        }
        g[a - 1].push(i);
        g[b - 1].push(i);
    }
    let mut visited = vec![false; n];
    let mut queue = Deque::new();
    for &s in starts.iter() {
        // println!("start from {}", s);
        if visited[s] {
            continue;
        }
        visited[s] = true;
        queue.push_back(s);
        while let Some(v) = queue.pop_front() {
            for &nv in g[v].iter() {
                if visited[nv] {
                    continue;
                }
                visited[nv] = true;
                queue.push_back(nv);
            }
        }
    }
    println!(
        "{}",
        visited
            .iter()
            .map(|&x| if x { 1 } else { 0 })
            .sum::<usize>()
    )
}
