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
        x: Chars,
    }
    // 99999...9があるので、x以上の最小の等差数は同じ桁数
    let n = x.len();
    let x0 = (x[0] as u8 - b'0') as i64;
    let starts = (1..=9).filter(|&d| d >= x0).collect::<Vec<i64>>();
    let mut ans = MAX;
    for &s in starts.iter() {
        for diff in -9..=9 {
            if let Some(ret) = solve(&x, n, diff, s as i64, s != x0 as i64) {
                ans = min(ans, ret);
            }
        }
    }
    println!("{}", ans);
}

fn solve(x: &[char], n: usize, diff: i64, start: i64, over: bool) -> Option<usize> {
    // println!("solve: diff={}, start={}, over={}", diff, start, over);
    let mut over = over;
    let mut disits = vec!['0'; n];
    disits[0] = char::from_digit(start as u32, 10).unwrap();
    for i in 1..n {
        let expected = start + diff * (i as i64);
        if expected < 0 || expected > 9 {
            return None;
        }

        let xi = (x[i] as u8 - b'0') as i64;
        if expected > xi {
            over = true;
        } else if expected < xi {
            if !over {
                return None;
            }
        }
        disits[i] = char::from_digit(expected as u32, 10).unwrap();
    }
    Some(disits.iter().collect::<String>().parse::<usize>().unwrap())
}
