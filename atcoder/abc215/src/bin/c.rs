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
        mut s: Chars,
        mut k: usize,
    }
    s.sort();
    while k > 1 {
        if !next_permutation(&mut s) {
            break;
        }
        k -= 1
    }

    println!("{}", s.iter().collect::<String>())
}

fn next_permutation<T: Ord>(a: &mut [T]) -> bool {
    let n = a.len();
    if n < 2 {
        return false;
    }

    let mut i = n - 2;
    while i != usize::MAX && a[i] >= a[i + 1] {
        if i == 0 {
            break;
        }
        i -= 1;
    }

    if i == 0 && a[0] >= a[1] {
        return false;
    }

    let mut j = n - 1;
    while a[j] <= a[i] {
        j -= 1;
    }

    a.swap(i, j);
    a[i + 1..].reverse();
    true
}
