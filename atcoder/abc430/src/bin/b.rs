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
        m: usize,
        s: [Chars; n],
    }
    let mut counter = Map::new();
    for i in 0..=n - m {
        for j in 0..=n - m {
            let mut tmp = vec![];
            for a in 0..m {
                for b in 0..m {
                    tmp.push(s[a + i][b + j]);
                }
            }
            // println!("i:{}, j:{} {:?}", i, j, tmp);
            if let Some(c) = counter.get_mut(&tmp) {
                *c += 1
            } else {
                counter.push((tmp, 1));
            }
        }
    }
    println!("{}", counter.len())
}
