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
        x: usize,
        y: usize,
        abc: [usize; 3],
    }
    let mut ok = false;
    for i in 0..3 {
        let a = abc[i];
        // b, cは順不同
        let b = abc[(i + 1) % 3];
        let c = abc[(i + 2) % 3];

        // y軸いっぱいにaを広げた時、消費するx軸の幅: ax
        let ax = (a + y - 1) / y;
        // x軸いっぱいにaを広げた時、消費するy軸の幅: ay
        let ay = (a + x - 1) / x;
        if x.saturating_sub(ax) > 0 {
            // a|b|cのパターンとa|b/cのパターン
            ok = ok || is_ok(x - ax, y, b, c) || is_ok(y, x - ax, b, c);
        }
        if y.saturating_sub(ay) > 0 {
            // a|b|cのパターンとa|b/cのパターン
            ok = ok || is_ok(x, y - ay, b, c) || is_ok(y - ay, x, b, c);
        }
    }
    println!("{}", if ok { "Yes" } else { "No" });
}

// y軸いっぱいにb, cを広げたとき、xに収まるか
fn is_ok(x: usize, y: usize, b: usize, c: usize) -> bool {
    (b + y - 1) / y + (c + y - 1) / y <= x
}
