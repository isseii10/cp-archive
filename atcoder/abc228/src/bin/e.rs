use ac_library::ModInt;
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
type Mint = ac_library::ModInt;
#[allow(dead_code)]
const MOD: usize = 998_244_353;
// const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        mut n: usize,
        mut k: usize,
        m: usize,
    }
    // 1以上K以下の長さNの整数列の数 K^N
    // そのそれぞれに1以上M以下の点数をつける M^(K^N)

    if Mint::new(m).val() == 0 {
        println!("0");
        return;
    }

    //
    // フェルマーの小定理
    // a^(p-1) ≡ 1 (mod p)
    // n^k=eとして、 e = q*(p-1)+rとすると、m^e = m^(q*(p-1)+r) ≡ m^r (mod p)
    ModInt::set_modulus(998244353 - 1);
    // NG
    // 指数を先にmod計算すると成り立たない
    // (a ≡ b (mod m)だからといって、x^a ≡ x^b (mod m)とは限らない)
    // let r = ModInt::new(n);
    // let kn = ModInt::new(k).pow(r.val() as u64);
    // OK
    let kn = ModInt::new(k).pow(n as u64);
    ModInt::set_modulus(998244353);
    println!("{}", ModInt::new(m).pow(kn.val() as u64));
}

// 繰り返し二乗法
// b^e mod m を計算する
// O(log e) (eが10^18くらいまで計算可能)
fn mod_pow(base: usize, exp: usize, modulus: usize) -> usize {
    let mut result = 1;
    let mut b = base % modulus;
    let mut e = exp;
    while e > 0 {
        if e % 2 == 1 {
            result = (result * b) % modulus;
        }
        b = (b * b) % modulus;
        e /= 2;
    }
    result
}
