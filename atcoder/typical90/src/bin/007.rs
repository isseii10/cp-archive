#[allow(unused_imports)]
use amplify::confinement::Collection;
use num::abs;
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
        mut a: [i64; n],
        q: usize,
        b: [i64; q],
    }
    a.sort();
    a.dedup();
    for &bi in b.iter() {
        let idx = a.partition_point(|&ai| ai < bi);
        if idx == a.len() {
            println!("{}", bi - a[a.len() - 1]);
        } else {
            let idx2 = idx.saturating_sub(1);
            // println!("idx1:{}, bi:{}, a[idx]:{}", idx, bi, a[idx]);
            // println!("idx2:{}, bi:{}, a[idx2]:{}", idx2, bi, a[idx2]);
            println!("{}", min(abs(bi - a[idx]), abs(bi - a[idx2])));
        }
    }
}
