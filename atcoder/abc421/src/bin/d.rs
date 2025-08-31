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
        rc1: (i64, i64),
        rc2: (i64, i64),
        _n: usize,
        m: usize,
        l: usize,
        mut sa: [(char, i64); m],
        mut tb: [(char, i64); l],
    }

    let mut i1 = 0;
    let mut i2 = 0;
    let mut now1 = rc1;
    let mut now2 = rc2;
    let mut ans = 0;
    while i1 < m && i2 < l {
        let (s, a) = sa[i1];
        let (t, b) = tb[i2];

        if a < b {
            i1 += 1;
            tb[i2] = (t, b - a);
            ans += judge(now1, now2, d(s), d(t), a);

            now1 = moved(now1, d(s), a);
            now2 = moved(now2, d(t), a);
        } else if b < a {
            i2 += 1;
            sa[i1] = (s, a - b);
            ans += judge(now1, now2, d(s), d(t), b);
            now1 = moved(now1, d(s), b);
            now2 = moved(now2, d(t), b);
        } else {
            i1 += 1;
            i2 += 1;
            ans += judge(now1, now2, d(s), d(t), a);
            now1 = moved(now1, d(s), a);
            now2 = moved(now2, d(t), b);
        }
    }
    println!("{}", ans);
}

fn moved(rc: (i64, i64), d: (i64, i64), time: i64) -> (i64, i64) {
    let (r, c) = rc;
    let (dr, dc) = d;
    (r + dr * time, c + dc * time)
}

// 最初に同じ位置にいる場合はカウントしない
fn judge(now1: (i64, i64), now2: (i64, i64), d1: (i64, i64), d2: (i64, i64), time: i64) -> usize {
    if d1 == d2 {
        if now1 == now2 {
            return time as usize;
        } else {
            return 0;
        }
    }

    let (x1, y1) = now1;
    let (x2, y2) = now2;
    let (dx1, dy1) = d1;
    let (dx2, dy2) = d2;

    // x1-x2 = (dx2-dx1)*t
    // y1-y2 = (dy2-dy1)*t
    // を満たす整数tが存在するか

    // t = (x2 - x1) / (dx1 - dx2)
    // t = (y2 - y1) / (dy1 - dy2)
    if (x1 - x2) * (dy2 - dy1) == (y1 - y2) * (dx2 - dx1) {
        let mut t = 0;
        if dx1 != dx2 {
            if (x2 - x1) % (dx1 - dx2) != 0 {
                return 0;
            }
            t = (x2 - x1) / (dx1 - dx2);
        } else {
            if (y2 - y1) % (dy1 - dy2) != 0 {
                return 0;
            }
            t = (y2 - y1) / (dy1 - dy2);
        }

        if 0 < t && t <= time {
            return 1;
        } else {
            return 0;
        }
    }

    0
}

fn d(s: char) -> (i64, i64) {
    if s == 'U' {
        (-1, 0)
    } else if s == 'D' {
        (1, 0)
    } else if s == 'L' {
        (0, -1)
    } else {
        (0, 1)
    }
}
