use ac_library::Dsu;
use num::pow;
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

fn main() {
    input! {
        a: [[usize; 4]; 4],
    }
    let mut ans = 0;
    for i in 0..pow(2, 16) {
        if solve(&a, i) {
            ans += 1;
        }
    }
    println!("{}", ans)
}

fn solve(a: &Vec<Vec<usize>>, state: usize) -> bool {
    let mut b = vec![vec![false; 6]; 6];
    let outside = (0, 0);
    let mut inside = (1, 1);
    for i in 0..16 {
        let a_h = i / 4;
        let a_w = i % 4;
        if state >> i & 1 == 0 && a[a_h][a_w] == 1 {
            return false;
        }
        if state >> i & 1 == 1 {
            b[a_h + 1][a_w + 1] = true;
            inside = (a_h + 1, a_w + 1);
        }
    }

    // 連結判定
    let mut dsu = Dsu::new(36);
    for i in 0..6 {
        for j in 0..6 {
            let now = i * 6 + j;
            if i > 0 {
                let up = (i - 1) * 6 + j;
                if b[i][j] == b[i - 1][j] {
                    dsu.merge(now, up);
                }
            }
            if j > 0 {
                let left = i * 6 + (j - 1);
                if b[i][j] == b[i][j - 1] {
                    dsu.merge(now, left);
                }
            }
        }
    }

    return dsu.size(inside.0 * 6 + inside.1) + dsu.size(outside.0 * 6 + outside.1) == 36;
}
