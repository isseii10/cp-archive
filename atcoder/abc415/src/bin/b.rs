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
        s: Chars,
    }
    let mut first = None;
    let mut second = None;
    for i in 0..s.len() {
        if s[i] == '#' {
            if first.is_none() {
                first = Some(i + 1);
            } else if second.is_none() {
                second = Some(i + 1);
            }
            if !first.is_none() && !second.is_none() {
                println!("{},{}", first.unwrap(), second.unwrap());
                first = None;
                second = None;
            }
        }
    }
}
