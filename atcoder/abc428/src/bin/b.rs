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
        n: usize,
        k: usize,
        s: Chars,
    }
    let mut ans = 0;
    for i in 0..=n - k {
        let a = s[i..i + k].iter().collect::<String>();
        let mut res = 1;
        for j in i + 1..=n - k {
            let b = s[j..j + k].iter().collect::<String>();
            if a == b {
                res += 1;
            }
        }
        ans = max(ans, res);
    }
    let mut anss = vec![];
    for i in 0..=n - k {
        let mut res = 1;
        let a = s[i..i + k].iter().collect::<String>();
        for j in i + 1..=n - k {
            let b = s[j..j + k].iter().collect::<String>();
            if a == b {
                res += 1;
            }
        }
        if res == ans {
            anss.push(a)
        }
    }
    anss.sort();
    println!("{}", ans);
    println!("{}", anss.join(" "));
}
