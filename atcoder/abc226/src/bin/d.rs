use amplify::confinement::Collection;
use num::abs;
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
        xy: [(isize, isize); n],
    }
    let mut set = Set::new();
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let (xi, yi) = xy[i];
            let (xj, yj) = xy[j];
            // i -> j
            let mut dx = xj - xi;
            let mut dy = yj - yi;
            let dx_abs = abs(dx) as usize;
            let dy_abs = abs(dy) as usize;
            let g = gcd(dx_abs, dy_abs);
            dx /= g as isize;
            dy /= g as isize;
            set.push((dx, dy));
        }
    }
    println!("{}", set.len());
}

// 最大公約数を求めるユークリッドの互除法 O(log(min(a, b)))
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}
