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
        mut a: [usize; n],
    }
    let mut is_infinite = true;
    let mut sum_a = 0;
    for &v in a.iter() {
        sum_a += v;
        if sum_a > m {
            is_infinite = false;
        }
    }
    if is_infinite {
        println!("infinite");
        return;
    }

    a.sort();
    let mut left = 0;
    let mut right = a[n - 1];
    while right - left > 1 {
        let mid = (left + right) / 2;
        if ok(&a, &mid, m) {
            left = mid;
        } else {
            right = mid;
        }
    }
    println!("{}", left)
}

fn ok(a: &Vec<usize>, x: &usize, m: usize) -> bool {
    let mut s = 0;
    for v in a {
        s += min(v, x);
    }
    if s > m {
        false
    } else {
        true
    }
}
