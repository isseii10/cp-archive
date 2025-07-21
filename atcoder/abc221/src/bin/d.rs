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
        mut ab: [(usize, usize); n],
    }
    let mut xs = ab
        .iter()
        .flat_map(|&(a, b)| vec![a, a + b])
        .collect::<Vec<_>>();
    xs.sort();
    xs.dedup();
    // println!("xs: {:?}", xs);

    // いもす法
    let mut acc: Vec<isize> = vec![0; xs.len() + 1];

    for (a, b) in ab {
        let login_idx = xs.partition_point(|&x| x < a);
        let logout_idx = xs.partition_point(|&x| x < a + b);
        if login_idx == xs.len() || logout_idx == xs.len() {
            // ありえない
            panic!("error");
        }
        // println!(
        //     "login: {}, logout: {}, login_idx: {}, logout_idx: {}",
        //     a,
        //     a + b,
        //     login_idx,
        //     logout_idx,
        // );
        acc[login_idx + 1] += 1;
        acc[logout_idx + 1] -= 1;
    }

    for i in 0..xs.len() {
        acc[i + 1] += acc[i];
    }
    // println!("{:?}", acc);

    let mut ans = vec![0; n + 1];
    for i in 1..xs.len() {
        ans[acc[i] as usize] += xs[i] - xs[i - 1];
    }

    for i in 1..n {
        print!("{} ", ans[i])
    }
    println!("{}", ans[n]);
}
