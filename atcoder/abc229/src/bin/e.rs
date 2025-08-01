use ac_library::Dsu;
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
        m: usize,
        ab: [(usize, usize); m],
    }
    let mut graph = vec![vec![]; n];
    for (a, b) in ab.iter() {
        graph[a - 1].push(b - 1);
    }
    let mut ans = vec![0; n + 1];
    ans[n] = 0;

    let mut dsu = Dsu::new(n);
    for i in (0..n).rev() {
        ans[i] = ans[i + 1] + 1;
        for &j in graph[i].iter() {
            if !dsu.same(i, j) {
                dsu.merge(i, j);
                ans[i] -= 1;
            }
        }
    }
    for i in 1..=n {
        println!("{}", ans[i]);
    }
}
