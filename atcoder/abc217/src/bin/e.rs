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

fn main() {
    input! {
        q: usize,
    }
    let mut queue: VecDeque<usize> = Deque::new();
    let mut heapq: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
    let mut ans = vec![];
    for _ in 0..q {
        input! {
            q_type: usize,
        }
        if q_type == 1 {
            input! {
                x: usize,
            }
            queue.push_back(x);
        } else if q_type == 2 {
            if heapq.is_empty() {
                let fr = queue.pop_front().unwrap();
                ans.push(fr);
            } else {
                let fr = heapq.pop().unwrap();
                ans.push(fr.0);
            }
        } else {
            while !queue.is_empty() {
                heapq.push(Reverse(queue.pop_front().unwrap()));
            }
        }
    }
    for v in ans.iter() {
        println!("{}", v)
    }
}
