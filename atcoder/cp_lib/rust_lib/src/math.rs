// 平方根の整数部分を求める関数
fn u64_floor_sqrt(n: u64) -> u64 {
    let tmp = (n as f64).sqrt() as u64;
    let tmp_m1 = tmp.saturating_sub(1);
    if tmp_m1 * (tmp_m1 + 2) < n {
        tmp
    } else {
        tmp_m1
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

    // 0, 1は素数ではない
    is_prime[0] = false;
    is_prime[1] = false;
    min_factor[0] = 0; // 未定義
    min_factor[1] = 0; // 未定義

    for p in 2..=n {
        if !is_prime[p] {
            continue;
        }
        min_factor[p] = p;
        // 2*pからふるいおとしていく
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

// 高速素因数分解 素因数とその指数をタプルで返す O(logn)
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

// 素因数分解 素因数とその指数をタプルで返す O(sqrt(n))
// 上のfast_prime_factorizeの方が速いが、処理がシンプルなのでこちらも残す
// NOTE: 事前にエラトステネスの篩でprimesを取得する
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

// 最大公約数を求めるユークリッドの互除法 O(log(min(a, b)))
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}
// 繰り返し二乗法
// b^e mod m を計算する
// O(log e) (e<=10^18程度なら間に合う)
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
