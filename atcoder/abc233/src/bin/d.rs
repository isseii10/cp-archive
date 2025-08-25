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
        k: isize,
        a: [isize; n],
    }
    let mut sa = vec![0; n + 1];
    for i in 0..n {
        sa[i + 1] = sa[i] + a[i];
    }
    let mut ans = 0;
    let mut map: Map<isize, usize> = Map::new();
    map.insert(0, 1);
    for r in 1..=n {
        let v = sa[r] - k;
        if let Some(&cnt) = map.get(&v) {
            ans += cnt;
        }
        map.insert(sa[r], map.get(&sa[r]).unwrap_or(&0) + 1);
    }
    println!("{}", ans);
}
