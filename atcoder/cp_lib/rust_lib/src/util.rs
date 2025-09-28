use cargo_snippet::snippet;

#[snippet("inside")]
fn inside(i: isize, j: isize, h: usize, w: usize) -> bool {
    0 <= i && i < h as isize && 0 <= j && j < w as isize
}

/// 平方根の整数部分を求める関数
#[snippet("u64_floor_sqrt")]
fn u64_floor_sqrt(n: u64) -> u64 {
    let tmp = (n as f64).sqrt() as u64;
    let tmp_m1 = tmp.saturating_sub(1);
    if tmp_m1 * (tmp_m1 + 2) < n {
        tmp
    } else {
        tmp_m1
    }
}

/// 繰り返し二乗法
/// b^e mod m を計算する
/// O(log e) (e<=10^18程度なら間に合う)
#[snippet("mod_pow")]
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

/// 最大公約数を求めるユークリッドの互除法 O(log(min(a, b)))
#[snippet("gcd")]
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

/// 最小公倍数 O(log(min(a, b)))
#[snippet("lcm")]
#[snippet(include = "gcd")]
fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}
