#[allow(unused_imports)]
use amplify::confinement::Collection;
use ordered_float::OrderedFloat;
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
        t: usize,
    }
    for _ in 0..t {
        solve();
    }
}

fn solve() {
    input! {
        n: usize,
        mut k: usize,
        mut x: usize,
        a: [OrderedFloat<f64>; n],
    }
    let mut hq = Heap::new();
    for &v in a.iter() {
        hq.push((v, 1));
    }

    // println!("{:?}", hq);
    while k > 0 {
        let (v, c) = hq.pop().unwrap();
        if c <= k {
            hq.push((v / 2.0, c * 2));
            k -= c;
        } else {
            hq.push((v, c - k));
            hq.push((v / 2.0, k * 2));
            break;
        }
    }
    // println!("{:?}", hq);
    loop {
        let (v, c) = hq.pop().unwrap();
        if c >= x {
            println!("{}", v);
            return;
        }
        x -= c;
    }
}
