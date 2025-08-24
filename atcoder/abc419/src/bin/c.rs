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
        rc: [(usize, usize); n],
    }
    let mut max_r = 0;
    let mut min_r = 10_000_000_000;
    let mut max_c = 0;
    let mut min_c = 10_000_000_000;
    for &(r, c) in rc.iter() {
        if r > max_r {
            max_r = r;
        }
        if r < min_r {
            min_r = r;
        }
        if c > max_c {
            max_c = c;
        }
        if c < min_c {
            min_c = c;
        }
    }
    let ans_r = if (max_r - min_r) % 2 == 0 {
        (max_r - min_r) / 2
    } else {
        (max_r - min_r) / 2 + 1
    };
    let ans_c = if (max_c - min_c) % 2 == 0 {
        (max_c - min_c) / 2
    } else {
        (max_c - min_c) / 2 + 1
    };
    println!("{}", max(ans_r, ans_c));
}
