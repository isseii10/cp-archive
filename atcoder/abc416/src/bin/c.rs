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
        n: usize,
        k: usize,
        x: usize,
        s: [Chars; n],
    }
    let mut result = vec![];
    let mut crr = vec![];
    product(&(0..n).collect_vec(), k, &mut crr, &mut result);

    let mut ans = vec![];
    for res in result {
        let mut ss = String::new();
        for i in 0..k {
            ss.push_str(&s[res[i]].iter().collect::<String>());
        }
        ans.push(ss);
    }
    ans.sort();
    println!("{}", ans[x - 1]);
}

fn product<T: Clone>(values: &[T], k: usize, current: &mut Vec<T>, results: &mut Vec<Vec<T>>) {
    if current.len() == k {
        results.push(current.clone());
        return;
    }
    for v in values {
        current.push(v.clone());
        product(values, k, current, results);
        current.pop();
    }
}
