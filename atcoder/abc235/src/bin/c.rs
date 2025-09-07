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
        q: usize,
        a: [usize; n],
    }
    let mut indice = Map::new();
    for (i, &v) in a.iter().enumerate() {
        indice
            .entry(v)
            .or_insert_with(Vec::new)
            .push(i as isize + 1);
    }

    for _ in 0..q {
        input! {
            x: usize,
            k: usize,
        }
        println!(
            "{}",
            indice.get(&x).and_then(|v| v.get(k - 1)).unwrap_or(&-1)
        );
    }
}
