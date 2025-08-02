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
        m: usize,
        ab: [(usize, usize); m],
    }
    let mut deg = vec![0; n];
    let mut dsu = Dsu::new(n);
    for &(a, b) in ab.iter() {
        dsu.merge(a - 1, b - 1);
        deg[a - 1] += 1;
        deg[b - 1] += 1;
        if deg[a - 1] > 2 || deg[b - 1] > 2 {
            println!("No");
            return;
        }
    }

    let mut edge_num = vec![0; n];
    for &(a, _) in ab.iter() {
        edge_num[dsu.leader(a - 1)] += 1;
    }

    let mut ok = true;
    for g in dsu.groups() {
        let l = dsu.leader(g[0]);
        if g.len() != edge_num[l] + 1 {
            ok = false;
            break;
        }
    }
    if ok {
        println!("Yes");
    } else {
        println!("No");
    }
}
