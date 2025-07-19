use num::pow;
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
        t: usize,
    }
    let mut ans = vec![false; t];
    for i in 0..t {
        input! {
            n: usize,
            s: Chars,
        }
        ans[i] = solve(n, &s)
    }

    for i in 0..t {
        if ans[i] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

fn solve(n: usize, s: &Vec<char>) -> bool {
    // 最後の状態から巻き戻っていく
    let end = pow(2, n) - 1;
    if s[end - 1] == '1' {
        return false; // 最後の状態が1なら不可能
    }
    let mut visited = vec![false; end + 1];
    visited[end] = true;
    let mut queue = Deque::new();
    queue.push_back(end);
    while !queue.is_empty() {
        let p = queue.pop_front().unwrap();
        for i in 0..n {
            if p >> i & 1 == 1 {
                let c = p ^ (1 << i);
                if c == 0 {
                    // 0まで辿り着けた
                    return true;
                }
                if s[c - 1] == '1' {
                    continue; // 1の状態は危険
                }
                if visited[c] {
                    continue; // 既に訪問済み
                }
                visited[c] = true;
                queue.push_back(c);
            }
        }
    }
    false
}
