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
        mut n: usize,
        m: usize,
        mut ab: [(usize, usize); m],
    }
    ab.sort();

    let mut aba: Vec<(usize, (usize, usize))> = vec![(0, (0, 0)); m];
    aba[0] = (ab[0].0, (ab[0].0, ab[0].1));
    for i in 1..m {
        let (_, (pa, pb)) = aba[i - 1];
        let (a, b) = ab[i];
        // a - bは消費量
        if a - b > pa - pb {
            aba[i] = (a, (pa, pb))
        } else {
            aba[i] = (a, (a, b))
        }
    }
    // println!("{:?}", aba);
    let mut ans = 0;
    while n > 0 {
        let idx = aba.partition_point(|&(a, _)| a <= n);
        if idx == 0 {
            break;
        }
        let (_, (a, b)) = aba[idx - 1];
        // (n-a)/(a-b):nがa以上を満たすという条件でa-bが何回できるか
        // +1: a以上からa未満にするときの一回
        let count = (n - a) / (a - b) + 1;
        ans += count;
        // println!("{}", ans);
        n -= count * (a - b);
    }
    println!("{}", ans)
}
