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
        k: usize,
    }
    let mut l = 0;
    let mut r = s.len() + 1;
    while r - l > 1 {
        let mid = (l + r) / 2;
        let mut ok = false;
        let mut dot_cnt = 0;
        for i in 0..s.len() {
            if s[i] == '.' {
                dot_cnt += 1;
            }

            if i + 1 >= mid {
                if dot_cnt <= k {
                    ok = true;
                    break;
                }
                if s[i + 1 - mid] == '.' {
                    dot_cnt -= 1;
                }
            }
        }
        if ok {
            l = mid;
        } else {
            r = mid;
        }
    }
    println!("{}", l);
}
