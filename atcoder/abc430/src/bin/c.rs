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
        a: usize,
        b: usize,
        s: Chars,
    }
    let mut a_acc = vec![0; n + 1];
    let mut b_acc = vec![0; n + 1];
    for i in 0..n {
        a_acc[i + 1] = a_acc[i] + if s[i] == 'a' { 1 } else { 0 }
    }
    for i in 0..n {
        b_acc[i + 1] = b_acc[i] + if s[i] == 'b' { 1 } else { 0 }
    }

    // 尺取法
    // NOTE: 尺取法は実質ok/ngの境界探し
    // 二分探索と本質的に変わらない
    // ok/ngのどっちを指すポインタを持っているかに注意すればバグりにくい
    let mut ans = 0;
    let mut ra = 0;
    let mut rb = 0;
    for l in 0..=n {
        while ra <= n && a_acc[ra] - a_acc[l] < a {
            ra += 1;
        }
        // この時点でraはa未満とa以上の境界の"a以上の方"を指している

        while rb <= n && b_acc[rb] - b_acc[l] < b {
            rb += 1;
        }
        // この時点でrbはb未満とb以上の境界の"b以上の方"を指している

        // 答えはrb-ra
        ans += rb.saturating_sub(ra);
    }
    println!("{}", ans)
}
