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

struct Node {
    front: Option<usize>,
    back: Option<usize>,
}

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut mp = Map::<usize, Node>::new();
    for i in 0..n {
        mp.insert(
            i + 1,
            Node {
                front: None,
                back: None,
            },
        );
    }

    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            input! {
                x: usize,
                y: usize,
            }

            if let Some(node) = mp.get_mut(&x) {
                node.back = Some(y);
            }
            if let Some(node) = mp.get_mut(&y) {
                node.front = Some(x);
            }
        } else if t == 2 {
            input! {
                x: usize,
                y: usize,
            }
            if let Some(node) = mp.get_mut(&x) {
                node.back = None;
            }
            if let Some(node) = mp.get_mut(&y) {
                node.front = None;
            }
        } else if t == 3 {
            input! {
                x: usize,
            }
            let mut curr = x;
            while let Some(next) = mp.get(&curr).unwrap().front {
                curr = next
            }
            let mut ans = vec![];
            ans.push(curr);
            while let Some(next) = mp.get(&curr).unwrap().back {
                ans.push(next);
                curr = next;
            }
            println!(
                "{} {}",
                ans.len(),
                ans.iter()
                    .map(|&v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            )
        }
    }
}
