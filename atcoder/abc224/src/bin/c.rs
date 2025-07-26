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
        xy: [(isize, isize); n],
    }
    let mut cnt = 0;
    for i in 0..n - 2 {
        let (x1, y1) = xy[i];
        for j in i + 1..n - 1 {
            let (x2, y2) = xy[j];
            for k in j + 1..n {
                let (x3, y3) = xy[k];
                cnt += if (y3 - y1) * (x2 - x1) == (y2 - y1) * (x3 - x1) {
                    1
                } else {
                    0
                };
            }
        }
    }
    let ans = n * (n - 1) * (n - 2) / 6 - cnt;
    println!("{}", ans)
}
