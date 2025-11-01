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
// type Mint = ac_library::ModInt1000000007;
const INF: usize = 1_000_000_000_000;

fn main() {
    input! {
        n: usize,
        x: [usize; n],
    }
    let mut pos = BTreeMap::new();
    pos.push((0, INF));
    let mut ans = INF;
    for &now_pos in x.iter() {
        let mut now_val = INF;
        if let Some((&before_pos, &before_val)) = pos.range(..now_pos).next_back() {
            // println!("[b]pos:{} val:{}", before_pos, before_val);
            let new_before_val = now_pos - before_pos;
            if before_val > new_before_val {
                // ansの更新
                ans -= before_val;
                ans += new_before_val;
                // posの更新
                *pos.get_mut(&before_pos).unwrap() = new_before_val;
            }
            now_val = min(now_val, new_before_val);
        }
        if let Some((&after_pos, &after_val)) = pos.range(now_pos..).next() {
            // println!("[a]pos:{} val:{}", after_pos, after_val);
            let new_after_val = after_pos - now_pos;
            if after_val > new_after_val {
                // ansの更新
                ans -= after_val;
                ans += new_after_val;
                // posの更新
                *pos.get_mut(&after_pos).unwrap() = new_after_val;
            }
            now_val = min(now_val, new_after_val)
        }
        ans += now_val;
        pos.push((now_pos, now_val));
        println!("{}", ans)
    }
}
