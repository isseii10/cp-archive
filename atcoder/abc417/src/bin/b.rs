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
        a: [usize; n],
        b: [usize; m],
    }
    let mut cnt = BTreeMap::new();
    for i in 0..n {
        *cnt.entry(a[i]).or_insert(0) += 1;
    }
    for i in 0..m {
        let bi = b[i];
        if let Some(c) = cnt.get_mut(&bi) {
            *c -= 1;
        }
    }
    let mut ans = vec![];
    for (&ai, &c) in cnt.iter() {
        for _ in 0..c {
            ans.push(ai);
        }
    }
    for i in 0..ans.len() {
        if i == ans.len() - 1 {
            println!("{}", ans[i]);
        } else {
            print!("{} ", ans[i]);
        }
    }
}
