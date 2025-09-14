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
    }
    let mut ans = vec![];
    for i in 0..1 << n {
        let mut cnt = 0;
        let mut chars = vec![];
        let mut valid = true;
        for j in 0..n {
            if i >> j & 1 == 1 {
                cnt += 1;
                chars.push('(');
            } else {
                if cnt > 0 {
                    cnt -= 1;
                    chars.push(')');
                } else {
                    valid = false;
                    break;
                }
            }
        }
        if valid && cnt == 0 {
            ans.push(chars.iter().collect::<String>());
        }
    }

    ans.sort();
    for s in ans.iter() {
        println!("{}", s);
    }
}
