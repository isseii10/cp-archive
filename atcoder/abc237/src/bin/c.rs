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
// type Mint = ac_library::ModInt1000000007;

fn main() {
    input! {
        s: Chars,
    }
    let n = s.len();
    let mut end = 0;
    for i in (0..n).rev() {
        if s[i] == 'a' {
            end += 1;
        } else {
            break;
        }
    }
    let mut start = 0;
    for i in 0..n {
        if s[i] == 'a' {
            start += 1;
        } else {
            break;
        }
    }
    let mut b = vec![];
    for _ in start..end {
        b.push('a');
    }
    for i in 0..n {
        b.push(s[i])
    }

    println!("{}", if check(&b) { "Yes" } else { "No" })
}

fn check(s: &[char]) -> bool {
    let n = s.len();
    let m = if n % 2 == 0 { n / 2 } else { n / 2 + 1 };
    for i in 0..m {
        if s[i] != s[n - 1 - i] {
            return false;
        }
    }
    return true;
}
