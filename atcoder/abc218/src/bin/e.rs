use ac_library::Dsu;
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
        m: usize,
        mut abc: [(usize, usize, isize); m],
    }
    // 負の辺からくっつけていって、連結になった時の残りの辺の和
    // ただし、連結になってもまだ負の辺の時は使う
    let mut dsu = Dsu::new(n + 1);
    abc.sort_by(|&(_, _, c1), (_, _, c2)| c1.cmp(c2));
    let mut ans = 0;
    for (a, b, c) in abc {
        // println!("a:{}, b:{}, c: {}", a, b, c);
        if c <= 0 {
            dsu.merge(a, b);
            continue;
        }
        if dsu.same(a, b) {
            // わざわざくっつけなくていい
            // println!("same: a:{}, b:{}", a, b);
            ans += c;
        } else {
            // println!("not same: a:{}, b:{}", a, b);
            dsu.merge(a, b);
        }
    }

    println!("{}", ans)
}
