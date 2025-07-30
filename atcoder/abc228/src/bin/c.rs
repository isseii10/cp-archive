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
        k: usize,
        p: [[usize; 3]; n],
    }
    let mut sum = vec![0; n];
    for i in 0..n {
        for j in 0..3 {
            sum[i] += p[i][j]
        }
    }
    let mut sum_i = sum.iter().enumerate().collect::<Vec<_>>();
    sum_i.sort_by_key(|&(_, s)| Reverse(*s));

    let mut ans = vec![false; n];

    for i in 0..n {
        if i < k {
            let (idx, _) = sum_i[i];
            ans[idx] = true;
        } else {
            let (_, &k_score) = sum_i[k - 1];
            let (idx, &score) = sum_i[i];
            if score + 300 >= k_score {
                ans[idx] = true
            }
        }
    }
    for i in 0..n {
        println!("{}", if ans[i] { "Yes" } else { "No" });
    }
}
