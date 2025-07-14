use ac_library::Dsu;
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

fn main() {
    input! {
        l: usize,
        q: usize,
        cx: [(usize, usize); q],
    }

    let mut cut_indices = BTreeSet::new();
    cut_indices.push(0);
    cut_indices.push(l);
    for (c, x) in cx {
        if c == 1 {
            cut_indices.push(x);
        } else {
            println!(
                "{}",
                cut_indices.range(x..).next().unwrap()
                    - cut_indices.range(..x).next_back().unwrap(),
            )
        }
    }
}
