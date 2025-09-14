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
        r: usize,
        l: [usize; n],
    }
    let mut ans = 0;
    if let Some(asc) = dfs_asc(&l, r) {
        // println!("asc:{}", asc);
        ans += asc;
    }

    if let Some(desc) = dfs_desc(&l, r) {
        // println!("desc:{}", desc);
        ans += desc;
    }
    println!("{}", ans)
}

fn dfs_asc(l: &[usize], now: usize) -> Option<usize> {
    if now == l.len() {
        return None;
    }
    if let Some(ret) = dfs_asc(l, now + 1) {
        // nextにいく必要がある
        let ret = ret + l[now] + 1;
        Some(ret)
    } else {
        if l[now] == 1 {
            None
        } else {
            Some(1)
        }
    }
}

fn dfs_desc(l: &[usize], now: usize) -> Option<usize> {
    if now == 0 {
        return None;
    }
    if let Some(ret) = dfs_desc(l, now - 1) {
        // nextにいく必要がある
        let ret = ret + l[now - 1] + 1;
        Some(ret)
    } else {
        if l[now - 1] == 1 {
            None
        } else {
            Some(1)
        }
    }
}
