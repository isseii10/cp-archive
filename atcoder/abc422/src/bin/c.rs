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
        t: usize,
    }
    for _ in 0..t {
        input! {
            na: usize,
            nb: usize,
            nc: usize,
        }
        let mut ok = 0;
        let mut ng = 10usize.pow(9) + 10;
        while ng - ok > 1 {
            let mid = (ok + ng) / 2;
            if judge(na, nb, nc, mid) {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        println!("{}", ok);
    }
}

fn judge(na: usize, nb: usize, nc: usize, x: usize) -> bool {
    if na < x {
        return false;
    }
    if nc < x {
        return false;
    }

    let na = na - x;
    let nc = nc - x;
    return na + nb + nc >= x;
}
