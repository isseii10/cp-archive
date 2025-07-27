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
        m: usize,
        b: [[usize;m]; n],
    }
    let b0 = b[0][0] - 1;
    let y = b0 / 7;
    let x = b0 % 7;
    let mut ok = true;
    for i in 0..n {
        for j in 0..m {
            let y2 = (b[i][j] - 1) / 7;
            let x2 = (b[i][j] - 1) % 7;
            if y + i != y2 || x + j != x2 {
                // println!(
                //     "b:{}, y:{}, i:{}, y2:{}, x:{}, j:{}, x2:{}",
                //     b[i][j], y, i, y2, x, j, x2
                // );
                ok = false;
            }
        }
    }
    println!("{}", if ok { "Yes" } else { "No" });
}
