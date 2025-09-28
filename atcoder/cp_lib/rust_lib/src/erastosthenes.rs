use cargo_snippet::snippet;

/// エラトステネスの篩 O(nloglogn)
/// 定数倍改善1: pのforは2..=sqrt(n)で良い
/// 定数倍改善2: qのforはp*p..=nで良い
/// NOTE: 以下は基本形からmin_factorも返すように修正。
/// 素数判定の過程で応用が効くので、このコードに手を加えて利用することもあるかも
#[snippet("eratosthenes")]
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

// 最大公約数を求めるユークリッドの互除法 O(log(min(a, b)))

#[snippet("gcd")]
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

#[snippet("lcm")]
#[snippet(include = "gcd")]
fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

