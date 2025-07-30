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
        q: usize,
    }
    let n: usize = 1 << 20;
    let mask = n - 1;
    let mut a = BTreeMap::<usize, usize>::new();
    // [L, R)の空き領域を管理する. key=R, val=L
    let mut empty_interval = BTreeMap::<usize, usize>::new();
    // [0, n)の空き領域で初期化
    empty_interval.insert(n, 0);
    for _ in 0..q {
        input! {
            t: usize,
            x: usize,
        }
        if t == 1 {
            let mut i = x & mask;
            let (r, l) = empty_interval
                .range(i + 1..)
                .next()
                .map(|(&r, &l)| (r, l))
                .unwrap_or_else(|| {
                    i = 0;
                    let (&r, &l) = empty_interval.iter().next().unwrap();
                    (r, l)
                });
            if l <= i {
                // 空き領域内なので消費
                a.insert(i, x);
                // 空き領域を分割
                empty_interval.remove(&r);
                if l < i {
                    empty_interval.insert(i, l);
                }
                if i + 1 < r {
                    empty_interval.insert(r, i + 1);
                }
            } else {
                // 空き領域外なので[l, r)の空き領域から消費
                a.insert(l, x);
                // 空き領域を更新
                empty_interval.remove(&r);
                if l + 1 < r {
                    empty_interval.insert(r, l + 1);
                }
            }
        } else {
            if let Some(&v) = a.get(&(x & mask)) {
                println!("{}", v);
            } else {
                println!("-1");
            }
        }
    }
}
