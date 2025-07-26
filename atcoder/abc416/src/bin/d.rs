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
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            m: usize,
            mut a: [usize; n],
            mut b: [usize; n],
        }
        a.sort();
        b.sort();

        let mut l = 0;
        let mut r = n + 1;
        while r - l > 1 {
            let mid = (l + r) / 2;
            // a+bの和がm以上になる組み合わせがmid個以上あるか？
            let mut ok = true;
            for i in 0..mid {
                if a[n - 1 - i] + b[n - mid + i] < m {
                    ok = false;
                    break;
                }
            }
            if ok {
                l = mid
            } else {
                r = mid
            }
        }

        let mut ans = a.iter().sum::<usize>() + b.iter().sum::<usize>();
        ans -= l * m;
        println!("{}", ans)
    }
}
