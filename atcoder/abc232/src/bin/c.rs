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
        cd: [(usize, usize); m],
    }

    let mut ball = (0..n).collect::<Vec<_>>();
    let edge1 = ab
        .iter()
        .map(|&(a, b)| (a - 1, b - 1))
        .collect::<Set<(usize, usize)>>();
    let edge2 = cd
        .iter()
        .map(|&(a, b)| (a - 1, b - 1))
        .collect::<Set<(usize, usize)>>();

    let mut yes = true;
    let mut has_next = true;
    while has_next {
        yes = true;
        for &(a, b) in edge1.iter() {
            if !edge2.contains(&(ball[a], ball[b])) && !edge2.contains(&(ball[b], ball[a])) {
                yes = false;
                break;
            }
        }
        if yes {
            break;
        }
        has_next = next_permutation(&mut ball);
    }

    println!("{}", if yes { "Yes" } else { "No" });
}

fn next_permutation<T: Ord>(a: &mut [T]) -> bool {
    let n = a.len();
    if n < 2 {
        return false;
    }

    let mut i = n - 2;
    while i != usize::MAX && a[i] >= a[i + 1] {
        if i == 0 {
            break;
        }
        i -= 1;
    }

    if i == 0 && a[0] >= a[1] {
        return false;
    }

    let mut j = n - 1;
    while a[j] <= a[i] {
        j -= 1;
    }

    a.swap(i, j);
    a[i + 1..].reverse();
    true
}
