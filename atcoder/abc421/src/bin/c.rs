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
        s: Chars,
    }
    let goal1 = (0..n).flat_map(|_| vec!['A', 'B']).collect::<Vec<char>>();
    let goal2 = (0..n).flat_map(|_| vec!['B', 'A']).collect::<Vec<char>>();
    println!("{}", min(solve(&goal1, &s, n), solve(&goal2, &s, n)))
}

fn solve(goal: &[char], s: &[char], n: usize) -> usize {
    let mut ab = Heap::new();
    let mut ba = Heap::new();
    for i in 0..2 * n {
        if goal[i] != s[i] {
            if goal[i] == 'A' {
                ba.push(i);
            } else {
                ab.push(i);
            }
        }
    }

    let mut ans = 0;
    while !ab.is_empty() {
        let i1 = ab.pop().unwrap();
        let i2 = ba.pop().unwrap();
        if i1 < i2 {
            ans += i2 - i1;
        } else {
            ans += i1 - i2;
        }
    }
    // println!("ans: {}", ans);
    ans
}
