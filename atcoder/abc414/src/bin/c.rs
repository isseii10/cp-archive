use num::pow;
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
        a: usize,
        n: usize,
    }
    // println!("{:?}", n)
    let mut candidate = vec![];

    let mut pow10 = vec![];
    for i in 0..=6 {
        pow10.push(pow(10, i))
    }

    // 12桁の回文
    for l in 1..=6 {
        // lは回文の長さの半分
        for i in pow10[l - 1]..pow10[l] {
            let i = i as usize;
            let mut tmp1 = i * pow10[l];
            // tmp1: 10の時、1001を作りたい
            let mut tmp2 = i * pow10[l - 1];
            // tmp2: 10の時、101を作りたい

            // iのdisitを反転していく
            let mut ii = i;
            let mut disit_pos = l - 1;
            while ii > 0 {
                let disit = ii % 10;
                tmp1 += disit * pow10[disit_pos as usize];
                if disit_pos < l - 1 {
                    // 被るので l-1の時以外足す
                    tmp2 += disit * pow10[disit_pos as usize];
                }
                ii /= 10;
                disit_pos = disit_pos.saturating_sub(1);
            }
            candidate.push(tmp1);
            candidate.push(tmp2);
        }
    }

    // println!("{:?}", candidate);
    let mut ans = 0;
    for i in candidate {
        if i > n {
            continue;
        }
        if base(a, i) {
            ans += i
        }
    }
    println!("{}", ans)
}

// 逆順のままだけどこの問題はこのままで良い
fn base(a: usize, n: usize) -> bool {
    let mut ret: Vec<char> = vec![];
    let mut nn = n;
    while nn > 0 {
        let b = nn % a;
        let bchar = std::char::from_digit(b as u32, 10).unwrap();
        ret.push(bchar);
        nn /= a
    }
    // println!("n:{}, ret: {:?}", n, ret);

    for i in 0..=ret.len() / 2 {
        if ret[i] != ret[ret.len() - 1 - i] {
            return false;
        }
    }
    return true;
}
