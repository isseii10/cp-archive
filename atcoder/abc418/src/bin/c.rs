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
        q: usize,
        mut a: [usize; n],
    }
    a.sort();
    let mut sa = vec![0; n + 1];
    for i in 0..n {
        sa[i + 1] = sa[i] + a[i];
    }

    for _ in 0..q {
        input! {
            b: usize,
        }
        if b > a[n - 1] {
            println!("-1");
            continue;
        }
        let idx = a.partition_point(|&x| x < b - 1);
        if idx == n {
            println!("-1");
            continue;
        }
        let nb = sa[idx] + (n - idx) * (b - 1);
        println!("{}", nb + 1);
    }
}
