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
        a: usize,
        n: usize,
    }
    let m = 1_000_000;
    let mut dist = vec![MAX; m];
    let mut q = Deque::new();
    dist[1] = 0;
    q.push_back(1 as usize);

    while let Some(p) = q.pop_front() {
        let c1 = p * a;
        if c1 <= m && dist[c1] > dist[p] + 1 {
            dist[c1] = dist[p] + 1;
            q.push_back(c1);
        }
        if p >= 10 && p % 10 != 0 {
            let mut p_chars = p.to_string().chars().collect::<Vec<char>>();
            p_chars.rotate_right(1);
            let c2 = p_chars.iter().collect::<String>().parse::<usize>().unwrap();
            if c2 <= m && dist[c2] > dist[p] + 1 {
                dist[c2] = dist[p] + 1;
                q.push_back(c2);
            }
        }
    }

    println!("{}", if dist[n] == MAX { -1 } else { dist[n] as isize });
}
