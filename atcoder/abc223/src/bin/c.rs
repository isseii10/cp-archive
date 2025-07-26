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
        ab: [(isize, isize); n],
    }
    let mut c = vec![0.0; n];
    let mut total_s: f64 = 0.0;
    for (i, &(a, b)) in ab.iter().enumerate() {
        let ci = a as f64 / b as f64;
        total_s += ci;
        c[i] = ci;
    }

    let mut target_s = total_s / 2.0;

    let mut ans = 0.0;
    for i in 0..n {
        let (ai, bi) = ab[i];
        let ci = c[i];
        if target_s >= ci {
            ans += ai as f64;
            target_s -= ci;
        } else {
            ans += bi as f64 * target_s;
            break;
        }
    }
    println!("{}", ans);
}
