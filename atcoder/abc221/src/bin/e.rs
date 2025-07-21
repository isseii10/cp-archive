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
        a: [usize; n],
    }
    // 座標圧縮
    let mut mp = BTreeMap::new();
    for &v in a.iter() {
        mp.push((v, 0 as usize));
    }
    let mut idx: usize = 0;
    for (_, v) in mp.iter_mut() {
        *v = idx;
        idx += 1;
    }
    // println!("{:?}", mp);

    let mut fenwick = ac_library::FenwickTree::new(idx, Mint::new(0));
    let mut pow_2 = Mint::new(1);
    let mut pow_inv2 = Mint::new(1) / 2;
    let mut ans = Mint::new(0);
    for i in 0..n {
        let r = a[i];
        let fenwick_idx = mp[&r];
        ans += fenwick.sum(0..fenwick_idx + 1) * pow_2;
        fenwick.add(fenwick_idx, pow_inv2);

        pow_2 *= 2;
        pow_inv2 /= 2;
    }
    println!("{}", ans.val());
}
