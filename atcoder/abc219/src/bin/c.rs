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

fn main() {
    input! {
        x: Chars,
        n: usize,
        s: [Chars; n],
    }
    let mut mp: Map<char, char> = Map::new();
    for i in 0..26 {
        let original_c = (b'a' + i as u8) as char;
        mp.push((x[i], original_c));
    }

    let mut ss = vec![];
    for (i, v) in s.iter().enumerate() {
        let mut new_v = vec![];
        for c in v {
            new_v.push(mp.get(c).unwrap());
        }
        ss.push((new_v, i));
    }
    ss.sort();

    for &(_, i) in ss.iter() {
        println!("{}", s[i].iter().join(""))
    }
}
