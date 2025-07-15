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

fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n],
    }
    let mut ans = 0;
    let mut set = Set::new();
    for &v in xy.iter() {
        set.push(v);
    }
    let mut counted = Set::new();

    for i in 0..n - 1 {
        for j in i + 1..n {
            let (x1, y1) = xy[i];
            let (x2, y2) = xy[j];
            if x1 == x2 || y1 == y2 {
                continue;
            }

            // (x1, y2), (x2, y1)を見つける
            if set.contains(&(x1, y2)) && set.contains(&(x2, y1)) {
                // left_top, right_bottomを見つける
                let rect = vec![(x1, y1), (x2, y2), (x1, y2), (x2, y1)];
                let left_top = find_left_top(&rect);
                let right_bottom = find_right_bottom(&rect);

                if counted.contains(&(left_top, right_bottom)) {
                    continue;
                }
                // println!("counted");
                counted.push((left_top, right_bottom));
                ans += 1
            }
        }
    }

    println!("{}", ans);
}

fn find_left_top(rect: &Vec<(usize, usize)>) -> (usize, usize) {
    let mut left = MAX;
    let mut top = MAX;
    for &(x, y) in rect {
        left = left.min(x);
        top = top.min(y);
    }

    (left, top)
}
fn find_right_bottom(rect: &Vec<(usize, usize)>) -> (usize, usize) {
    let mut right = 0;
    let mut bottom = 0;
    for &(x, y) in rect {
        right = right.max(x);
        bottom = bottom.max(y);
    }

    (right, bottom)
}
