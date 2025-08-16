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
        s: Chars,
    }
    let mut ans = 0.0;
    for i in 0..s.len() - 1 {
        if s[i] != 't' {
            continue;
        }
        let mut t = 1;
        for j in i + 1..s.len() {
            if s[j] == 't' {
                t += 1;
                if j > i + 1 {
                    let ret = (t as f64 - 2.0) / (j - i - 1) as f64;
                    if ans < ret {
                        ans = ret;
                    }
                }
            }
        }
    }
    println!("{}", ans);
}
