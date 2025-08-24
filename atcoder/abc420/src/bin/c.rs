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
        mut a: [usize; n],
        mut b: [usize; n],
    }
    let mut ans = 0;
    for i in 0..n {
        ans += min(a[i], b[i]);
    }
    for _ in 0..q {
        input! {
            c: Chars,
            i: usize,
            v: usize,
        }
        let av = a[i - 1];
        let bv = b[i - 1];
        if c[0] == 'A' {
            ans -= min(av, bv);
            ans += min(v, bv);
            a[i - 1] = v;
        } else {
            ans -= min(av, bv);
            ans += min(av, v);
            b[i - 1] = v;
        }
        println!("{}", ans);
    }
}
