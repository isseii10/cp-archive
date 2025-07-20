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
        m: usize,
        a: [usize; n],
    }
    let (_, min_factor) = eratosthenes(100_001);
    let mut sieve = vec![true; 100_001];
    for v in a {
        for (p, _) in fast_prime_factorize(v, &min_factor) {
            if sieve[p] {
                let mut q = p;
                while q <= m {
                    sieve[q] = false;
                    q += p;
                }
            }
        }
    }
    let ans: Vec<usize> = (1..=m).filter(|&x| sieve[x]).collect();
    println!("{}", ans.len());
    for a in ans {
        println!("{}", a)
    }
}

// エラトステネスの篩 O(nloglogn)
// 定数倍改善1: pのforは2..=sqrt(n)で良い
// 定数倍改善2: qのforはp*p..=nで良い
// NOTE: 以下は基本形からmin_factorも返すように修正。
// 素数判定の過程で応用が効くので、このコードに手を加えて利用することもあるかも
fn eratosthenes(n: usize) -> (Vec<usize>, Vec<usize>) {
    let mut is_prime = vec![true; n + 1];
    let mut min_factor = vec![0; n + 1]; // 最小素因数

    is_prime[0] = false;
    is_prime[1] = false;
    min_factor[1] = 1; // 1は素数ではないが、最小素因数を1にしておく

    for p in 2..=n {
        if !is_prime[p] {
            continue;
        }
        min_factor[p] = p;
        // pが素数なので、2*pからふるいおとしていく
        for q in (p * 2..=n).step_by(p) {
            is_prime[q] = false;
            if min_factor[q] == 0 {
                min_factor[q] = p;
            }
        }
    }
    let primes = (0..=n).filter(|&x| is_prime[x]).collect();
    (primes, min_factor)
}

// 素因数分解 素因数とその指数をタプルで返す
// NOTE: 事前にエラトステネスの篩でmin_factorを取得する
fn fast_prime_factorize(mut n: usize, min_factor: &[usize]) -> Vec<(usize, usize)> {
    let mut res = vec![];
    while n > 1 {
        let p = min_factor[n];
        let mut exp = 0;
        while min_factor[n] == p {
            n /= p;
            exp += 1;
        }
        res.push((p, exp));
    }
    res
}
