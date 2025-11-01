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

fn main() {
    input! {
        q: usize,
    }
    let mut not_paired_l = 0;
    let mut not_paired_r = 0;
    let mut history = Deque::new();
    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            input! {
                c: char,
            }

            if c == '(' {
                not_paired_l += 1;
                history.push_back(0);
            } else {
                // c == ')'
                if not_paired_l > 0 {
                    not_paired_l -= 1;
                    history.push_back(1);
                } else {
                    not_paired_r += 1;
                    history.push_back(2);
                }
            }

            if not_paired_r == 0 && not_paired_l == 0 {
                println!("Yes");
            } else {
                println!("No");
            }
        } else {
            if let Some(v) = history.pop_back() {
                if v == 0 {
                    not_paired_l -= 1;
                } else if v == 1 {
                    not_paired_l += 1;
                } else {
                    not_paired_r -= 1;
                }
            }
            if not_paired_r == 0 && not_paired_l == 0 {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}
