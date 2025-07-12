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
        m: usize,
        a: [usize; n],
    }
    let primes = eratosthenes(100_001);
    let mut prime_factors = Set::new();
    for e in a {
        for pf in prime_factorize(e, &primes) {
            prime_factors.push(pf);
        }
    }
    let mut ans = vec![];
    ans.push(1);
    for i in 2..=m {
        let pf = prime_factorize(i, &primes);
        let mut ok = true;
        for p in pf {
            if prime_factors.contains(&p) {
                ok = false;
                break;
            }
        }
        if ok {
            ans.push(i);
        }
    }
    println!("{}", ans.len());
    for a in ans {
        println!("{}", a)
    }
}

fn eratosthenes(n: usize) -> Vec<usize> {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for p in 2..=n {
        if !is_prime[p] {
            continue;
        }
        // pが素数なので、2*pからふるいおとしていく
        for q in (p * 2..=n).step_by(p) {
            is_prime[q] = false;
        }
    }
    (0..=n).filter(|&x| is_prime[x]).collect()
}

fn prime_factorize(mut n: usize, primes: &[usize]) -> Vec<usize> {
    let mut factors = Vec::new();

    for &p in primes {
        if p * p > n {
            break;
        }
        while n % p == 0 {
            factors.push(p);
            n /= p;
        }
    }

    if n > 1 {
        factors.push(n);
    }

    factors
}
