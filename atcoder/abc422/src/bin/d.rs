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
        mut k: usize,
    }
    let two = (0..n + 1).map(|i| 1 << i).collect::<Vec<usize>>();
    let mut arr = vec![k / two[n]; two[n]];
    k -= k / two[n] * two[n];
    let b = if k > 0 { 1 } else { 0 };

    if k > 0 {
        for i in 0..two[n] {
            let mut idx = 0;
            for j in 0..n {
                if (i >> j) & 1 == 1 {
                    idx += two[n - 1 - j];
                }
            }
            // println!("{}", idx);
            arr[idx] += 1;
            k -= 1;
            if k == 0 {
                break;
            }
        }
    }

    println!("{}", b);
    for i in 0..two[n] {
        if i == two[n] - 1 {
            println!("{}", arr[i])
        } else {
            print!("{} ", arr[i]);
        }
    }
}
