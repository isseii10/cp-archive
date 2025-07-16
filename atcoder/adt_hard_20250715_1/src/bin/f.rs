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
        n: usize,
        q: usize,
        mut s: Chars,
        xc: [(usize, char); q],
    }
    let mut abc_set = Set::new();
    abc_set.push('A');
    abc_set.push('B');
    abc_set.push('C');

    let mut ans = 0;
    for i in 0..n - 2 {
        if s[i] == 'A' && s[i + 1] == 'B' && s[i + 2] == 'C' {
            ans += 1;
        }
    }

    for i in 0..q {
        let (x, c) = xc[i];
        let x = x - 1;
        // 減らす処理
        if s[x] == 'A' && x + 2 < n {
            if s[x + 1] == 'B' && s[x + 2] == 'C' {
                ans -= 1;
            }
        }
        if s[x] == 'B' && 0 <= x as isize - 1 && x + 1 < n {
            if s[x - 1] == 'A' && s[x + 1] == 'C' {
                ans -= 1;
            }
        }
        if s[x] == 'C' && 0 <= x as isize - 2 {
            if s[x - 2] == 'A' && s[x - 1] == 'B' {
                ans -= 1;
            }
        }
        // 増やす処理
        if c == 'A' && x + 2 < n {
            if s[x + 1] == 'B' && s[x + 2] == 'C' {
                ans += 1;
            }
        }
        if c == 'B' && 0 <= x as isize - 1 && x + 1 < n {
            if s[x - 1] == 'A' && s[x + 1] == 'C' {
                ans += 1;
            }
        }
        if c == 'C' && 0 <= x as isize - 2 {
            if s[x - 2] == 'A' && s[x - 1] == 'B' {
                ans += 1;
            }
        }
        s[x] = c;
        println!("{}", ans);
    }
}
