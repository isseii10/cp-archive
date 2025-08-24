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
        mut s: Chars,
    }
    let mut i = 0;
    let mut ok = true;
    let mut ans = vec![];
    while i < s.len() {
        if i + 2 < s.len() && s[i] == 'r' && s[i + 1] == 'e' && s[i + 2] == 'd' {
            i += 3;
            ans.push("SSS");
            continue;
        }
        if i + 3 < s.len() && s[i] == 'b' && s[i + 1] == 'l' && s[i + 2] == 'u' && s[i + 3] == 'e' {
            i += 4;
            ans.push("FFF");
            continue;
        }
        if i + 4 < s.len()
            && s[i] == 'g'
            && s[i + 1] == 'r'
            && s[i + 2] == 'e'
            && s[i + 3] == 'e'
            && s[i + 4] == 'n'
        {
            i += 5;
            ans.push("MMM");
            continue;
        }
        ok = false;
        break;
    }
    if ok {
        println!("{}", ans.join(""));
    } else {
        println!("Unknown");
    }
}
