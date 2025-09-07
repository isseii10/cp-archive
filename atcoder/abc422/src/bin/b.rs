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
        h: usize,
        w: usize,
        s: [Chars; h],
    }
    let mut ok = true;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                let mut cnt = 0;
                if i > 0 && s[i - 1][j] == '#' {
                    cnt += 1;
                }
                if i + 1 < h && s[i + 1][j] == '#' {
                    cnt += 1;
                }
                if j > 0 && s[i][j - 1] == '#' {
                    cnt += 1;
                }
                if j + 1 < w && s[i][j + 1] == '#' {
                    cnt += 1;
                }

                if cnt != 2 && cnt != 4 {
                    ok = false;
                    break;
                }
            }
        }
    }
    println!("{}", if ok { "Yes" } else { "No" })
}
