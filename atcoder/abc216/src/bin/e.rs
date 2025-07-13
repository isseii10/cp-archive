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
        mut k: usize,
        mut a: [usize; n],
    }
    a.push(0); // lenをn+1にする

    a.sort_by(|x, y| y.cmp(x));

    let mut ans = 0;
    let mut width = 1;
    for i in 0..n {
        // kから引く数 = width * 項数
        if k >= width * (a[i] - a[i + 1]) {
            k -= width * (a[i] - a[i + 1]);
            // 公差1の等差数列の和: 項数 * (初項+末項) / 2
            ans += (a[i] - a[i + 1]) * (a[i + 1] + 1 + a[i]) / 2 * width;
        } else {
            // このiの回で終わる
            let x = k / width;
            ans += x * (2 * a[i] - x + 1) / 2 * width;
            // widthにみたない残りも引く
            ans += (k % width) * (a[i] - x);
            break;
        }
        width += 1;
    }
    println!("{}", ans)
}
