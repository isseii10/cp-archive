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
        mut xy: [(usize, usize); n],
    }
    // 傾きはy / x
    // 区間の右端は(x, y-1)なので、(y-1)/xで比較  (y2-1)*x1 <= (y1-1)*x2
    xy.sort_by(|&(x1, y1), &(x2, y2)| ((y2 - 1) * x1).cmp(&((y1 - 1) * x2)));

    let mut ans = 0;
    let mut last_x = None;
    let mut last_y = None;

    for &(x, y) in &xy {
        if last_x.is_none() || last_y.is_none() {
            ans += 1;
            last_x = Some(x);
            last_y = Some(y);
            continue;
        }
        let lx = last_x.unwrap();
        let ly = last_y.unwrap();
        if (ly - 1) * (x - 1) >= y * lx {
            ans += 1;
            last_x = Some(x);
            last_y = Some(y);
        }
    }

    // let mut ans = 1;
    // let mut l = 0;
    // let mut r = 0;
    // while l < n {
    //     let (xl, yl) = xy[l];
    //     let (mut xr, mut yr) = xy[r];
    //     // lの右端よりrの左端の方が傾き大きいなら選べない
    //     while r < n && (yl - 1) * (xr - 1) < yr * xl {
    //         r += 1;
    //         if r < n {
    //             (xr, yr) = xy[r];
    //         }
    //     }
    //     if r < n {
    //         ans += 1;
    //     }
    //     l = r;
    // }
    println!("{}", ans)
}
