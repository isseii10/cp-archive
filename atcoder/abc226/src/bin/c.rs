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
    }
    let mut t = vec![];
    let mut a = vec![];
    for _ in 0..n {
        input! {
            _t: usize,
            k: usize,
            mut ai: [usize; k],
        }
        t.push(_t);
        for i in 0..k {
            ai[i] -= 1;
        }
        a.push(ai);
    }
    let mut visited = vec![false; n];
    let mut deq = Deque::new();
    visited[n - 1] = true;
    deq.push_back(n - 1);
    while let Some(p) = deq.pop_front() {
        for &c in a[p].iter() {
            if !visited[c] {
                visited[c] = true;
                deq.push_back(c);
            }
        }
    }
    let ans = t
        .iter()
        .enumerate()
        .filter(|&(i, _)| visited[i])
        .fold(0, |acc, (_, &v)| acc + v);

    println!("{}", ans)
}
