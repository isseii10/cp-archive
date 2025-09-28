#[allow(unused_imports)]
use amplify::confinement::Collection;
use itertools::Itertools;
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
// type Mint = ac_library::ModInt1000000007;

fn main() {
    input! {
        n: usize,
        a: [isize; n],
    }
    let mut used = vec![false; n + 1];
    let mut p = vec![0; n + 1];
    let mut count = 0;
    for i in 0..n {
        if a[i] != -1 {
            used[a[i] as usize] = true;
            p[i] = a[i] as usize;
        } else {
            count += 1;
        }
    }
    let mut unused = vec![];
    for i in 1..=n {
        if !used[i] {
            unused.push(i);
        }
    }

    let ok = unused.len() == count;
    if !ok {
        println!("No");
    } else {
        println!("Yes");
        let mut ans = vec![];
        for i in 0..n {
            if p[i] == 0 {
                p[i] = unused.pop().unwrap();
            }
            ans.push(p[i]);
        }
        println!("{}", ans.iter().join(" "))
    }
}
