#[allow(unused_imports)]
use amplify::confinement::Collection;
use itertools::Itertools;
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
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    }
    let mut sum_h = vec![0; h];
    let mut sum_w = vec![0; w];
    for i in 0..h {
        for j in 0..w {
            sum_h[i] += a[i][j];
            sum_w[j] += a[i][j];
        }
    }
    let mut b = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            b[i][j] = sum_h[i] + sum_w[j] - a[i][j];
        }
    }

    for i in 0..h {
        println!("{}", b[i].iter().join(" "));
    }
}
