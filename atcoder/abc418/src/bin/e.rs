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
        n: usize,
        xy: [(i64, i64); n],
    }
    let mut a_map: HashMap<(i64, i64), HashMap<i64, i64>> = Map::new();
    for i in 0..n - 1 {
        let (xi, yi) = xy[i];
        for j in i + 1..n {
            let (xj, yj) = xy[j];

            let mut dx = xj - xi;
            let mut dy = yj - yi;
            if dx < 0 {
                dx = -dx;
                dy = -dy;
            } else if dx == 0 && dy < 0 {
                dy = -dy;
            }

            let g = if dy > 0 {
                gcd(dx as usize, dy as usize) as i64
            } else {
                gcd(dx as usize, -dy as usize) as i64
            };

            let a = (dx / g as i64, dy / g as i64);
            // println!("{:?}", a);
            if let Some(g_map) = a_map.get_mut(&a) {
                if let Some(v) = g_map.get_mut(&g) {
                    *v += 1;
                } else {
                    g_map.insert(g, 1);
                }
            } else {
                let mut g_map: HashMap<i64, i64> = Map::new();
                g_map.insert(g, 1);
                a_map.insert(a, g_map);
            }
        }
    }

    // println!("{:?}", a_map);
    let mut ans = 0;
    let mut minus = 0;
    for (_, g_map) in a_map.iter() {
        let mut cnt = 0;
        for (_, v) in g_map.iter() {
            cnt += v;
            if *v > 1 {
                minus += v * (v - 1) / 2;
            }
        }
        ans += cnt * (cnt - 1) / 2;
    }

    // println!("ans {}", ans);
    // println!("minus {}", minus);
    println!("{}", ans - minus / 2);
}

// 最大公約数を求めるユークリッドの互除法 O(log(min(a, b)))
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}
