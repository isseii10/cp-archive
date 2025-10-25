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
// type Mint = ac_library::ModInt1000000007;

fn main() {
    input! {
        n: usize,
    }
    let mut a = vec![0; n + 1];
    a[0] = 1;
    let mut prev = 0;
    for i in 1..=n {
        a[i] += prev + f(a[i - 1]);
        prev = a[i];
    }
    println!("{:?}", a[n]);
}

fn f(x: usize) -> usize {
    let mut x = x;
    let mut ret = 0;
    while x > 0 {
        ret += x % 10;
        x /= 10
    }
    ret
}
