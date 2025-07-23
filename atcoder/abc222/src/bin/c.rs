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
        a: [Chars; 2*n],
    }
    let mut scores: Vec<(Reverse<usize>, usize)> = vec![(Reverse(0), 0); 2 * n];
    for i in 0..2 * n {
        scores[i].1 = i
    }

    for i in 0..m {
        for j in (0..n * 2).step_by(2) {
            let (Reverse(score_x), idx_x) = scores[j];
            let (Reverse(score_y), idx_y) = scores[j + 1];
            let hand_x = a[idx_x][i];
            let hand_y = a[idx_y][i];
            if is_win(hand_x, hand_y) {
                scores[j] = (Reverse(score_x + 1), idx_x)
            }
            if is_win(hand_y, hand_x) {
                scores[j + 1] = (Reverse(score_y + 1), idx_y)
            }
        }
        scores.sort();
    }
    for (_, idx) in scores.iter() {
        println!("{}", idx + 1);
    }
}

fn is_win(me: char, another: char) -> bool {
    (me == 'G' && another == 'C') || (me == 'C' && another == 'P') || (me == 'P' && another == 'G')
}
