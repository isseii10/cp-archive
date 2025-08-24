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
        m: usize,
        s: Chars,
        t: Chars,
        lr: [(usize, usize);m],
    }
    let mut flip = vec![0; n + 1];
    for &(l, r) in lr.iter() {
        flip[l - 1] += 1;
        flip[r] -= 1;
    }
    for i in 0..n {
        flip[i + 1] += flip[i];
    }
    for i in 0..=n {
        flip[i] %= 2;
    }
    let mut ans = String::new();
    for i in 0..n {
        if flip[i] == 0 {
            ans.push(s[i]);
        } else {
            ans.push(t[i]);
        }
    }
    println!("{}", ans);
}
