use cargo_snippet::snippet;

// 高速素因数分解 素因数とその指数をタプルで返す O(logn)
// NOTE: 事前にエラトステネスの篩でmin_factorを取得する
#[snippet("prime_factorize")]
#[snippet(include = "eratosthenes")]
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

// 素因数分解 素因数とその指数をタプルで返す O(sqrt(n))
// 上のfast_prime_factorizeの方が速いが、処理がシンプルなのでこちらも残す
// NOTE: 事前にエラトステネスの篩でprimesを取得する
#[snippet("prime_factorize")]
fn prime_factorize(mut n: usize, primes: &[usize]) -> Vec<(usize, usize)> {
    let mut factors = Vec::new();
    for &p in primes {
        if p * p > n {
            break;
        }
        let mut exp = 0;
        while n % p == 0 {
            n /= p;
            exp += 1;
        }
        if exp > 0 {
            factors.push((p, exp));
        }
    }

    if n > 1 {
        factors.push((n, 1));
    }

    factors
}
