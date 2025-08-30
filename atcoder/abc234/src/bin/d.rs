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
        k: usize,
        p: [usize; n],
    }
    let mut hq = BinaryHeap::new();
    for i in 0..n {
        if i < k {
            hq.push(Reverse(p[i]));
            if i == k - 1 {
                if let Some(Reverse(min)) = hq.peek() {
                    println!("{}", min);
                }
            }
            continue;
        }

        if let Some(Reverse(now_min)) = hq.pop() {
            // 今入っている最小値よりも大きければ交換
            if now_min < p[i] {
                hq.push(Reverse(p[i]));
                if let Some(Reverse(next_min)) = hq.peek() {
                    println!("{}", next_min);
                }
            } else {
                hq.push(Reverse(now_min));
                println!("{}", now_min);
            }
        }
    }
}
