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
        t: usize,
    }
    for _ in 0..t {
        if solve() {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

fn solve() -> bool {
    input! {
        n: usize,
        mut lr: [(usize, usize); n],
    }
    // 貪欲法
    // rの小さい順に見ていきたい。(後続の処理に可能性を多く残せるため)
    lr.sort();
    lr.push((MAX, MAX)); // 番兵
    let mut curr = 1;
    // candidates: 今見ている箱に入れることができるボール。
    // つまり、l <= 今見ている箱 を満たしているボール。
    // 候補の中でrが最小のものを選んで今見ている箱に詰めていく貪欲法をしたいので、rをheapで管理する。
    let mut candidates = Heap::new();

    for (l, r) in lr {
        // 今の箱位置 curr が次の l に到達するまで、
        // 候補から詰められるだけ詰める
        while curr < l && !candidates.is_empty() {
            let Reverse(candidate_r) = candidates.pop().unwrap();
            if candidate_r < curr {
                return false;
            }
            curr += 1;
        }
        curr = l;
        candidates.push(Reverse(r));
    }
    true
}
