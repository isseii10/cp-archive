#[allow(unused_imports)]
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
        a: isize,
        b: isize,
        p: isize,
        q: isize,
        r: isize,
        s: isize,
    }
    let mut ans = vec![vec!['.'; (s - r + 1) as usize]; (q - p + 1) as usize];
    for i in p..=q {
        for j in r..=s {
            let dy = abs(a - i);
            let dx = abs(b - j);
            if dy == dx {
                ans[(i - p) as usize][(j - r) as usize] = '#';
            }
        }
    }
    for a in ans {
        println!("{}", a.into_iter().collect::<String>());
    }
}
