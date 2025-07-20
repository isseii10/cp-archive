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

fn main() {
    input! {
        n: usize,
        m: usize,

        mut a: [isize; n],
        b: [isize; m],
    }
    a.sort();
    let mut ans = 1_000_000_001;
    for i in 0..m {
        let left = a.partition_point(|&x| x < b[i]);
        if left != n {
            ans = ans.min(abs(a[left] - b[i]));
        }
        if left != 0 {
            ans = ans.min(abs(a[left - 1] - b[i]));
        }
    }
    println!("{}", ans);
}
