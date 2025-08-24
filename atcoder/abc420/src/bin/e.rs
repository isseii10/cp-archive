use ac_library::Dsu;
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
    }
    let mut dsu = Dsu::new(n + 1);
    let mut blacks = vec![0; n + 1];
    let mut is_black = vec![false; n + 1];

    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            input! {
                u: usize,
                v: usize,
            }
            let blacks_u = blacks[dsu.leader(u)];
            let blacks_v = blacks[dsu.leader(v)];
            if !dsu.same(u, v) {
                dsu.merge(u, v);
                blacks[dsu.leader(u)] = blacks_u + blacks_v;
            }
        } else if t == 2 {
            input! {
                u: usize,
            }
            if is_black[u] {
                blacks[dsu.leader(u)] -= 1;
                is_black[u] = false;
            } else {
                blacks[dsu.leader(u)] += 1;
                is_black[u] = true;
            }
        } else {
            input! {
                u: usize,
            }
            if blacks[dsu.leader(u)] > 0 {
                println!("Yes")
            } else {
                println!("No")
            }
        }
    }
}
