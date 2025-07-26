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
        mut s: Chars,
    }
    s.push('#');
    let mut dot_range = vec![];
    let mut l = None;
    let mut r = None;
    for (i, &v) in s.iter().enumerate() {
        if v == '.' {
            if l.is_none() {
                l = Some(i);
            }
            r = Some(i);
        }
        if v == '#' {
            if l.is_some() && r.is_some() {
                dot_range.push((l.unwrap(), r.unwrap() + 1));
            }
            l = None;
            r = None;
        }
    }

    for &(l, _) in dot_range.iter() {
        s[l] = 'o'
    }
    println!("{}", s[0..s.len() - 1].iter().join(""));
}
