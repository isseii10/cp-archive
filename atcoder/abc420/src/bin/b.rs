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
        s: [Chars; n],
    }
    let mut scores = vec![0; n];
    let mut max_score = 0;
    for j in 0..m {
        let mut x = 0;
        let mut y = 0;
        for i in 0..n {
            if s[i][j] == '0' {
                x += 1;
            } else {
                y += 1;
            }
        }
        if x == 0 || y == 0 {
            continue;
        }
        if x < y {
            for i in 0..n {
                if s[i][j] == '0' {
                    scores[i] += 1;
                    max_score = max(max_score, scores[i]);
                }
            }
        } else {
            for i in 0..n {
                if s[i][j] == '1' {
                    scores[i] += 1;
                    max_score = max(max_score, scores[i]);
                }
            }
        }
    }
    let mut ans = vec![];
    for i in 0..n {
        if scores[i] == max_score {
            ans.push(i + 1);
        }
    }

    for i in 0..ans.len() {
        if i < ans.len() - 1 {
            print!("{} ", ans[i]);
        } else {
            println!("{}", ans[i]);
        }
    }
}
