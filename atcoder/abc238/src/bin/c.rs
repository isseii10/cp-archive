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
// type Mint = ac_library::ModInt1000000007;

fn main() {
    input! {
        n: usize,
    }
    let mut nn = n;
    let mut d = 0;
    while nn > 0 {
        nn /= 10;
        d += 1;
    }

    let mut ans = Mint::new(0);
    let mut ten = 1;
    for i in 1..=d {
        if i == d {
            let s = 1;
            let e = n - ten + 1;
            let m = e - s + 1;
            ans += m * (s + e) / 2;
        } else if i == 1 {
            let s = 1;
            let e = ten * 10 - 1;
            let m = e - s + 1;
            ans += m * (s + e) / 2;
            println!("d:{} s:{} e:{} m:{}", i, s, e, m);
        } else {
            let s = 1;
            let e = ten;
            let m = e - s + 1;
            ans += m * (s + e) / 2;
            println!("d:{} s:{} e:{} m:{}", i, s, e, m);
        }
        // println!("d:{} ans:{}", i, ans);
        ten *= 10;
    }
    println!("{}", ans.val());
}
