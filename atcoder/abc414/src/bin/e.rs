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

use ac_library::modint::ModInt998244353;

fn main() {
    input! {
        n: usize,
    }
    let mut ans = ModInt998244353::new(n) * (n + 1) / 2;
    // 愚直に計算(n<=10^12なので無理)
    // for b in 1..=n {
    //     ans -= n / b
    // }

    // 高速化
    // 出現するn/bの値はsqrt(n)くらいしかないので、O(sqrt(n))になる
    // なぜ？:
    // 実数でy = n/xのグラフを考えた時に、x = sqrt(n)でxとyの大小が変わる。
    // x < sqrt(n) の時、xの取りうる整数値がたかだかsqrt(n)個しかないから、出現するfloor(n/x)の個数もたかだかsqrt(n)
    // sqrt(n) < x の時、yの取りうる整数値がたかだかsqrt(n)個しかないから、出現するfloor(n/x)の個数もたかだかsqrt(n)
    // 例:
    // n = 50のとき(sqrt(n)~7)
    // b:    1,  2,  3,  4,  5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,...
    // n/b: 50, 25, 16, 12, 10, 8, 6, 6, 5,  5,  4,  4,  3,  3,  3,  3,  2,  2,  2,  2,...
    // b=7まではn/bの出現個数は7個
    // b=7以降のn/bの出現個数は6個より大きくなることはない
    // 実装方針:
    // n/bが同じになるbはまとめて計算して、次にn/bが変わるところまでbを飛ばしていく
    let mut b = 1;
    while b <= n {
        // x = n/b
        let x = n / b;
        let nb = n / x + 1;
        // println!("b: {}, n/b: {}, nb: {}", b, x, nb);
        ans -= (nb - b) * x;
        b = nb
    }
    println!("{}", ans.val())
}
