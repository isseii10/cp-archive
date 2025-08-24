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
        x: usize,
    }
    let mut l = vec![];
    let mut a = vec![];
    for _ in 0..n {
        input! {
            li: usize,
            ai: [usize; li],
        }
        l.push(li);
        a.push(ai);
    }
    let mut ans = 0;
    for &ai in a[0].iter() {
        dfs(&a, n, 1, x, ai, &mut ans);
    }
    println!("{}", ans);
}

fn dfs(a: &Vec<Vec<usize>>, n: usize, i: usize, x: usize, curr: usize, ans: &mut usize) {
    for &c in a[i].iter() {
        if x % c != 0 {
            continue;
        }
        // println!("i: {}, curr: {}, c: {}", i, curr, c);

        if i == n - 1 {
            if curr == x / c {
                *ans += 1;
                // println!("i: {}, curr: {}, c: {}", i, curr, c);
            }
        } else if curr <= x / c {
            dfs(a, n, i + 1, x, curr * c, ans);
        }
    }
}
