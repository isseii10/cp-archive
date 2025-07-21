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
        mut n: Chars,
    }

    n.sort();
    let mut ans = 0;
    // 分離さえできれば逆順にsortして掛け合わせればいい。
    // whileは9!回る、どこで分離するかは10通り
    // 9! * 10 で全然間に合いそう
    // 過去の自分のコードを見返すと、aかbのどっちかに入るんだから、bitで全探索してた...天才
    let mut ok = true;
    while ok {
        for i in 1..n.len() {
            let mut a = n[0..i].to_vec();
            let mut b = n[i..n.len()].to_vec();
            a.sort_by(|c1, c2| c2.cmp(c1));
            b.sort_by(|c1, c2| c2.cmp(c1));
            if a[0] == '0' || b[0] == '0' {
                continue;
            }
            let num_a = a
                .iter()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .fold(0, |acc, d| acc * 10 + d);
            let num_b = b
                .iter()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .fold(0, |acc, d| acc * 10 + d);
            ans = ans.max(num_a * num_b);
        }

        ok = next_permutation(&mut n);
    }

    println!("{}", ans);
}

fn next_permutation<T: Ord>(a: &mut [T]) -> bool {
    let n = a.len();
    if n < 2 {
        return false;
    }

    let mut i = n - 2;
    while i != usize::MAX && a[i] >= a[i + 1] {
        if i == 0 {
            break;
        }
        i -= 1;
    }

    if i == 0 && a[0] >= a[1] {
        return false;
    }

    let mut j = n - 1;
    while a[j] <= a[i] {
        j -= 1;
    }

    a.swap(i, j);
    a[i + 1..].reverse();
    true
}
