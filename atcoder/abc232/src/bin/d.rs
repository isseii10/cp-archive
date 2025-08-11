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
        h: usize,
        w: usize,
        c: [Chars; h],
    }
    let mut q = Deque::new();
    let mut dist = vec![vec![MAX; w]; h];
    q.push_back((0 as usize, 0 as usize));
    dist[0][0] = 1;
    let mut ans = 1;
    while let Some((ch, cw)) = q.pop_front() {
        let nh = ch + 1;
        if nh != h && c[nh][cw] != '#' && dist[nh][cw] == MAX {
            dist[nh][cw] = dist[ch][cw] + 1;
            q.push_back((nh, cw));
            ans = max(ans, dist[nh][cw]);
        }
        let nw = cw + 1;
        if nw != w && c[ch][nw] != '#' && dist[ch][nw] == MAX {
            dist[ch][nw] = dist[ch][cw] + 1;
            q.push_back((ch, nw));
            ans = max(ans, dist[ch][nw]);
        }
    }
    println!("{}", ans);
}
