// -*- coding:utf-8-unix -*-

use proconio::input;

// [L+1, R]の区間に存在する素べきの数 + 1
// (+1はLの分)
fn main() {
    input! {
        lr: [(usize, usize); 1],
    };
    let (l, r) = lr[0];

    // l-rが10^7なので、区間ふるいをやる

    // nums: l..=rの数を格納
    let mut nums: Vec<usize> = (l..=r).collect();
    // factor_counts: l..=rの素因数の個数を記録する(素べきなら1になる)
    let mut factor_counts = vec![0; r - l + 1];
    // println!("{:?}", nums)
    // sqrt(r)はたかだか10^7なので、rまでの合成数を篩おとすならsqrt(r)までで良い
    let sqrt_r = u64_floor_sqrt(r as u64) as usize;
    let mut is_prime = vec![true; sqrt_r + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    // ふるい開始
    for p in 2..=sqrt_r {
        if !is_prime[p] {
            continue;
        }

        // 素数iの倍数をふるいおとしていく
        for j in (p * 2..=sqrt_r).step_by(p) {
            is_prime[j] = false;
        }
        // l以上の素数の倍数を見つける
        let start = (l + p - 1) / p * p;
        for q in (start..=r).step_by(p) {
            factor_counts[q - l] += 1; // 素因数の個数をカウント
            while nums[q - l] % p == 0 {
                nums[q - l] /= p; // 割れるだけ割って素因数pを消す
            }
        }
    }
    let mut ans = 1;
    for i in 1..=r - l {
        if nums[i] != 1 {
            // 1でない場合、sqrt_rまでに出てこなかった素数かその倍数の場合がある
            // 例) [3, 26]の場合はsqrt_r=5で、例えば7 -> 7, 14 -> 7(素数2の篩の時点で割られる), 21 -> 7(素数3の篩の時点で割られる)
            factor_counts[i] += 1
        }
        if factor_counts[i] == 1 {
            ans += 1
        }
    }

    println!("{}", ans);
}

fn u64_floor_sqrt(n: u64) -> u64 {
    let tmp = (n as f64).sqrt() as u64;
    let tmp_m1 = tmp.saturating_sub(1);
    if tmp_m1 * (tmp_m1 + 2) < n {
        tmp
    } else {
        tmp_m1
    }
}
