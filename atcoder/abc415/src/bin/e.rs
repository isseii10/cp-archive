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
        h: usize,
        w: usize,
        mut a: [[isize;w]; h],
        p: [isize; h+w-1],
    }

    for i in 0..h {
        for j in 0..w {
            a[i][j] -= p[i + j]
        }
    }

    let mut ok: isize = 1_000_000_000_000_000;
    let mut ng: isize = -1;
    while ok - ng > 1 {
        let mid = (ok + ng) / 2;
        if is_ok(h, w, &a, mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{}", ok)
}

fn is_ok(h: usize, w: usize, grid: &Vec<Vec<isize>>, x: isize) -> bool {
    let mut benefit = vec![vec![MIN; w]; h];
    benefit[0][0] = grid[0][0] + x;
    for i in 0..h {
        for j in 0..w {
            if benefit[i][j] < 0 {
                benefit[i][j] = MIN;
                continue;
            }
            if i + 1 < h {
                benefit[i + 1][j] = max(benefit[i + 1][j], benefit[i][j] + grid[i + 1][j]);
            }
            if j + 1 < w {
                benefit[i][j + 1] = max(benefit[i][j + 1], benefit[i][j] + grid[i][j + 1]);
            }
        }
    }

    // println!("{:?}", benefit);
    benefit[h - 1][w - 1] >= 0
}
