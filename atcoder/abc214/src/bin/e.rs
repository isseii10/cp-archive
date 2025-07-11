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
    lr.push((MAX, MAX));
    let mut curr_box = 1;
    // candidate_balls: 今見ている箱に入れることができるボール。
    // つまり、l <= 今見ている箱 を満たしているボール。
    // 候補の中でrが最小のものを選んで今見ている箱に詰めていく貪欲法をしたいので、rをheapで管理する。
    let mut candidate_balls: BinaryHeap<Reverse<usize>> = Heap::new();
    for (l, r) in lr {
        // l <= curr_box の場合は今見ている区間も候補に入る。候補がまだ出揃っていないので詰める処理(whileの中)に入ることはできない。
        while curr_box < l && !candidate_balls.is_empty() {
            // 候補が出揃ったなら、貪欲に詰めていく
            let candidate_ball_r = candidate_balls.pop().unwrap().0;
            if candidate_ball_r < curr_box {
                // 詰めようとしたが、今見ている箱がすでに範囲外になってしまっているなら、もうだめ
                return false;
            }

            // 今見ている箱に詰めたので次の箱へ
            curr_box += 1;
        }
        // lまで飛ばす。つまり今見ている区間[l, r]はcandidateに入る
        curr_box = l;
        candidate_balls.push(Reverse(r));
    }
    true
}
