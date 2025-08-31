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
        x: usize,
        y: usize,
    }
    let mut a = vec![MAX; 10];
    a[0] = x;
    a[1] = y;
    for i in 2..10 {
        a[i] = rev(a[i - 1] + a[i - 2])
    }

    println!("{}", a[9])
}

fn rev(s: usize) -> usize {
    let ss = s.to_string().chars().collect::<Vec<char>>();
    let mut res = vec![];
    for &c in ss.iter().rev() {
        res.push(c);
    }
    res.iter().collect::<String>().parse::<usize>().unwrap()
}
