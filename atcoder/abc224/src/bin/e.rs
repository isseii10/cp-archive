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
        h: usize,
        w: usize,
        n: usize,
        rca: [(usize, usize, usize); n],
    }
    let mut arci = BTreeMap::<Reverse<usize>, Vec<(usize, usize, usize)>>::new();

    for (i, &(r, c, a)) in rca.iter().enumerate() {
        if let Some(v) = arci.get_mut(&Reverse(a)) {
            v.push((r - 1, c - 1, i));
        } else {
            arci.insert(Reverse(a), vec![(r - 1, c - 1, i)]);
        }
    }

    let mut h_max = vec![-1; h];
    let mut w_max = vec![-1; w];

    let mut ans = vec![0; n];

    for (_, v) in arci.iter() {
        for &(r, c, i) in v.iter() {
            ans[i] = max(h_max[r], w_max[c]) + 1;
        }
        for &(r, c, i) in v.iter() {
            h_max[r] = h_max[r].max(ans[i]);
            w_max[c] = w_max[c].max(ans[i]);
        }
    }

    for v in ans {
        println!("{}", v);
    }
}
