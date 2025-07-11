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
        s: [usize; n],
        t: [usize; n],
    }
    let mut first = 0;
    let mut min_t = MAX;
    for i in 0..n {
        if min_t > t[i] {
            first = i;
            min_t = t[i]
        }
    }
    let mut ans = vec![MAX; n];
    let mut time = min_t;
    for i in first..first + n {
        let i = i % n;
        if time > t[i] {
            ans[i] = t[i];
            time = t[i]
        } else {
            ans[i] = time
        }
        time += s[i]
    }
    for a in ans {
        println!("{}", a)
    }
}
