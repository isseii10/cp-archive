use ac_library::Dsu;
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

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        mut abc: [(usize, usize, usize); m],
        uvw: [(usize, usize, usize); q],
    }
    // クエリの辺もまとめる
    let mut all_e = [
        // moveしていい場合はそのまま使ってもいい
        // abc,
        // uvw,
        abc.iter().cloned().collect_vec(),
        uvw.iter().cloned().collect_vec(),
    ]
    .concat();
    all_e.sort_by_key(|&(_, _, w)| w);

    // どの辺がどのクエリか覚えておく
    let mut w = uvw
        .iter()
        .enumerate()
        .map(|(i, &(_, _, w))| (i, w))
        .collect_vec();
    w.sort_by_key(|&(_, w)| w);

    let mut dsu = Dsu::new(n + 1);

    let mut ans = vec![false; q];
    let mut iw = 0;
    for &(a, b, c) in &all_e {
        if iw < q && c == w[iw].1 {
            let iq = w[iw].0;
            if !dsu.same(a, b) {
                ans[iq] = true;
            }
            iw += 1;
        } else {
            if !dsu.same(a, b) {
                dsu.merge(a, b);
            }
        }
    }
    for &a in ans.iter() {
        println!("{}", if a { "Yes" } else { "No" });
    }
}
